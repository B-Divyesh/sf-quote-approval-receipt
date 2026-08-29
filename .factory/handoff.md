# Repair 5 handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-repair-5`
- Repaired application commit: `4d68afe8ba19f84a7e83e90e8a006426addd6621`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Live revision: `sf-quote-approval-receipt--0000023`
- Image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:repair-5`
- Image digest: `sha256:8bb40fcb016866e7d89e21412c234b0cc80e2e1576c3967fc76d1084facae653`

## Fixed release blockers

The verifier's `POST /api/demo` followed by `GET /api/share/<token>` failure was a release-topology defect, not a front-end mismatch. Before this repair, the real Container App had `maxReplicas: 3`, no Azure Files volume, and no `DURABLE_DATA_DIR`, even though the committed deployment contract said otherwise. That split local SQLite copies and the in-memory rate limiter across replicas.

The live template is now the checked contract: one active serving replica (`minReplicas: 1`, `maxReplicas: 1`), Azure Files share `quote-approval-receipt-data` mounted at `/durable`, and `DURABLE_DATA_DIR=/durable`. The service's committed snapshots now restore across replacement, and its process-wide limiter is global for the one-replica deployment. `/health` now exposes `durable_snapshot`; live returns `true` alongside the exact build SHA so a release check can prove this topology.

Regression coverage added:

- `tests/durable-snapshot.mjs` asserts the mount, exact one-replica manifest, replacement persistence for 20 records, and durable health mode.
- `@claim:rate-limits` now asserts exactly fifteen accepted writes and a sixteenth `429` with `Retry-After: 1`.
- `npm run test:live-topology` interrogates the actual Container Apps template; it failed before deploy on `maxReplicas: 3` and passes on revision `0000023`.
- `npm run test:live-rate-limit` sends a concurrent sixteen-write admission burst, requires 15 `201` plus one `429` and `Retry-After: 1`, and removes all sample workspaces even on failure.

## Verification

Clean local install and release gates passed:

```bash
npm ci
npm test
npm run check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
```

`npm test` passed all 13 claim mappings, production TypeScript/Vite build, 4 Rust tests, only-`PORT` runtime start, durable replacement test, and 20 Playwright desktop/mobile checks. Production output remains 27.11 kB JS (8.96 kB gzip) and 15.55 kB CSS (4.20 kB gzip).

Live checks on the released build:

```bash
npm run test:live-topology
LIVE_URL=https://quote-approval-receipt.sociobot.in \
EXPECTED_BUILD_SHA=4d68afe8ba19f84a7e83e90e8a006426addd6621 \
npm run test:live-demo
LIVE_URL=https://quote-approval-receipt.sociobot.in npm run test:live-rate-limit
```

- Topology check: pass; one mounted durable replica.
- Build identity: `/health` returns `4d68afe8ba19f84a7e83e90e8a006426addd6621` and `durable_snapshot: true`.
- Demo core flow: pass; 20/20 separate 390 px Chromium contexts created (`201`), read (`200`), and deleted (`404`) isolated sample workspaces.
- Rate limit: pass; 15 concurrent writes returned `201`; the sixteenth admission returned `429` with `Retry-After: 1`; all samples were removed.
- Live mobile review: pass; landing, builder, Privacy, Terms, 404, demo cleanup, same-origin request policy, metadata, no horizontal overflow, and zero serious/critical Axe violations through Playwright AxeBuilder. Evidence is in `.factory/evidence/repair-5/`.
- `/opt/fleet/lib/verify-url.sh`: pass; 571 ms desktop load, title/lang/one H1/main/alt/button checks pass, no console errors. The standalone Axe CLI was attempted with its default browser and the installed Playwright Chromium, but its Selenium ChromeDriver is version-mismatched; the equivalent Playwright AxeBuilder audit above passed for every public route.

The ACR build `chv7` built the source tarball with `.git` excluded and `BUILD_SHA=4d68afe…`.

## Deployment

The Container App was patched with the work-order configuration in `.factory/containerapp-deploy.json` after the ACR build. Do not scale this service above one replica without moving both record state and rate-limit state to a shared transactional service. The checked deployment manifest and `npm run test:live-topology` are the regression guard against reintroducing the replica split.

## Known gaps

None for the product release. The optional standalone Axe CLI needs a ChromeDriver matching the preinstalled Chromium; release accessibility coverage uses the repository's Playwright Axe integration and passed live.
