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
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{info, warn};

const BUILD_SHA: &str = env!("BUILD_SHA", "dev");

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
    db_path: PathBuf,
    durable_db: Option<PathBuf>,
    persist_lock: Arc<Mutex<()>>,
    privacy_salt: Arc<Vec<u8>>,
    limiter: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
    billing_base: String,
    http: reqwest::Client,
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
    license: Option<String>,
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
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
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
    let database_override = env::var("DATABASE_URL").ok();
    let durable_dir = env::var("DURABLE_DATA_DIR").ok().map(PathBuf::from);
    if let Some(directory) = &durable_dir {
        fs::create_dir_all(directory).await?;
    }
    let durable_db = database_override
        .is_none()
        .then(|| durable_dir.as_ref().map(|dir| dir.join("quotes.sqlite3")))
        .flatten();
    if let Some(source) = durable_db.as_ref().filter(|path| path.exists()) {
        copy_file_bytes(source, &db_path).await?;
    }
    let database_url =
        database_override.unwrap_or_else(|| format!("sqlite://{}?mode=rwc", db_path.display()));
    let salt_path = durable_dir
        .as_ref()
        .unwrap_or(&data_dir)
        .join("privacy_salt");
    let (salt, generated) = load_or_create_salt(&salt_path).await?;
    info!(database = %db_path.display(), durable_snapshot = durable_db.as_ref().map(|p| p.display().to_string()), privacy_salt = if generated { "generated" } else { "persisted" }, build_sha = BUILD_SHA, "configuration ready");

    let db = SqlitePoolOptions::new()
        // This service deliberately deploys as one replica with one SQLite
        // connection. The database lives on its durable mounted volume.
        .max_connections(1)
        .connect(&database_url)
        .await?;
    migrate(&db).await?;
    cleanup_expired(&db).await?;
    let state = AppState {
        db,
        db_path,
        durable_db,
        persist_lock: Arc::new(Mutex::new(())),
        privacy_salt: Arc::new(salt),
        limiter: Arc::new(Mutex::new(HashMap::new())),
        billing_base: env::var("BILLING_BASE_URL").unwrap_or_else(|_| {
            "https://api.sociobot.in/api/v1/products/quote-approval-receipt".into()
        }),
        http: reqwest::Client::builder()
            .timeout(StdDuration::from_secs(8))
            .build()?,
    };
    persist_database(&state).await?;
    let cleanup_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(StdDuration::from_secs(3600));
        loop {
            interval.tick().await;
            if let Err(error) = cleanup_expired(&cleanup_state.db).await {
                warn!(%error, "expired-record cleanup failed");
            } else if let Err(error) = persist_database(&cleanup_state).await {
                warn!(%error, "durable database snapshot failed");
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
        .route("/studio", get(studio_status))
        .with_state(state.clone());

    let dist = env::var("STATIC_DIR").unwrap_or_else(|_| "dist".into());
    let fallback = ServeFile::new(format!("{dist}/index.html"));
    let app = Router::new()
        .route("/health", get(health))
        .nest("/api", api)
        .fallback_service(ServeDir::new(&dist).fallback(fallback))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit))
        .layer(middleware::from_fn(security_headers))
        .layer(CompressionLayer::new())
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

async fn persist_database(state: &AppState) -> std::io::Result<()> {
    let Some(destination) = &state.durable_db else {
        return Ok(());
    };
    let _guard = state.persist_lock.lock().await;
    copy_file_bytes(&state.db_path, destination).await
}

async fn copy_file_bytes(source: &FsPath, destination: &FsPath) -> std::io::Result<()> {
    let bytes = fs::read(source).await?;
    fs::write(destination, bytes).await
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok","build_sha":BUILD_SHA}))
}

#[derive(Deserialize)]
struct LicenseVerdict {
    valid: bool,
}

async fn verify_studio_license(state: &AppState, license: &str) -> ApiResult<bool> {
    let response = state
        .http
        .get(format!("{}/verify", state.billing_base))
        .query(&[("license", license)])
        .send()
        .await
        .map_err(|_| {
            fail(
                StatusCode::SERVICE_UNAVAILABLE,
                "The Studio license service is unavailable. Use 30-day retention and try again later.",
            )
        })?;
    if !response.status().is_success() {
        return Ok(false);
    }
    response
        .json::<LicenseVerdict>()
        .await
        .map(|result| result.valid)
        .map_err(|_| {
            fail(
                StatusCode::BAD_GATEWAY,
                "The Studio license response could not be read. Use 30-day retention and try again later.",
            )
        })
}

#[derive(Serialize)]
struct StudioStatus {
    available: bool,
    checkout_url: Option<String>,
}

