use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Method, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Alphanumeric, Rng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::{
    collections::{HashMap, VecDeque},
    env,
    net::SocketAddr,
    path::{Path as FsPath, PathBuf},
    sync::Arc,
    time::{Duration as StdDuration, Instant},
};
use tokio::{fs, signal, sync::Mutex};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{info, warn};

const BUILD_SHA: &str = env!("BUILD_SHA", "dev");

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
    privacy_salt: Arc<Vec<u8>>,
    limiter: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
}

#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

type ApiResult<T> = Result<T, (StatusCode, Json<ApiError>)>;

fn fail(status: StatusCode, message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
    (
        status,
        Json(ApiError {
            error: message.into(),
        }),
    )
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LineItem {
    description: String,
    quantity: f64,
    rate: f64,
}

#[derive(Debug, Deserialize)]
struct CreateQuote {
    creator_name: String,
    business_name: String,
    quote_number: String,
    client_name: String,
    currency: String,
    summary: String,
    items: Vec<LineItem>,
    tax_percent: Option<f64>,
    consent_text: String,
    retention_days: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreatedQuote {
    id: String,
    public_token: String,
    owner_token: String,
    share_path: String,
    expires_at: String,
}

#[derive(Debug, FromRow, Serialize, Clone)]
struct QuoteRow {
    id: String,
    public_token: String,
    creator_name: String,
    business_name: String,
    quote_number: String,
    client_name: String,
    currency: String,
    summary: String,
    items_json: String,
    subtotal: f64,
    tax: f64,
    total: f64,
    consent_text: String,
    created_at: String,
    expires_at: String,
    demo_workspace: Option<String>,
}

#[derive(Debug, Serialize)]
struct PublicQuote {
    id: String,
    creator_name: String,
    business_name: String,
    quote_number: String,
    client_name: String,
    currency: String,
    summary: String,
    items: Vec<LineItem>,
    subtotal: f64,
    tax: f64,
    total: f64,
    consent_text: String,
    created_at: String,
    expires_at: String,
    decided: bool,
    demo: bool,
}

#[derive(Debug, Deserialize)]
struct DecisionInput {
    name: String,
    title: String,
    email: Option<String>,
    decision: String,
    note: Option<String>,
    consent: bool,
}

#[derive(Debug, FromRow, Serialize)]
struct ReceiptRow {
    id: String,
    quote_id: String,
    approver_name: String,
    approver_title: String,
    approver_email: Option<String>,
    decision: String,
    note: Option<String>,
    consent_text: String,
    decided_at: String,
    snapshot_hash: String,
}

#[derive(Debug, Serialize)]
struct ReceiptResponse {
    receipt: ReceiptRow,
    quote: PublicQuote,
    pdf_path: String,
}

#[derive(Debug, Deserialize)]
struct DemoDecision {
    workspace: String,
    name: String,
    title: String,
    email: Option<String>,
    decision: String,
    note: Option<String>,
    consent: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    if let Err(error) = run().await {
        warn!(%error, "server stopped");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let data_dir = env::var("DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("data"));
    fs::create_dir_all(&data_dir).await?;
    let db_path = data_dir.join("quotes.sqlite3");
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| format!("sqlite://{}?mode=rwc", db_path.display()));
    let salt_path = data_dir.join("privacy_salt");
    let (salt, generated) = load_or_create_salt(&salt_path).await?;
    info!(database = %db_path.display(), privacy_salt = if generated { "generated" } else { "persisted" }, build_sha = BUILD_SHA, "configuration ready");

    let db = SqlitePoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await?;
    migrate(&db).await?;
    cleanup_expired(&db).await?;
    let state = AppState {
        db,
        privacy_salt: Arc::new(salt),
        limiter: Arc::new(Mutex::new(HashMap::new())),
    };
    let cleanup_db = state.db.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(StdDuration::from_secs(3600));
        loop {
            interval.tick().await;
            if let Err(error) = cleanup_expired(&cleanup_db).await {
                warn!(%error, "expired-record cleanup failed");
            }
        }
    });

    let api = Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes/:id", get(get_owned_quote).delete(delete_quote))
        .route("/quotes/:id/export", get(export_quote))
        .route("/share/:token", get(get_shared_quote))
        .route("/share/:token/decision", post(record_decision))
        .route("/receipts/:id", get(get_receipt))
        .route("/receipts/:id/pdf", get(get_receipt_pdf))
        .route("/demo", post(create_demo))
        .route("/demo/:workspace/decision", post(record_demo_decision))
        .with_state(state.clone());

    let dist = env::var("STATIC_DIR").unwrap_or_else(|_| "dist".into());
    let fallback = ServeFile::new(format!("{dist}/index.html"));
    let app = Router::new()
        .route("/health", get(health))
        .nest("/api", api)
        .fallback_service(ServeDir::new(&dist).fallback(fallback))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit))
        .layer(middleware::from_fn(security_headers))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(%addr, "listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn load_or_create_salt(path: &FsPath) -> std::io::Result<(Vec<u8>, bool)> {
    match fs::read(path).await {
        Ok(bytes) if bytes.len() >= 32 => Ok((bytes, false)),
        _ => {
            let mut bytes = vec![0u8; 32];
            rand::thread_rng().fill_bytes(&mut bytes);
            fs::write(path, &bytes).await?;
            Ok((bytes, true))
        }
    }
}

