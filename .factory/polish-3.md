# Polish round 3 — cumulative finding receipt

- Work order: `quote-approval-receipt-polish-3`
- Reviewed candidate: `6bb800a4f4228660bbc8335d0f14f272541c4116`
- Review commit: `13bb7540d2364d9b68a9ccd18d9cf3bcb14fe692`
- Repaired application commit: `309f25e5054d1672dacbf024a573352853b88e9c`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Deployed revision: `sf-quote-approval-receipt--0000021`

## Adversarial review findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 — production state split | Restored a single serving replica with the existing Azure Files share mounted at `/durable`. The snapshot writer now holds the only SQLite pool connection while copying, so a committed snapshot cannot race a write. | `npm run test:durable-snapshot` replaces the server after 20 writes; `live-demo.log` records 20/20 fresh Chromium create 201 → read 200 → exit cleanup 404 cycles; `deployment-live.json` records one replica and the mount. Live `/health` reports `309f25e…`. |
| F-1-2 — 404 mobile overflow | Retained the clipped product-specific 404 art and expanded the 390 px/200% reflow regression across every public route. Legal links now also meet the 44 px target. | Test `@mobile 390px pages reflow at 200% text and controls meet target size`; `live-review.json` reports no overflow; screenshot `evidence/polish-3/404-mobile.png`; live `/missing-review-route` returns 404. |
| F-1-3 — public claims lacked observable tests | Added a manifest integrity gate that requires 13 unique IDs and exactly one matching claim test. Expanded Studio and durability tests; retained observable PDF, privacy, retention, control, and isolation tests. | `npm run test:claims-manifest`; all 13 exact manifest commands passed from clean clone `/tmp/qar-polish3-clean.yZwPYH/repo`; full suite reports 20/20 browser tests. |
| F-1-4 — decorative or unclear labels | Retained the literal section labels and action names from earlier repairs and removed the remaining vague first-screen terms. | `.factory/copy-audit.md`; test `landing says what to do…`; screenshot `evidence/polish-3/landing-mobile.png`; live first screen shows the task, audience, action, result, and three facts. |
| F-1-5 — shared route metadata | Retained route-specific titles and updated description, canonical, Open Graph, Twitter, robots, H1, and focus assertions for all public routes and the real 404. | Test `each public route has real status, metadata, headings, legal links, and a bounded 404`; `live-review.json`; `verify.json`. |
| F-2-1 — demo survived exit | Retained transactional demo deletion. Reset deletes the old workspace before reseeding; Start for real deletes the current workspace and clears every `demo:` session key. | `@claim:demo-sandbox`; `live-demo.log` 20/20 cleanup 404; `live-review.json` records `deletedStatus: 404` and `demoKeys: []`; screenshot `evidence/polish-3/demo-mobile.png`. |
| F-2-2 — metaphorical 404 heading | Retained the literal `This page was not found` H1 while preserving the cassette illustration. | Route test and Axe pass; `live-review.json`; screenshot `evidence/polish-3/404-mobile.png`; live route returns HTTP 404. |
| F-3-1 — live demo state regressed | Made the deploy topology an asserted release contract: manifest requires `minReplicas = maxReplicas = 1`, Azure Files, `/durable`, and `DURABLE_DATA_DIR`. Deployed that exact manifest. | `npm run test:durable-snapshot`; `deployment-live.json`; `live-demo.log` proves 20/20 independent processes; `live-rate-limit.log` proves one 15-write allowance followed by five 429 responses. |
| F-3-2 — vague `Clear: PDF…` fact | Rewrote it as `Receipt: PDF after a decision`. | Test `landing says what to do…`; `.factory/copy-audit.md`; `landing-mobile.png`; live `/`. |
| F-3-3 — vague `one clear link` heading | Rewrote it as `Turn an existing quote into an approval link`. | Test `landing says what to do…`; `.factory/copy-audit.md`; `landing-mobile.png`; live `/`. |
| F-3-4 — README renamed the output | Standardized the README on `PDF receipt`. | README text review; clean manifest/full suite; product controls and README now use the same term. |
| F-3-5 — Privacy slogan H1 | Replaced it with `How we store quote and approver data`. | Route metadata/H1 test; `live-review.json`; screenshot `evidence/polish-3/privacy-mobile.png`; live `/privacy`. |
| F-3-6 — Terms promotional H1 | Replaced it with `Terms for quote approval records`. | Route metadata/H1 test; `live-review.json`; screenshot `evidence/polish-3/terms-mobile.png`; live `/terms`. |
| F-3-7 — untested cross-device claim | Removed the README promise that a license works on any device. The remaining statement only says that license holders can enter a license, which the UI test observes. | `@claim:studio-offer`; clean-clone claim pass; README audit. |
| F-3-8 — unlisted payment-party statements | Removed Dodo and merchant-of-record assertions from public copy. Kept the observable statement that checkout opens on Sociobot and expanded `studio-offer` to assert the exact checkout URL, available/unavailable states, license entry, and matching legal copy. | `@claim:studio-offer`; route/legal tests; screenshots `privacy-mobile.png` and `terms-mobile.png`; live `/privacy` and `/terms`. |

