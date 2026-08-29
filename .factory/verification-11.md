# Independent verification 11 — FAIL

- **Candidate commit:** `aea90d11a961be20b07825a5a917fa092268324d`
- **Live URL:** <https://quote-approval-receipt.sociobot.in>
- **Verified:** 2026-08-29 UTC
- **Decision:** **FAIL — do not release.** The candidate is deployed (its `/health` reports the exact SHA), and the local product passes its full quality suite, but the live backend is an unsafe non-durable multi-replica deployment and freshly loses a newly-created demo approval link.

## First read — PASS

A cold desktop Chromium visit returned HTTP 200 with the title `Quote Approval Receipt — Record quote decisions`. The first screen answers the three required questions in plain words:

- **What it does:** “Record who approved your quote”.
- **For whom:** contractors whose clients approve by email or chat and may later change expectations.
- **First click:** **Try it with sample data**, with the adjacent explanation “Loads a private sample quote. Nothing enters your records.”

The one-click action is in the first viewport. The live footer reports `v1.3 · aea90d1` and `/health` reports the complete candidate SHA.

## Required claim tests — PASS locally (13/13)

From the clean checkout I ran every literal command in `.factory/claims.json`, in manifest order, through the shipped local demo/test entry point. The loop completed all 13 without stopping; the final Playwright run status was `passed`.

| Claim | Result |
| --- | --- |
| `demo-sandbox` | PASS locally; **FAILS live intermittently** (critical finding below) |
| `quote-snapshot` | PASS locally |
| `pdf-receipt` | PASS locally |
| `record-control` | PASS locally |
| `first-party-only` | PASS locally |
| `retention-policy` | PASS locally |
| `studio-offer` | PASS locally |
| `private-links` | PASS locally |
| `single-decision` | PASS locally |
| `network-address-privacy` | PASS locally |
| `rate-limits` | PASS locally and live |
| `runtime-contract` | PASS locally |
| `durable-snapshot` | PASS locally; **FAILS live** (`durable_snapshot:false`) |

## Local quality gates — PASS

- `npm ci` — PASS; 30 packages installed and npm reported 0 vulnerabilities.
- `npm test` — PASS: claims manifest, production build, 4 Rust unit tests, runtime contract, replica-split regression, durable replacement test, and **22 Playwright tests**.
- `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm run build` — PASS.
- Production client build: JavaScript **27.11 kB** (8.96 kB gzip); CSS **15.55 kB** (4.20 kB gzip). `dist/` is produced.
- The full local browser suite covers keyboard/skip-link focus, visible focus, reduced motion, 390px/200% text reflow, form errors/recovery, offline recovery, normal and invalid input, PDF receipt, deletion/export, decision concurrency, and serious/critical Axe checks.
- Docker’s CLI is not installed in this verifier container, so the exact image build could not be independently run. The repository runtime-contract and release-binary checks passed; this does not mitigate the live failure.

## Live verification

- `GET /health` returned HTTP 200 and `{"build_sha":"aea90d11a961be20b07825a5a917fa092268324d","durable_snapshot":false,"status":"ok"}`.
- `LIVE_URL=https://quote-approval-receipt.sociobot.in EXPECTED_BUILD_SHA=... npm run test:live-topology` — **FAIL**: checked-in deployment inspection reports `maxReplicas` **3**, where the one-writer deployment contract requires 1.
- `npm run test:live-workflow` — **FAIL** before creating data because public health reports `durable_snapshot:false` instead of the required true.
- `npm run test:live-demo` initially completed 20/20 sessions, but a fresh repeated run immediately **FAILED**: `POST /api/demo` returned 201 and the immediate `GET /api/share/<token>` returned 404 on attempt 1.
- Independent fresh 390px Chromium evidence reproduced the same failure: `POST /api/demo` 201, then `GET /api/share/b44mYKiG3OpiZdA459zLnBNOcajBCnSJ` 404. The UI showed “THE DEMO COULD NOT START” and logged that failed resource request.
- `npm run test:live-rate-limit` — PASS: 40 concurrent reads were admitted, the 41st returned 429 with `Retry-After: 1`; 15 invalid writes were admitted, the 16th returned 429 with `Retry-After: 1`.
- A live desktop and 390px route audit of `/`, `/new`, `/privacy`, `/terms`, and a 404 found one `h1`, one `main`, no horizontal overflow, no unexpected console/page errors, and zero Axe serious/critical findings. Keyboard Tab gives the skip link a 4px solid focus outline; Enter moves focus to `main`; this also worked with reduced-motion emulation.
- The cold landing and failed demo request logs used only `https://quote-approval-receipt.sociobot.in`; no analytics, advertising, or third-party runtime request occurred. The failure prevented a reliable successful live end-to-end privacy-flow confirmation.
- Public responses send `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `X-Frame-Options: DENY`, restrictive permissions policy, and CSP including response-header `frame-ancestors 'none'`. Hashed JS/CSS assets have `Cache-Control: public, max-age=31536000, immutable`.

## Release-blocking defect

### Critical — deployed SQLite state is split/non-durable, losing newly-created approval links

The product’s smallest useful job is to create a private approval link that an external client can immediately open. The live candidate cannot reliably do that.

Evidence binds the problem to this candidate rather than a stale deployment: public `/health` reports build `aea90d11a961be20b07825a5a917fa092268324d`. The same health response reports `durable_snapshot:false`. The repository’s own `test:live-topology` inspected the active Container App and failed at `maxReplicas: 3` (required 1). A fresh browser then observed demo creation return 201 followed immediately by the matching share read returning 404; the checked-in 20-session live demo checker subsequently failed on its first session with the same 201-to-404 loss.

This is consistent with process-local SQLite state being served by different replicas, and directly breaks the defensible-record contract. Rate limiting and occasional successful demo sessions do not make this releaseable.

## Required remediation

1. Deploy the checked-in single-writer configuration: exactly one active/ready replica, Azure Files `quote-approval-receipt-data` mounted at `/durable`, and `DURABLE_DATA_DIR=/durable`.
2. Confirm `/health` reports the candidate SHA and `durable_snapshot:true`.
3. Repeat and require passes for `test:live-topology`, `test:live-workflow`, a 20-session `test:live-demo`, and `test:live-rate-limit` from fresh clients.

This is a web-with-backend product, not a library/CLI or PWA, and it has no sign-in flow; consumer-package, service-worker, and Entra checks do not apply.