async fn migrate(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA journal_mode=WAL").execute(db).await?;
    sqlx::query(r#"CREATE TABLE IF NOT EXISTS quotes (
      id TEXT PRIMARY KEY, public_token TEXT NOT NULL UNIQUE, owner_token TEXT NOT NULL UNIQUE,
      creator_name TEXT NOT NULL, business_name TEXT NOT NULL, quote_number TEXT NOT NULL,
      client_name TEXT NOT NULL, currency TEXT NOT NULL, summary TEXT NOT NULL, items_json TEXT NOT NULL,
      subtotal REAL NOT NULL, tax REAL NOT NULL, total REAL NOT NULL, consent_text TEXT NOT NULL,
      created_at TEXT NOT NULL, expires_at TEXT NOT NULL, demo_workspace TEXT, deleted_at TEXT
    )"#).execute(db).await?;
    sqlx::query(r#"CREATE TABLE IF NOT EXISTS decisions (
      id TEXT PRIMARY KEY, quote_id TEXT NOT NULL UNIQUE REFERENCES quotes(id), approver_name TEXT NOT NULL,
      approver_title TEXT NOT NULL, approver_email TEXT, decision TEXT NOT NULL, note TEXT,
      consent_text TEXT NOT NULL, decided_at TEXT NOT NULL, snapshot_hash TEXT NOT NULL,
      ip_hash TEXT NOT NULL, user_agent TEXT NOT NULL
    )"#).execute(db).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quotes_public ON quotes(public_token)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_decisions_quote ON decisions(quote_id)")
        .execute(db)
        .await?;
    Ok(())
}

async fn cleanup_expired(db: &SqlitePool) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "DELETE FROM decisions WHERE quote_id IN (SELECT id FROM quotes WHERE expires_at < ?)",
    )
    .bind(&now)
    .execute(db)
    .await?;
    sqlx::query("DELETE FROM quotes WHERE expires_at < ?")
        .bind(&now)
        .execute(db)
        .await?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok","build_sha":BUILD_SHA}))
}

fn clean(value: &str, name: &str, min: usize, max: usize) -> ApiResult<String> {
    let value = value.trim();
    if value.len() < min || value.len() > max {
        return Err(fail(
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("{name} must be {min}–{max} characters."),
        ));
    }
    Ok(value.to_string())
}

