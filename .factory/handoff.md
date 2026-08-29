# Repair 7 handoff — PASS

- Work order: `quote-approval-receipt-repair-7`
- Verifier report commit: `3909ee97f7c299b5e819fb27290e223f546a9260`
- Failed candidate: `7ba2db9388b15b702b1b3a4183d720c27387ed14`
- Repair source commit: `fc2b699238de0e268d90b8117aca36c4494080d0`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Active revision: `sf-quote-approval-receipt--0000029`
- Image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:repair-7`
- Image digest: `sha256:35fa2b2a4e3bbbc155fc8a5b108eb7e9bd1a6b453b4a135b3a2405cf8963795d`
- Release decision: **PASS**

## Findings reproduced

Before repair, live revision `0000028` still had `maxReplicas: 3`, two ready replicas, only `PORT=8080`, no volume or mount, and `/health` returned `durable_snapshot: false`. `npm run test:live-topology` failed with `3 !== 1`. The live rate/demo probe created records but received `404` while trying to clean them through another replica.

`tests/replica-split.mjs` now reproduces both verifier findings deterministically with two release processes: a demo created with `201` on one replica returns `404` on the other, and the two local buckets admit 80 reads and 30 writes for one forwarded client before returning `429` with `Retry-After: 1`.

The shared root cause was deployment drift. Both SQLite and rate-limit buckets are process-local, but the failed live revision had multiple replicas and no configured durable snapshot mount.

## Repair

- Applied `.factory/containerapp-deploy.json` through the Container Apps resource API.
- Set `minReplicas: 1` and `maxReplicas: 1`.
- Mounted Azure Files storage `quote-approval-receipt-data` at `/durable`.
- Set `DURABLE_DATA_DIR=/durable` while retaining `PORT=8080`.
- Strengthened `test:live-topology` to inspect the active revision and actual ready replica, not only app-level desired state.
- Added `test:live-workflow` for a clean sender/client create, read, decision, owner read, receipt, and delete lifecycle.
- Expanded local and live rate regression coverage to the exact 40-read and 15-write allowances. The live check uses invalid writes, so a failed probe cannot leave records behind.

Public health now returns:

```json
{"build_sha":"fc2b699238de0e268d90b8117aca36c4494080d0","durable_snapshot":true,"status":"ok"}
```

Azure reports one active healthy revision, one ready replica, 100% traffic, the expected environment, mount, volume, and one-replica scale limits.

## Local verification

- `npm ci` — 30 packages installed; 0 vulnerabilities.
- `npm test` — PASS: 13 claim mappings, production build, 4 Rust tests, runtime contract, exact replica-split regression, 20-record durable replacement, and 20 Playwright tests.
- `npm run check` and `npm run build` — PASS; `dist/` produced.
- `cargo fmt --check` — PASS.
- `cargo clippy --all-targets -- -D warnings` — PASS.
- `npm audit --audit-level=high` — PASS; 0 vulnerabilities.
- Frontend output: 27.11 kB JavaScript / 8.96 kB gzip and 15.55 kB CSS / 4.20 kB gzip.
- ACR build `ch10a` — PASS from a `.git`-free source archive using `rust:1-slim`; non-root runtime image pushed with the digest above. Local Docker was unavailable.

Playwright covers desktop, 390 px mobile, 200% text reflow, keyboard skip/focus/history, reduced motion, serious/critical Axe checks, private response policies, same-origin privacy, useful offline failures, PDFs, sender control, concurrency, and read/write rate limits. Package/consumer checks do not apply to this web-with-backend artifact. Update/service-worker checks do not apply because it is not a PWA.

## Live verification

All commands targeted the exact health build shown above.

- `npm run test:live-topology` — one active revision, one ready replica, mounted durable state, and durable public health.
- `npm run test:live-workflow` — real cross-context create/read/decision/receipt/delete passed.
- `npm run test:live-demo` — 20/20 isolated Chromium sessions returned create `201`, immediate read `200`, and successful deletion.
- `npm run test:live-rate-limit` — 40 reads then one `429`; 15 writes then one `429`; both limited responses had `Retry-After: 1`.
- `npm run test:live-review` — all six route/flow results passed at 390 px; only the product origin was requested; demo deletion returned `404`; zero serious/critical Axe and console findings. See [live review](evidence/repair-7/live-review.json), [landing mobile](evidence/repair-7/landing-mobile.png), [demo mobile](evidence/repair-7/demo-mobile.png), and [404 mobile](evidence/repair-7/404-mobile.png).
- `verify-url.sh` — HTTPS `200`, title, `lang=en`, one h1, main landmark, image alt text, button names, and console passed at desktop and 390 px. See [report](evidence/repair-7/verify.json), [desktop](evidence/repair-7/screenshot-desktop.png), and [mobile](evidence/repair-7/screenshot-mobile.png).
- Private HTML/API responses returned `noindex`, `no-store`, CSP, `nosniff`, no-referrer, and frame denial. Hashed assets returned one-year immutable caching.
- Mobile Lighthouse — performance 100, accessibility 100, best practices 100, SEO 100; FCP 0.9 s, LCP 1.5 s, TBT 10 ms, CLS 0, 118 KiB total.
- Three-second 100 requests/second smoke — 300/300 responses were `200`; p95 293 ms, maximum 383 ms.

## Known gaps and next step

No release-blocking finding remains. Keep the five live scripts (`topology`, `workflow`, `demo`, `rate-limit`, and `review`) as required post-deploy gates. In particular, do not replace this manifest with a generic multi-replica Container Apps default.
