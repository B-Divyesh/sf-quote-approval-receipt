# Repair 8 handoff — PASS

- Work order: `quote-approval-receipt-repair-8`
- Verifier report commit: `6de4497ea52deaec0ab8d0dab883e23599ecf3ab`
- Failed candidate: `1a5283f8246b96f1cb0105f6018f476fa124cdfc`
- Repair source commit: `3bd69066b6e07d309e1942e011b97eb0bc17d617`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Active revision: `sf-quote-approval-receipt--0000031`
- Image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:repair-8`
- Image digest: `sha256:c7a1a0ffd66027dec0e6da9736fe7c1763448d86e7e13e2bcbd68415b3e04185`
- Release decision: **PASS**

## Findings reproduced and repaired

### Critical — non-durable multi-replica live deployment

The verifier's failure was reproduced before deployment: revision `0000030` ran image `1a5283f8246b`, allowed `maxReplicas: 3`, supplied only `PORT`, had no volume or mount, and reported `{"durable_snapshot":false}` from `/health`. The checked-in live topology test failed because the deployment was not a single writer. This state can split the SQLite database and process-local rate-limit buckets, reproducing the reported create `201` followed by read `404` loss.

The `repair-8` image was built from the root Dockerfile with `BUILD_SHA=3bd69066b6e07d309e1942e011b97eb0bc17d617` and deployed through the Container Apps resource API using `.factory/containerapp-deploy.json`. The active revision now has exactly one ready replica and 100% traffic; `minReplicas: 1`, `maxReplicas: 1`; Azure Files storage `quote-approval-receipt-data` mounted read/write at `/durable`; and `DURABLE_DATA_DIR=/durable`. The former revision initially remained active at 0% traffic, which `test:live-topology` correctly rejected; it was explicitly deactivated. Final public health is:

```json
{"build_sha":"3bd69066b6e07d309e1942e011b97eb0bc17d617","durable_snapshot":true,"status":"ok"}
```

### High — flaky 390px demo target audit

The mobile test was able to enumerate demo-bar controls while `renderDemo()` was replacing its loading shell with the sample quote. Those detached elements report `0 × 0`, so the test could fail even though the usable sample screen uses the 44px controls defined by the product.

`tests/product.spec.ts` now waits for the rendered “Review and decide on this quote” heading, the “Half-day product shoot” sample, and removal of `.loading` before assessing reflow or control bounds. It records numeric width/height rather than a transient `DOMRect` and adds `@mobile @regression:mobile-demo target audit waits for the rendered sample quote`. This preserves the existing demo behavior while auditing the screen visitors can use.

## Local verification

- `npm ci` — PASS: 30 packages installed; `npm audit --audit-level=high` reports 0 vulnerabilities.
- Every literal command in `.factory/claims.json` — PASS: all 11 `npm test -- --grep @claim:…` commands plus `npm run test:runtime-contract` and `npm run test:durable-snapshot`.
- `npm test` — PASS: 13 claim mappings, production build, four Rust tests, runtime contract, replica-split regression, durable replacement of 20 committed demos, and **22 Playwright tests**.
- `npm run check`, `npm run build`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings` — PASS.
- Browser coverage passed on desktop and 390px mobile: 200% text reflow, 44px controls, keyboard skip link/history/focus, reduced motion, offline recovery, serious/critical Axe checks, same-origin privacy, PDF/download, deletion, decision concurrency, response policies, and rate-limit boundaries.
- Production bundle: 27.11 kB JavaScript (8.96 kB gzip) and 15.55 kB CSS (4.20 kB gzip).
- ACR run `ch114` — PASS from the `.git`-excluded Docker context, using `node:22-bookworm-slim`, `rust:1-slim`, and non-root runtime image. The image digest is listed above.

## Live verification

- `npm run test:live-topology` with `EXPECTED_BUILD_SHA=3bd69066b6e07d309e1942e011b97eb0bc17d617` — PASS: one active revision, one ready replica, durable Azure Files topology, and durable public health.
- `npm run test:live-workflow` — PASS: cross-context create, read, named decision, sender read, receipt retrieval, and deletion.
- `npm run test:live-demo` — PASS: **20/20** isolated 390px Chromium demo sessions each created (`201`), read (`200`), and removed (`404` after exit) their sample record.
- `npm run test:live-rate-limit` — PASS: 40 reads then `429`, and 15 invalid writes then `429`; both limits returned `Retry-After: 1`.
- `npm run test:live-review` — PASS: mobile routes, first-screen fit, same-origin requests only, demo cleanup, zero serious/critical Axe violations, and no console errors. See [review JSON](evidence/repair-8/live-review.json), [landing mobile](evidence/repair-8/landing-mobile.png), [demo mobile](evidence/repair-8/demo-mobile.png), and [404 mobile](evidence/repair-8/404-mobile.png).
- `verify-url.sh` — PASS: HTTPS 200, page load 581 ms, title, `lang=en`, one `h1`, main landmark, image alt text, button names, and zero browser errors. See [report](evidence/repair-8/verify.json), [desktop](evidence/repair-8/screenshot-desktop.png), and [mobile](evidence/repair-8/screenshot-mobile.png).
- Live public/private policy probe — PASS: restrictive CSP with response-header `frame-ancestors 'none'`, `nosniff`, `no-referrer`, and `DENY`; hashed assets are one-year immutable; private HTML/API responses are `no-store` and `noindex, nofollow, noarchive`.
- Mobile Lighthouse — 99 performance, 100 accessibility, 100 best practices, 100 SEO; FCP 985 ms, LCP 1,585 ms, TBT 67 ms, CLS 0, and 120,762 bytes. See the [JSON report](evidence/repair-8/lighthouse.report.json).
- Three-second 100 requests/second `/health` smoke — PASS: 300/300 `200`; p95 254 ms and maximum 265 ms.

Package/consumer checks do not apply to this web-with-backend artifact. It has no sign-in flow or service worker/PWA, so identity-provider and update checks do not apply.

## Known gaps and next step

No release-blocking finding remains. Keep the checked-in topology check as a required post-deploy gate, and explicitly deactivate any previous active revision even if it carries 0% traffic: this service has one SQLite writer and its live checker intentionally rejects more than one active revision.