fn token(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

async fn insert_quote(
    state: &AppState,
    input: CreateQuote,
    demo_workspace: Option<String>,
) -> ApiResult<CreatedQuote> {
    if input.items.is_empty() || input.items.len() > 40 {
        return Err(fail(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Add between 1 and 40 quote items.",
        ));
    }
    for item in &input.items {
        clean(&item.description, "Item description", 1, 160)?;
        if !(item.quantity > 0.0
            && item.quantity <= 100000.0
            && item.rate >= 0.0
            && item.rate <= 10000000.0)
        {
            return Err(fail(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Each quantity and rate must be a valid positive amount.",
            ));
        }
    }
    let tax_percent = input.tax_percent.unwrap_or(0.0);
    if !(0.0..=100.0).contains(&tax_percent) {
        return Err(fail(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Tax must be from 0 to 100 percent.",
        ));
    }
    let subtotal = input.items.iter().map(|i| i.quantity * i.rate).sum::<f64>();
    let tax = subtotal * tax_percent / 100.0;
    let id = uuid::Uuid::new_v4().to_string();
    let public_token = token(32);
    let owner_token = token(40);
    let created = Utc::now();
    let days = if demo_workspace.is_some() {
        1
    } else {
        input.retention_days.unwrap_or(30).clamp(1, 365)
    };
    let expires = created + Duration::days(days);
    sqlx::query(r#"INSERT INTO quotes (id,public_token,owner_token,creator_name,business_name,quote_number,client_name,currency,summary,items_json,subtotal,tax,total,consent_text,created_at,expires_at,demo_workspace)
      VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"#)
      .bind(&id).bind(&public_token).bind(&owner_token)
      .bind(clean(&input.creator_name,"Your name",2,100)?).bind(clean(&input.business_name,"Business name",2,120)?)
      .bind(clean(&input.quote_number,"Quote number",1,60)?).bind(clean(&input.client_name,"Client name",2,120)?)
      .bind(clean(&input.currency,"Currency",3,3)?.to_uppercase()).bind(clean(&input.summary,"Scope summary",4,2000)?)
      .bind(serde_json::to_string(&input.items).map_err(|_| fail(StatusCode::BAD_REQUEST,"Quote items could not be saved."))?)
      .bind(subtotal).bind(tax).bind(subtotal + tax).bind(clean(&input.consent_text,"Consent text",12,500)?)
      .bind(created.to_rfc3339()).bind(expires.to_rfc3339()).bind(&demo_workspace)
      .execute(&state.db).await.map_err(internal)?;
    Ok(CreatedQuote {
        id,
        public_token: public_token.clone(),
        owner_token,
        share_path: format!("/approve/{public_token}"),
        expires_at: expires.to_rfc3339(),
    })
}

async fn create_quote(
    State(state): State<AppState>,
    Json(input): Json<CreateQuote>,
) -> ApiResult<(StatusCode, Json<CreatedQuote>)> {
    Ok((
        StatusCode::CREATED,
        Json(insert_quote(&state, input, None).await?),
    ))
}

async fn find_public(state: &AppState, token: &str) -> ApiResult<QuoteRow> {
    let quote = sqlx::query_as::<_, QuoteRow>("SELECT id,public_token,creator_name,business_name,quote_number,client_name,currency,summary,items_json,subtotal,tax,total,consent_text,created_at,expires_at,demo_workspace FROM quotes WHERE public_token=? AND deleted_at IS NULL")
      .bind(token).fetch_optional(&state.db).await.map_err(internal)?
      .ok_or_else(|| fail(StatusCode::NOT_FOUND,"This approval link was not found or has been deleted."))?;
    if DateTime::parse_from_rfc3339(&quote.expires_at)
        .map(|d| d.with_timezone(&Utc) < Utc::now())
        .unwrap_or(true)
    {
        return Err(fail(
            StatusCode::GONE,
            "This approval link has expired. Ask the sender for a new link.",
        ));
    }
    Ok(quote)
}

async fn public_quote(state: &AppState, quote: QuoteRow) -> ApiResult<PublicQuote> {
    let decided = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM decisions WHERE quote_id=?")
        .bind(&quote.id)
        .fetch_one(&state.db)
        .await
        .map_err(internal)?
        > 0;
    Ok(PublicQuote {
        id: quote.id,
        creator_name: quote.creator_name,
        business_name: quote.business_name,
        quote_number: quote.quote_number,
        client_name: quote.client_name,
        currency: quote.currency,
        summary: quote.summary,
        items: serde_json::from_str(&quote.items_json).unwrap_or_default(),
        subtotal: quote.subtotal,
        tax: quote.tax,
        total: quote.total,
        consent_text: quote.consent_text,
        created_at: quote.created_at,
        expires_at: quote.expires_at,
        decided,
        demo: quote.demo_workspace.is_some(),
    })
}

async fn get_shared_quote(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> ApiResult<Json<PublicQuote>> {
    let quote = find_public(&state, &token).await?;
    Ok(Json(public_quote(&state, quote).await?))
}

fn owner(headers: &HeaderMap) -> ApiResult<&str> {
    headers
        .get("x-owner-token")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            fail(
                StatusCode::UNAUTHORIZED,
                "This management link is missing its owner key.",
            )
        })
}

async fn owned_row(state: &AppState, id: &str, owner_token: &str) -> ApiResult<QuoteRow> {
    sqlx::query_as::<_, QuoteRow>("SELECT id,public_token,creator_name,business_name,quote_number,client_name,currency,summary,items_json,subtotal,tax,total,consent_text,created_at,expires_at,demo_workspace FROM quotes WHERE id=? AND owner_token=? AND deleted_at IS NULL")
      .bind(id).bind(owner_token).fetch_optional(&state.db).await.map_err(internal)?
      .ok_or_else(|| fail(StatusCode::NOT_FOUND,"This quote was not found. Check the management link."))
}

async fn get_owned_quote(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Json<PublicQuote>> {
    let row = owned_row(&state, &id, owner(&headers)?).await?;
    Ok(Json(public_quote(&state, row).await?))
}

async fn export_quote(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let row = owned_row(&state, &id, owner(&headers)?).await?;
    let quote = public_quote(&state, row).await?;
    let receipt = sqlx::query_as::<_,ReceiptRow>("SELECT id,quote_id,approver_name,approver_title,approver_email,decision,note,consent_text,decided_at,snapshot_hash FROM decisions WHERE quote_id=?").bind(&id).fetch_optional(&state.db).await.map_err(internal)?;
    let body = serde_json::to_vec_pretty(&serde_json::json!({"quote":quote,"receipt":receipt}))
        .map_err(internal)?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/json"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=quote-record.json",
            ),
        ],
        body,
    )
        .into_response())
}