## Earlier independent-verification findings

| Finding | Current resolution | Evidence |
| --- | --- | --- |
| V-1-Critical, V-2-Critical, V-3-Critical — split production state | One durable writer, atomic snapshot, one mounted replica. | `test:durable-snapshot`, `deployment-live.json`, `live-demo.log`. |
| V-1-High-Studio — dead or bypassable paid retention | The server verifies a Sociobot license before accepting 365-day retention; unavailable checkout is not presented as purchasable. | `@claim:studio-offer`, `@claim:retention-policy`. |
| V-1-High-PDF — international text corruption | DejaVu-backed receipts preserve `José Núñez — Direção` and the complete snapshot. | Rust test `pdf_preserves_complete_international_receipt_text`; `@claim:pdf-receipt`. |
| V-1-High-Claims, V-2-High-README-claims | Thirteen claims cover every material public promise, with a uniqueness/coverage gate. | `test:claims-manifest`; every exact command passed from the clean clone. |
| V-1-Medium-Validation | Currency and numeric/item bounds fail with actionable validation. | Test `invalid currencies fail cleanly and response policies are enforced`; Rust and integration suites. |
| V-1-Medium-Reflow-targets | Every public route reflows at 390 px and 200%; visible links/buttons are at least 44 px. | `@mobile`; live mobile screenshots in `evidence/polish-3/`. |
| V-1-Medium-Cache/privacy | Private routes are `no-store`/`noindex`; hashed assets are Brotli-compressed and immutable. | `@claim:private-links`; response-policy integration test; live route review. |
| V-1-Low-404-status | Unknown routes return the styled page with a real HTTP 404. | Route test; `live-review.json`; `404-mobile.png`. |
| V-1-Low-manage-robots | Management, approval, receipt, and API routes emit response-level noindex. | `@claim:private-links`; response-policy integration test. |
| V-1-Low-startup-log | Startup records whether the salt was generated or persisted and includes build identity without exposing secret material. | `npm run test:runtime-contract`. |
| V-2-High-owner-PDF | A sender can reopen a decided quote and download its receipt PDF. | `@regression:owner-receipt-link`. |
| V-2-High-Docker | Docker uses `rust:1-slim`, build arguments rather than `.git`, and a non-root runtime. | `test:runtime-contract`; successful ACR build `chu0`. |
| V-3-rate-limit-topology | The live service has one limiter allowance and returns retry guidance. | `live-rate-limit.log`: 15×422 followed by 5×429, all with `Retry-After: 1`; `deployment-live.json`. |
| V-4-P3 — 404 2 px overflow | The decorated panel is clipped and regression-tested. | `@mobile`; `live-review.json`; `404-mobile.png`. |

## Final evidence

- Clean clone: `npm ci` found zero vulnerabilities; every one of the 13 claim commands passed; `npm test` passed 4 Rust and 20 Playwright tests.
- Static gates: `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high` passed.
- Build: `dist/` produced 27.11 kB JavaScript (8.96 kB gzip) and 15.55 kB CSS (4.20 kB gzip).
- Accessibility: every public screen had zero serious/critical Axe findings; `verify-url.sh` found one H1, `lang=en`, a main landmark, complete image alt text, and no console errors.
- Routing/assets: every public route, robots file, sitemap, icon, social card, and external factory link resolved to 200; the deliberate missing route returned 404.
- Mobile Lighthouse 13.0.1: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.53 s, TBT 6 ms, CLS 0.
- Load smoke: 100 concurrent `/health` requests returned 100×200 in 359 ms (about 279 requests/s).
- Cold live screenshots: `landing-mobile.png`, `demo-mobile.png`, `new-mobile.png`, `privacy-mobile.png`, `terms-mobile.png`, `404-mobile.png`, `screenshot-mobile.png`, and `screenshot-desktop.png` under `.factory/evidence/polish-3/`.

No finding from reviews 1–3 or verifications 1–4 remains open.
