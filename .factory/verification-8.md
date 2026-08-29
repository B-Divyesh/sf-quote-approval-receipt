# Independent verification 8 — FAIL

- **Candidate commit:** `327aa1551253196d9cf6e85db5b94acebaafd57a`
- **Live URL:** <https://quote-approval-receipt.sociobot.in>
- **Verified:** 2026-08-29 UTC
- **Decision:** **FAIL — do not release.** The checked-out product is sound locally, but the live candidate is deployed with an unsafe, non-durable multi-replica SQLite topology. This breaks the required one-click demo and can lose or split real approval records.

## First-read result — pass

A cold 1440px visit returned 200 with title `Quote Approval Receipt — Record quote decisions`, h1 **“Record who approved your quote”**, and these first-screen answers in plain words:

- It records who approved a fixed quote and issues a receipt.
- It is for contractors whose clients approve by email or chat.
- The first action is **“Try it with sample data”**, with the immediate outcome “Loads a private sample quote. Nothing enters your records.”

The footer reports `v1.3 · 327aa15`; `/health` reports the full candidate SHA, so the live URL is serving the candidate.

## Required claims — pass locally

After `npm ci` from this checkout (0 reported npm vulnerabilities), every command listed in `.factory/claims.json` was run exactly, individually, through the supplied local demo/test entry point:

| Claim | Result |
| --- | --- |
| `demo-sandbox` | PASS |
| `quote-snapshot` | PASS |
| `pdf-receipt` | PASS |
| `record-control` | PASS |
| `first-party-only` | PASS |
| `retention-policy` | PASS |
| `studio-offer` | PASS |
| `private-links` | PASS |
| `single-decision` | PASS |
| `network-address-privacy` | PASS |
| `rate-limits` | PASS |
| `runtime-contract` | PASS |
| `durable-snapshot` | PASS |

`npm test` was then run unfiltered: manifest validation, production frontend build, 4 Rust tests, runtime contract, durable snapshot test, and all 20 Playwright tests passed. The latter included normal approval/PDF, invalid input recovery, concurrent decision conflict, export/delete, keyboard/history, 390px/200% reflow, reduced motion, private headers, and offline error behavior.

## Local quality gates — pass

- `npm run check` — pass
- `cargo fmt --check` — pass
- `cargo clippy --all-targets -- -D warnings` — pass
- `npm audit --audit-level=high` — pass (0 vulnerabilities)
- `npm run build` — pass; output JS is 27.11 kB / **8.96 kB gzip**, CSS is 15.55 kB / **4.20 kB gzip**
- Docker image build was not run because `docker` is not installed in this verifier container. The repository runtime contract test passed instead. The Dockerfile was read and has the required multi-stage/non-root/build-arg shape.

## Live product checks

### What passed

- Candidate identity: `GET /health` returned `{"build_sha":"327aa1551253196d9cf6e85db5b94acebaafd57a","durable_snapshot":false,"status":"ok"}`.
- Cold landing and demo requests used only `quote-approval-receipt.sociobot.in`; no console/page errors were observed.
- Desktop normal demo flow worked in a single session: blank submission showed “Complete each required field…”, approving after required fields generated a PDF (`200`, `application/pdf`, 3,200,139 bytes), reset returned a fresh sample, and exiting cleared `demo:workspace` and owner keys.
- At 390px the landing/demo had no horizontal overflow. Five routes (`/`, `/?demo=1`, `/new`, `/privacy`, `/terms`) had no Axe serious/critical violations and no console errors. Keyboard Tab starts at the skip link and showed a visible `rgb(242, 200, 75) solid 4px` focus ring. Reduced-motion media query was active and the shipped CSS contains a reduced-motion path.
- Response headers include CSP with `frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `X-Frame-Options: DENY`, and Permissions-Policy. Hashed JS/CSS cache for one year immutable. Demo creation response includes `X-Robots-Tag: noindex, nofollow, noarchive` and `Cache-Control: no-store`.
- The rate-limit admission check observed the documented allowance: **15** concurrent demo writes succeeded and the 16th returned **429** with `Retry-After: 1`. Its cleanup subsequently exposed the replica-split defect below.

`verify-url.sh` was not present in this checkout or PATH, so it could not be run; equivalent live title/lang/main/console, image/axe, keyboard, mobile and response-header checks above were performed directly.

## Release-blocking defects

### Critical — deployed topology discards/splits durable records and violates the backend contract

Fresh Azure Container Apps inspection of the active revision `sf-quote-approval-receipt--0000026` (100% traffic, image `sociobotregistry.azurecr.io/sf-quote-approval-receipt:327aa1551253`) found:

- `maxReplicas: 3` instead of the required `1` for a SQLite writer;
- only `PORT=8080` in the container environment;
- no `DURABLE_DATA_DIR=/durable`, no volume mount, and no Azure Files volume;
- live `/health` reports `durable_snapshot: false`.

This disagrees with the committed `.factory/deployment.md` and `.factory/containerapp-deploy.json`, which require a single replica, `/durable` Azure Files mount, and `DURABLE_DATA_DIR=/durable`.

Reproduction from fresh live browsers:

1. `LIVE_URL=https://quote-approval-receipt.sociobot.in npm run test:live-demo` failed on attempt 1: `POST /api/demo` returned 201, then its `GET /api/share/<token>` returned **404**.
2. `LIVE_URL=… EVIDENCE_DIR=<temp> npm run test:live-review` failed waiting for the sample scope text, because the newly created demo was not readable.
3. `LIVE_URL=… npm run test:live-rate-limit` completed the 15-success/one-429 admission assertion, then failed cleanup 2 with **404**. A direct create/read/delete showed the same state can be seen on one request and absent on another.
4. `npm run test:live-topology` failed: `durable SQLite release must never split state or the limiter`, actual max replicas `3`, expected `1`.

For this product, a record of quote approval must be defensible and retained. Replica-local SQLite makes records and demos intermittently vanish, defeats the configured durable snapshot boundary, and makes per-client rate limiting non-global. This is a production release blocker, not a cosmetic deployment variance.

## Required remediation and re-verification

Deploy the candidate with the repository’s stated contract: exactly one replica, Azure Files storage `quote-approval-receipt-data` mounted at `/durable`, and `DURABLE_DATA_DIR=/durable`. Confirm `/health` returns `durable_snapshot: true`, then rerun `test:live-topology`, `test:live-demo`, `test:live-rate-limit`, and `test:live-review` against the new active revision. Do not accept the release until all four pass.