async fn delete_quote(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<StatusCode> {
    owned_row(&state, &id, owner(&headers)?).await?;
    sqlx::query("DELETE FROM decisions WHERE quote_id=?")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(internal)?;
    sqlx::query("DELETE FROM quotes WHERE id=?")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn save_decision(
    state: &AppState,
    quote: QuoteRow,
    input: DecisionInput,
    headers: &HeaderMap,
) -> ApiResult<ReceiptResponse> {
    if !input.consent {
        return Err(fail(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Tick the consent box before recording your decision.",
        ));
    }
    if input.decision != "approved" && input.decision != "changes_requested" {
        return Err(fail(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Choose Approve quote or Request changes.",
        ));
    }
    if sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM decisions WHERE quote_id=?")
        .bind(&quote.id)
        .fetch_one(&state.db)
        .await
        .map_err(internal)?
        > 0
    {
        return Err(fail(
            StatusCode::CONFLICT,
            "A decision has already been recorded for this quote.",
        ));
    }
    let name = clean(&input.name, "Name", 2, 100)?;
    let title = clean(&input.title, "Title", 2, 100)?;
    let email = input
        .email
        .filter(|s| !s.trim().is_empty())
        .map(|s| clean(&s, "Email", 3, 160))
        .transpose()?;
    if email.as_ref().is_some_and(|e| !e.contains('@')) {
        return Err(fail(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Enter a complete email address or leave it blank.",
        ));
    }
    let note = input
        .note
        .filter(|s| !s.trim().is_empty())
        .map(|s| clean(&s, "Note", 1, 1000))
        .transpose()?;
    let decided_at = Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let snapshot = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        quote.id,
        quote.items_json,
        quote.total,
        name,
        title,
        input.decision,
        quote.consent_text,
        decided_at
    );
    let snapshot_hash = hex::encode(Sha256::digest(snapshot.as_bytes()));
    let ip = client_ip(headers);
    let mut hasher = Sha256::new();
    hasher.update(&*state.privacy_salt);
    hasher.update(ip.as_bytes());
    let ip_hash = hex::encode(hasher.finalize());
    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .chars()
        .take(200)
        .collect::<String>();
    sqlx::query("INSERT INTO decisions (id,quote_id,approver_name,approver_title,approver_email,decision,note,consent_text,decided_at,snapshot_hash,ip_hash,user_agent) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)")
      .bind(&id).bind(&quote.id).bind(&name).bind(&title).bind(&email).bind(&input.decision).bind(&note).bind(&quote.consent_text).bind(&decided_at).bind(&snapshot_hash).bind(ip_hash).bind(ua)
      .execute(&state.db).await.map_err(|e| if e.to_string().contains("UNIQUE") { fail(StatusCode::CONFLICT,"A decision has already been recorded for this quote.") } else { internal(e) })?;
    let receipt = ReceiptRow {
        id: id.clone(),
        quote_id: quote.id.clone(),
        approver_name: name,
        approver_title: title,
        approver_email: email,
        decision: input.decision,
        note,
        consent_text: quote.consent_text.clone(),
        decided_at,
        snapshot_hash,
    };
    let public = public_quote(state, quote).await?;
    Ok(ReceiptResponse {
        receipt,
        quote: public,
        pdf_path: format!("/api/receipts/{id}/pdf"),
    })
}

async fn record_decision(
    State(state): State<AppState>,
    Path(token): Path<String>,
    headers: HeaderMap,
    Json(input): Json<DecisionInput>,
) -> ApiResult<(StatusCode, Json<ReceiptResponse>)> {
    let quote = find_public(&state, &token).await?;
    Ok((
        StatusCode::CREATED,
        Json(save_decision(&state, quote, input, &headers).await?),
    ))
}

async fn receipt_response(state: &AppState, id: &str) -> ApiResult<ReceiptResponse> {
    let receipt=sqlx::query_as::<_,ReceiptRow>("SELECT id,quote_id,approver_name,approver_title,approver_email,decision,note,consent_text,decided_at,snapshot_hash FROM decisions WHERE id=?").bind(id).fetch_optional(&state.db).await.map_err(internal)?.ok_or_else(||fail(StatusCode::NOT_FOUND,"This receipt was not found."))?;
    let row=sqlx::query_as::<_,QuoteRow>("SELECT id,public_token,creator_name,business_name,quote_number,client_name,currency,summary,items_json,subtotal,tax,total,consent_text,created_at,expires_at,demo_workspace FROM quotes WHERE id=? AND deleted_at IS NULL").bind(&receipt.quote_id).fetch_optional(&state.db).await.map_err(internal)?.ok_or_else(||fail(StatusCode::NOT_FOUND,"The quote for this receipt was deleted."))?;
    Ok(ReceiptResponse {
        pdf_path: format!("/api/receipts/{id}/pdf"),
        quote: public_quote(state, row).await?,
        receipt,
    })
}

async fn get_receipt(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<ReceiptResponse>> {
    Ok(Json(receipt_response(&state, &id).await?))
}

fn pdf_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .chars()
        .filter(|c| c.is_ascii() && !c.is_control())
        .collect()
}
fn make_pdf(r: &ReceiptResponse) -> Vec<u8> {
    let lines = vec![
        "QUOTE APPROVAL RECEIPT".to_string(),
        format!(
            "Decision: {}",
            if r.receipt.decision == "approved" {
                "APPROVED"
            } else {
                "CHANGES REQUESTED"
            }
        ),
        format!("Quote: {}", r.quote.quote_number),
        format!("From: {}", r.quote.business_name),
        format!("For: {}", r.quote.client_name),
        format!(
            "Approver: {} — {}",
            r.receipt.approver_name, r.receipt.approver_title
        ),
        format!("Recorded: {}", r.receipt.decided_at),
        format!("Total: {} {:.2}", r.quote.currency, r.quote.total),
        format!("Snapshot hash: {}", r.receipt.snapshot_hash),
        format!("Consent: {}", r.receipt.consent_text),
    ];
    let mut stream = String::from("BT /F1 18 Tf 54 760 Td 22 TL ");
    for (i, l) in lines.iter().enumerate() {
        if i == 1 {
            stream.push_str("/F1 12 Tf ");
        }
        stream.push_str(&format!("({}) Tj T* ", pdf_escape(l)));
    }
    stream.push_str("ET");
    let objects=["<< /Type /Catalog /Pages 2 0 R >>".to_string(),"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>".to_string(),format!("<< /Length {} >>\nstream\n{}\nendstream",stream.len(),stream),"<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>".to_string()];
    let mut out = b"%PDF-1.4\n".to_vec();
    let mut offsets = vec![0];
    for (i, obj) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", i + 1, obj).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for o in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{:010} 00000 n \n", o).as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer << /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF",
            objects.len() + 1,
            xref
        )
        .as_bytes(),
    );
    out
}
async fn get_receipt_pdf(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Response> {
    let r = receipt_response(&state, &id).await?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=approval-receipt.pdf",
            ),
            (
                header::HeaderName::from_static("x-robots-tag"),
                "noindex, nofollow",
            ),
        ],
        make_pdf(&r),
    )
        .into_response())
}