async fn studio_status(State(state): State<AppState>) -> Json<StudioStatus> {
    let product = state.http.get(&state.billing_base).send().await;
    let available = match product {
        Ok(response) if response.status().is_success() => response
            .json::<serde_json::Value>()
            .await
            .ok()
            .is_some_and(|value| {
                value.get("price_minor").and_then(|v| v.as_i64()) == Some(2900)
                    && value.get("currency").and_then(|v| v.as_str()) == Some("USD")
            }),
        _ => false,
    };
    Json(StudioStatus {
        available,
        checkout_url: available.then(|| format!("{}/checkout", state.billing_base)),
    })
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
    let currency = clean(&input.currency, "Currency", 3, 3)?.to_uppercase();
    const CURRENCIES: [&str; 6] = ["USD", "EUR", "GBP", "CAD", "AUD", "INR"];
    if !CURRENCIES.contains(&currency.as_str()) {
        return Err(fail(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Choose a supported currency: USD, EUR, GBP, CAD, AUD, or INR.",
        ));
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
        match input.retention_days.unwrap_or(30) {
            30 => 30,
            365 => {
                let license = input.license.as_deref().unwrap_or("").trim();
                if license.is_empty() || !verify_studio_license(state, license).await? {
                    return Err(fail(
                        StatusCode::FORBIDDEN,
                        "A valid Studio license is required for 365-day retention.",
                    ));
                }
                365
            }
            _ => {
                return Err(fail(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Retention must be 30 days, or 365 days with Studio.",
                ))
            }
        }
    };
    let expires = created + Duration::days(days);
    sqlx::query(r#"INSERT INTO quotes (id,public_token,owner_token,creator_name,business_name,quote_number,client_name,currency,summary,items_json,subtotal,tax,total,consent_text,created_at,expires_at,demo_workspace)
      VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"#)
      .bind(&id).bind(&public_token).bind(&owner_token)
      .bind(clean(&input.creator_name,"Your name",2,100)?).bind(clean(&input.business_name,"Business name",2,120)?)
      .bind(clean(&input.quote_number,"Quote number",1,60)?).bind(clean(&input.client_name,"Client name",2,120)?)
      .bind(currency).bind(clean(&input.summary,"Scope summary",4,2000)?)
      .bind(serde_json::to_string(&input.items).map_err(|_| fail(StatusCode::BAD_REQUEST,"Quote items could not be saved."))?)
      .bind(subtotal).bind(tax).bind(subtotal + tax).bind(clean(&input.consent_text,"Consent text",12,500)?)
      .bind(created.to_rfc3339()).bind(expires.to_rfc3339()).bind(&demo_workspace)
      .execute(&state.db).await.map_err(internal)?;
    persist_database(state).await.map_err(internal)?;
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
    persist_database(&state).await.map_err(internal)?;
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
    let ip_hash = hash_network_address(&state.privacy_salt, &ip);
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
    persist_database(state).await.map_err(internal)?;
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

fn make_pdf(r: &ReceiptResponse) -> Result<Vec<u8>, genpdf::error::Error> {
    use genpdf::{elements, style, Element};
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
    let regular =
        genpdf::fonts::FontData::load("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", None)?;
    let bold = genpdf::fonts::FontData::load(
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        None,
    )?;
    let family = genpdf::fonts::FontFamily {
        regular: regular.clone(),
        bold,
        italic: regular.clone(),
        bold_italic: regular,
    };
    let mut document = genpdf::Document::new(family);
    document.set_title("Quote approval receipt");
    document.set_minimal_conformance();
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(18);
    document.set_page_decorator(decorator);
    for (index, line) in lines.into_iter().enumerate() {
        let paragraph = elements::Paragraph::new(line).padded(genpdf::Margins::trbl(0, 0, 3, 0));
        if index == 0 {
            document.push(paragraph.styled(style::Style::new().bold().with_font_size(18)));
        } else {
            document.push(paragraph.styled(style::Style::new().with_font_size(11)));
        }
    }
    let mut output = Vec::new();
    document.render(&mut output)?;
    Ok(output)
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
        make_pdf(&r).map_err(internal)?,
    )
        .into_response())
}

