# Repair 6 handoff — PASS

- Work order: `quote-approval-receipt-repair-6`
- Verifier base/report: `669d8841b399c703c53606233cef68e6b12bcc0e` / `.factory/verification-8.md`
- Repaired application candidate: `327aa1551253196d9cf6e85db5b94acebaafd57a`
- Repair regression commit: `e41f68d6b395b64cba963e044fa82fd445a189f2` (the current commit records its exact deployment evidence)
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Active revision/image: `sf-quote-approval-receipt--0000027` / `sociobotregistry.azurecr.io/sf-quote-approval-receipt:327aa1551253`
- Release decision: **PASS**

## Release-blocker repair

The verifier’s failure was reproduced before repair at the deployment boundary: active revision `0000026` had `maxReplicas: 3`, no Azure Files volume/mount, no `DURABLE_DATA_DIR`, and public health reported `durable_snapshot: false`. That is the unsafe replica-local SQLite configuration responsible for the verifier’s `POST /api/demo` 201 followed by a separate-request `GET /api/share/...` 404.

The application image was deliberately not changed. The exact verified candidate image was redeployed in revision `0000027` with the checked-in state contract:

- exactly one replica (`minReplicas: 1`, `maxReplicas: 1`);
- Azure Files environment storage `quote-approval-receipt-data` mounted read/write at `/durable`;
- `DURABLE_DATA_DIR=/durable` and `PORT=8080`.

Public `/health` now returns the exact candidate SHA and `durable_snapshot: true`. The deployment and health receipts are [deployment-live.json](evidence/repair-6/deployment-live.json) and [health.json](evidence/repair-6/health.json).

## Regression coverage

- `tests/replica-split.mjs` is an adversarial reproduction of the reported defect: two processes with separate non-durable SQLite directories produce create `201` on one replica and share-read `404` on the other.
- `tests/durable-snapshot.mjs` remains the positive counterpart: after 20 committed demos, a replacement process with a different local directory restores all 20 from the mounted durable state.
- `scripts/live-topology-check.mjs` now checks both Azure’s one-replica/mount/env manifest and the active public `/health` durable signal. An optional `EXPECTED_BUILD_SHA` binds it to the requested candidate.

## Verification

Clean install: `npm ci` completed with 0 vulnerabilities.

Local quality gates all passed:

- `npm test`: claims manifest, production build (8.96 kB gzip JS; 4.20 kB gzip CSS), 4 Rust tests, runtime contract, the new replica-split regression, durable replacement test, and 20 Playwright tests all passed.
- `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high` passed.
- Playwright covers desktop and 390px/200% reflow, keyboard/history/focus, serious/critical Axe findings, reduced motion, same-origin privacy, private response policies, PDFs, rate limits, and the useful offline error path.

Post-deploy live checks all passed against the active exact candidate:

- `LIVE_URL=https://quote-approval-receipt.sociobot.in EXPECTED_BUILD_SHA=327aa1551253196d9cf6e85db5b94acebaafd57a npm run test:live-topology` — one replica, Azure Files mount, durable health signal.
- `LIVE_URL=… EXPECTED_BUILD_SHA=… npm run test:live-demo` — 20/20 isolated Chromium sessions: create 201, read 200, cleanup 404; [log](evidence/repair-6/live-demo.log).
- `LIVE_URL=… npm run test:live-rate-limit` — 15 writes then one 429 with `Retry-After: 1`, and all 15 cleanups returned 204; [log](evidence/repair-6/live-rate-limit.log).
- `LIVE_URL=… EVIDENCE_DIR=.factory/evidence/repair-6 npm run test:live-review` — mobile 390px routes, Axe, metadata, first-screen flow, privacy request origin, and demo cleanup passed; [log](evidence/repair-6/live-review.log), [report](evidence/repair-6/live-review.json).

`verify-url.sh` and Docker are not installed in this worker image. Their applicable checks are covered by the passing Playwright route/accessibility suite and runtime/Dockerfile contract test; no package/consumer check applies to this web-with-backend product, and no PWA/service-worker update check applies.

## Known gaps / next step

No release-blocking finding remains. Keep `test:live-topology`, `test:live-demo`, `test:live-rate-limit`, and `test:live-review` as the required post-deployment gate for every revision of this single-writer SQLite service.