fn demo_quote() -> CreateQuote {
    CreateQuote{creator_name:"Mara Bell".into(),business_name:"Northstar Studio".into(),quote_number:"NS-2048".into(),client_name:"Juniper Market".into(),currency:"USD".into(),summary:"Brand launch photography: one planning call, a half-day shoot, and 24 edited product images.".into(),items:vec![LineItem{description:"Half-day product shoot".into(),quantity:1.0,rate:1450.0},LineItem{description:"Edited product images".into(),quantity:24.0,rate:35.0}],tax_percent:Some(0.0),consent_text:"I confirm that I am authorised to make this decision for the client named above.".into(),retention_days:Some(1)}
}
#[derive(Serialize)]
struct DemoQuoteCreated {
    public_token: String,
    share_path: String,
}
#[derive(Serialize)]
struct DemoCreated {
    workspace: String,
    quote: DemoQuoteCreated,
}
async fn create_demo(State(state): State<AppState>) -> ApiResult<(StatusCode, Json<DemoCreated>)> {
    let workspace = token(24);
    let created = insert_quote(&state, demo_quote(), Some(workspace.clone())).await?;
    let quote = DemoQuoteCreated {
        public_token: created.public_token,
        share_path: created.share_path,
    };
    Ok((StatusCode::CREATED, Json(DemoCreated { workspace, quote })))
}
async fn record_demo_decision(
    State(state): State<AppState>,
    Path(workspace): Path<String>,
    headers: HeaderMap,
    Json(input): Json<DemoDecision>,
) -> ApiResult<(StatusCode, Json<ReceiptResponse>)> {
    if input.workspace != workspace {
        return Err(fail(
            StatusCode::FORBIDDEN,
            "This demo workspace does not match.",
        ));
    }
    let quote=sqlx::query_as::<_,QuoteRow>("SELECT id,public_token,creator_name,business_name,quote_number,client_name,currency,summary,items_json,subtotal,tax,total,consent_text,created_at,expires_at,demo_workspace FROM quotes WHERE demo_workspace=? AND deleted_at IS NULL").bind(&workspace).fetch_optional(&state.db).await.map_err(internal)?.ok_or_else(||fail(StatusCode::NOT_FOUND,"This demo expired. Reset the demo to start again."))?;
    let d = DecisionInput {
        name: input.name,
        title: input.title,
        email: input.email,
        decision: input.decision,
        note: input.note,
        consent: input.consent,
    };
    Ok((
        StatusCode::CREATED,
        Json(save_decision(&state, quote, d, &headers).await?),
    ))
}