fn demo_quote() -> CreateQuote {
    CreateQuote{creator_name:"Mara Bell".into(),business_name:"Northstar Studio".into(),quote_number:"NS-2048".into(),client_name:"Juniper Market".into(),currency:"USD".into(),summary:"Brand launch photography: one planning call, a half-day shoot, and 24 edited product images.".into(),items:vec![LineItem{description:"Half-day product shoot".into(),quantity:1.0,rate:1450.0},LineItem{description:"Edited product images".into(),quantity:24.0,rate:35.0}],tax_percent:Some(0.0),consent_text:"I confirm that I am authorised to make this decision for the client named above.".into(),retention_days:Some(1),license:None}
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
fn hash_network_address(salt: &[u8], address: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(address.as_bytes());
    hex::encode(hasher.finalize())
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
    let path = req.uri().path().to_owned();
    let private = path.starts_with("/api/")
        || path.starts_with("/approve/")
        || path.starts_with("/receipt/")
        || path.starts_with("/manage/");
    let known_spa = matches!(
        path.as_str(),
        "/" | "/new" | "/demo" | "/privacy" | "/terms"
    ) || path.starts_with("/approve/")
        || path.starts_with("/receipt/")
        || path.starts_with("/manage/");
    let mut res = next.run(req).await;
    let should_be_404 = !known_spa
        && !path.starts_with("/api/")
        && res.status() == StatusCode::OK
        && res
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.starts_with("text/html"));
    if should_be_404 {
        *res.status_mut() = StatusCode::NOT_FOUND;
    }
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
        h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    } else if path.starts_with("/assets/") {
        h.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
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
    fn pdf_preserves_complete_international_receipt_text() {
        let r = ReceiptResponse {
            receipt: ReceiptRow {
                id: "r".into(),
                quote_id: "q".into(),
                approver_name: "José Núñez".into(),
                approver_title: "Direção".into(),
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
                quote_number: "Q-Á1".into(),
                client_name: "Café São Bento".into(),
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
        let p = make_pdf(&r).expect("PDF should render");
        assert!(p.starts_with(b"%PDF-1."));
        assert!(p.len() > 100_000, "the Unicode font should be embedded");
    }

    #[test]
    fn network_address_hash_is_salted_and_one_way() {
        let address = "198.51.100.212";
        let first = hash_network_address(b"first private salt", address);
        let second = hash_network_address(b"second private salt", address);
        assert_ne!(first, second);
        assert!(!first.contains(address));
        assert_eq!(first.len(), 64);
    }

    #[tokio::test]
    async fn studio_retention_requires_live_verification() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("mock listener");
        let address = listener.local_addr().expect("mock address");
        let mock = tokio::spawn(async move {
            axum::serve(
                listener,
                Router::new().route(
                    "/verify",
                    get(|| async { Json(serde_json::json!({"valid": true})) }),
                ),
            )
            .await
        });
        let directory = tempfile::tempdir().expect("temporary database directory");
        let database_url = format!(
            "sqlite://{}?mode=rwc",
            directory.path().join("quotes.sqlite3").display()
        );
        let db = SqlitePoolOptions::new()
            .connect(&database_url)
            .await
            .expect("database connection");
        migrate(&db).await.expect("database migration");
        let state = AppState {
            db,
            db_path: directory.path().join("quotes.sqlite3"),
            durable_db: None,
            persist_lock: Arc::new(Mutex::new(())),
            privacy_salt: Arc::new(vec![1; 32]),
            limiter: Arc::new(Mutex::new(HashMap::new())),
            billing_base: format!("http://{address}"),
            http: reqwest::Client::new(),
        };

        let mut licensed = demo_quote();
        licensed.retention_days = Some(365);
        licensed.license = Some("valid-test-license".into());
        let created = insert_quote(&state, licensed, None)
            .await
            .expect("verified Studio record");
        let remaining = DateTime::parse_from_rfc3339(&created.expires_at)
            .expect("expiry")
            .with_timezone(&Utc)
            - Utc::now();
        assert!(remaining.num_days() >= 364);

        let mut forged = demo_quote();
        forged.retention_days = Some(365);
        let error = insert_quote(&state, forged, None)
            .await
            .expect_err("missing license must fail");
        assert_eq!(error.0, StatusCode::FORBIDDEN);
        mock.abort();
    }

    #[tokio::test]
    async fn durable_snapshot_restores_a_committed_record() {
        let directory = tempfile::tempdir().expect("temporary storage");
        let local = directory.path().join("local.sqlite3");
        let durable = directory.path().join("durable.sqlite3");
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&format!("sqlite://{}?mode=rwc", local.display()))
            .await
            .expect("database connection");
        migrate(&db).await.expect("database migration");
        let state = AppState {
            db,
            db_path: local.clone(),
            durable_db: Some(durable.clone()),
            persist_lock: Arc::new(Mutex::new(())),
            privacy_salt: Arc::new(vec![2; 32]),
            limiter: Arc::new(Mutex::new(HashMap::new())),
            billing_base: "http://127.0.0.1:1".into(),
            http: reqwest::Client::new(),
        };
        let created = insert_quote(&state, demo_quote(), Some("durable-demo".into()))
            .await
            .expect("record creation");
        state.db.close().await;
        copy_file_bytes(&durable, &directory.path().join("restored.sqlite3"))
            .await
            .expect("restore snapshot");
        let restored = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "sqlite://{}?mode=rw",
                directory.path().join("restored.sqlite3").display()
            ))
            .await
            .expect("restored connection");
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM quotes WHERE id=?")
            .bind(created.id)
            .fetch_one(&restored)
            .await
            .expect("restored query");
        assert_eq!(count, 1);
    }
}
