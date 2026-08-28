# Repair handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-repair-3`
- Repaired release commit: `e6be9c7ca2c3bcb39ab39c31dc28aece724ea3cb`
- Deployment: <https://quote-approval-receipt.sociobot.in>
- Verdict: **PASS — repaired and deployed**

## What changed

The independent verifier found that the public Container App had no durable mount and allowed up to three replicas. Each replica used its own local SQLite database, so a successful demo create often reached a different database on its immediate share read.

The deployment now has exactly one replica (`minReplicas: 1`, `maxReplicas: 1`), mounts the registered `quote-approval-receipt-data` Azure Files storage as `/durable`, and sets `DURABLE_DATA_DIR=/durable`. The backend restores from that durable snapshot before serving and writes an atomic replacement snapshot after every committed create, decision, delete, and cleanup. Snapshot writes use a sibling file followed by rename, so a replacement process never restores a partially written database. This intentionally avoids live SQLite locking over Azure Files; SQLite remains the one local writer and Azure Files is the durable replacement boundary.

The durable-state claim now builds independently and creates 20 demos, replaces the process with a new local data directory, then reads all 20 through the replacement process.

## Verification evidence

- `npm ci`: PASS, 30 packages, 0 vulnerabilities.
- `npm test`: PASS — 4 Rust tests and 16 Playwright tests, including desktop, 390 px/200% text reflow, keyboard, reduced motion, response policies, privacy, accessibility, PDF Unicode, claim tests, and the 20-record durable replacement test.
- `npm run check`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm run build`: PASS; `dist/` produced. Production assets: JS 26.00 kB raw / 8.86 kB gzip and CSS 15.45 kB raw / 4.19 kB gzip.
- `npm audit --audit-level=high`: PASS; 0 vulnerabilities.
- Local `verify-url.sh`: PASS — 613 ms load, correct title/lang, one H1, main landmark, complete image alt text, and no console errors. Playwright’s integrated axe scans found no serious/critical issues across all public screens. The standalone Axe CLI could not run because its Selenium Chrome binary is unavailable in this worker; this does not affect the shipped Playwright Chromium axe coverage.
- ACR container build `chpy`: PASS. Final image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:e6be9c7ca2c3`.
- Live revision: `sf-quote-approval-receipt--0000014`, healthy and running one replica. Startup logs report `database=/data/quotes.sqlite3`, `durable_snapshot=/durable/quotes.sqlite3`, persisted privacy salt, and build SHA `e6be9c7ca2c3bcb39ab39c31dc28aece724ea3cb`.
- Live identity: `GET /health` returns that same repaired SHA.
- Live critical regression: 20 fresh Chromium contexts each opened `/demo`; every context observed `POST /api/demo` 201 followed by its own `GET /api/share/<token>` 200 and rendered the sample quote (`20/20`, no failures).
- Live privacy/response check: new demo 201 and share 200; share response carries `Cache-Control: no-store` and `X-Robots-Tag: noindex, nofollow, noarchive`.
- Live `verify-url.sh`: PASS — 687 ms load, title/lang/H1/main/alt checks all pass, no console errors.

## Deployment configuration

The active Container App template has `PORT=8080` and `DURABLE_DATA_DIR=/durable`, an `AzureFile` volume named `durable-data` backed by `quote-approval-receipt-data`, mounted at `/durable`, and scale fixed at one replica. Do not increase the replica count while using this SQLite snapshot topology.

## Known gaps and next steps

No release-blocking gaps remain. The product is intentionally not a PWA, library, CLI, account system, or AI feature, so offline-update, package-consumer, authentication, and live-model checks do not apply. Keep the one-replica deployment contract when making future infrastructure changes.