fn internal<E: std::fmt::Display>(e: E) -> (StatusCode, Json<ApiError>) {
    warn!(error=%e,"request failed");
    fail(
        StatusCode::INTERNAL_SERVER_ERROR,
        "The record could not be saved. Try again.",
    )
}
fn client_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .unwrap_or("unknown")
        .trim()
        .chars()
        .take(64)
        .collect()
}
async fn rate_limit(State(state): State<AppState>, req: Request<Body>, next: Next) -> Response {
    if !req.uri().path().starts_with("/api/") {
        return next.run(req).await;
    }
    let key = client_ip(req.headers());
    let limit = if req.method() == Method::GET { 40 } else { 15 };
    let now = Instant::now();
    let mut map = state.limiter.lock().await;
    let q = map.entry(key).or_default();
    while q
        .front()
        .is_some_and(|t| now.duration_since(*t) > StdDuration::from_secs(1))
    {
        q.pop_front();
    }
    if q.len() >= limit {
        drop(map);
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, "1")],
            Json(ApiError {
                error: "Too many requests. Wait one second and try again.".into(),
            }),
        )
            .into_response();
    }
    q.push_back(now);
    drop(map);
    next.run(req).await
}
async fn security_headers(req: Request<Body>, next: Next) -> Response {
    let private =
        req.uri().path().starts_with("/approve/") || req.uri().path().starts_with("/receipt/");
    let mut res = next.run(req).await;
    let h = res.headers_mut();
    h.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    h.insert("referrer-policy", HeaderValue::from_static("no-referrer"));
    h.insert("x-frame-options", HeaderValue::from_static("DENY"));
    h.insert(
        "permissions-policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    h.insert("content-security-policy",HeaderValue::from_static("default-src 'self'; img-src 'self' data:; style-src 'self'; script-src 'self'; connect-src 'self' https://api.sociobot.in; base-uri 'self'; form-action 'self' https://api.sociobot.in; frame-ancestors 'none'"));
    if private {
        h.insert(
            "x-robots-tag",
            HeaderValue::from_static("noindex, nofollow, noarchive"),
        );
    }
    res
}
async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.expect("ctrl-c handler") };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {_=ctrl_c=>{},_=terminate=>{}}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pdf_is_valid_enough() {
        let r = ReceiptResponse {
            receipt: ReceiptRow {
                id: "r".into(),
                quote_id: "q".into(),
                approver_name: "Ava".into(),
                approver_title: "Director".into(),
                approver_email: None,
                decision: "approved".into(),
                note: None,
                consent_text: "I agree".into(),
                decided_at: "2026-01-01T00:00:00Z".into(),
                snapshot_hash: "abc".into(),
            },
            quote: PublicQuote {
                id: "q".into(),
                creator_name: "M".into(),
                business_name: "Studio".into(),
                quote_number: "Q1".into(),
                client_name: "Client".into(),
                currency: "USD".into(),
                summary: "Work".into(),
                items: vec![],
                subtotal: 10.0,
                tax: 0.0,
                total: 10.0,
                consent_text: "I agree".into(),
                created_at: "now".into(),
                expires_at: "later".into(),
                decided: true,
                demo: false,
            },
            pdf_path: "x".into(),
        };
        let p = make_pdf(&r);
        assert!(p.starts_with(b"%PDF-1.4"));
        assert!(String::from_utf8_lossy(&p).contains("APPROVED"));
    }
    #[test]
    fn pdf_escapes_parentheses() {
        assert_eq!(pdf_escape("a(b)\\c"), "a\\(b\\)\\\\c");
    }
}
