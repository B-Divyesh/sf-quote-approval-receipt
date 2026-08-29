# Polish round 4 — cumulative zero-finding receipt

- Work order: `quote-approval-receipt-polish-4`
- Reviewed candidate: `8f22bab8d72a2c9ea7bb6a19c44af86083fa589a`
- Review commit: `4a1f3f96d053e5049b95517abf3d8fadb1a0b2b7`
- Repair commit: `cfac15d79d708086b8324d8ca2902defe96c4c50`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Deployed revision: `sf-quote-approval-receipt--polish4`
- Image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:repair-8` (`sha256:a107f6d5c869deecb83ba1714c7e268b62f3bb2c6a13a103fc34ffc09173291d`)

## Adversarial-review findings

| Finding ID | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Reapplied the durable single-replica Azure Files contract. The direct `?demo=1` sample remains isolated, readable across fresh processes, resettable, and discardable. | `test:live-demo`: 20/20 `201` → `200` → `404`; [demo mobile](evidence/polish-4/demo-mobile.png); live <https://quote-approval-receipt.sociobot.in/?demo=1>. |
| F-1-2 | Retained clipped cassette recovery art and the 390 px regression. | `@mobile`; [404 mobile](evidence/polish-4/404-mobile.png); live missing route is HTTP 404 with no overflow. |
| F-1-3 | Retained the 13-entry observable claim manifest and integrity gate. Removed the final untestable global-limiter assertion. | `test:claims-manifest`; all 13 clean-clone commands; [landing mobile](evidence/polish-4/landing-mobile.png); live same-origin request check. |
| F-1-4 | Retained literal preview, step, and Studio-license labels while preserving the cassette-zine art direction. | Landing copy audit and landing test; [landing mobile](evidence/polish-4/landing-mobile.png); live `/`. |
| F-1-5 | Retained per-route title, description, canonical, Open Graph, and Twitter updates. | `test:live-review`; [live-review.json](evidence/polish-4/live-review.json); live `/`, `/new`, `/privacy`, `/terms`, and 404. |
| F-2-1 | Retained transactional demo deletion on Reset, Start for real, and route exit. | `@claim:demo-sandbox`; [demo mobile](evidence/polish-4/demo-mobile.png); live review records no `demo:` keys and deleted share `404`. |
| F-2-2 | Retained `This page was not found` with the broken-cassette recovery art. | Route/a11y test; [404 mobile](evidence/polish-4/404-mobile.png); live missing route. |
| F-3-1 | Reapplied one replica, `/durable` Azure Files, and `DURABLE_DATA_DIR=/durable`. | `test:live-topology`, 20/20 `test:live-demo`; live `/health` reports `durable_snapshot: true`. |
| F-3-2 | Kept `Receipt: PDF after a decision`. | Landing test; [landing mobile](evidence/polish-4/landing-mobile.png); live `/`. |
| F-3-3 | Kept `Turn an existing quote into an approval link`. | Landing test; [landing mobile](evidence/polish-4/landing-mobile.png); live `/`. |
| F-3-4 | Kept `PDF receipt` as the single output term. | `@claim:pdf-receipt`; [demo mobile](evidence/polish-4/demo-mobile.png); live demo decision flow. |
| F-3-5 | Kept the literal Privacy H1. | `test:live-review`; [privacy mobile](evidence/polish-4/privacy-mobile.png); live `/privacy`. |
| F-3-6 | Kept the literal Terms H1. | `test:live-review`; [terms mobile](evidence/polish-4/terms-mobile.png); live `/terms`. |
| F-3-7 | Kept scoped license-entry wording; the unsupported cross-device promise remains absent. | `@claim:studio-offer`; [landing mobile](evidence/polish-4/landing-mobile.png); live Studio section. |
| F-3-8 | Kept unverified Dodo/merchant language out of legal copy; conditional Sociobot checkout is covered. | `@claim:studio-offer`; [privacy](evidence/polish-4/privacy-mobile.png), [terms](evidence/polish-4/terms-mobile.png); live legal routes. |
| F-4-1 | Removed `This one-replica contract also makes the 15-write-per-second limit global for the live service.` from README and the matching deployment-guide statement. Added a regression assertion rejecting both phrases. | `test:claims-manifest`; clean clone at `cfac15d`; live deployed repair <https://quote-approval-receipt.sociobot.in/>. |

## Independent-verification findings

| Finding ID | Change made | Evidence |
| --- | --- | --- |
| V-1-Critical, V-2-Critical, V-3-Critical, V-5-Critical, V-6-Critical | Reapplied durable, single-replica state and exercised it from fresh clients. | `test:durable-snapshot`, `test:live-topology`, 20/20 `test:live-demo`; [demo mobile](evidence/polish-4/demo-mobile.png); live `/health`. |
| V-1-High-Studio | Kept server-verified 365-day retention and conditional checkout. | `@claim:retention-policy`, `@claim:studio-offer`; [landing mobile](evidence/polish-4/landing-mobile.png); live `/`. |
| V-1-High-PDF | Kept Unicode-safe PDF receipts containing approver, time, consent, hash, and quote snapshot. | `@claim:pdf-receipt`; [demo mobile](evidence/polish-4/demo-mobile.png); live demo. |
| V-1-High-Claims, V-2-High-README-claims | Every retained claim maps to one exact command; the final untestable README claim is gone. | All 13 exact clean-clone claim commands; `test:claims-manifest`; live `/`. |
| V-1-Medium-Validation | Kept actionable currency and numeric validation. | `invalid currencies fail cleanly and response policies are enforced`; [new mobile](evidence/polish-4/new-mobile.png); live `/new`. |
| V-1-Medium-Reflow-targets, V-4-P3 | Kept 390 px/200% reflow and 44 px controls across public routes. | `@mobile`; [landing](evidence/polish-4/landing-mobile.png) and [404](evidence/polish-4/404-mobile.png); live public routes. |
| V-1-Medium-Cache/privacy, V-1-Low-manage-robots | Kept private `no-store`/`noindex` and same-origin demo traffic. | `@claim:private-links`, `@claim:first-party-only`; [live-review.json](evidence/polish-4/live-review.json); live demo. |
| V-1-Low-404-status | Kept a styled HTTP 404. | Route response test; [404 mobile](evidence/polish-4/404-mobile.png); live missing route. |
| V-1-Low-startup-log | Kept only-`PORT` startup, generated/persisted salt, and build identity. | `test:runtime-contract`; live `/health` reports `cfac15d…`. |
| V-2-High-owner-PDF | Kept sender receipt/PDF access after a decision. | `@regression:owner-receipt-link`; durable live demo continuity. |
| V-2-High-Docker | Kept multi-stage, non-root, `.git`-independent `rust:1-slim` container. | ACR run `chwu` from `.git`-excluded source; `test:runtime-contract`. |
| V-3-rate-limit-topology, V-5-High-rate-limit | Kept 15-write admission and retry guidance; removed the unprovable global marketing claim. | `test:live-rate-limit`: 15 writes then `429` + `Retry-After: 1`; live URL. |

## Final verification

- Clean GitHub clone: `/tmp/qar-polish4-clean.kk0Oqu/repo` at `cfac15d79d708086b8324d8ca2902defe96c4c50`; `npm ci` reported zero vulnerabilities. Every literal manifest command passed individually: 11 `npm test -- --grep @claim:<id>` commands, `test:runtime-contract`, and `test:durable-snapshot`.
- `npm test` passed with 4 Rust and 20 Playwright tests. `npm run build`, `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high` passed.
- `verify-url.sh` completed in 618 ms with `lang=en`, title, one H1, main, alt text, and no console errors: [verify.json](evidence/polish-4/verify-url/verify.json). `test:live-review` found zero serious/critical Axe findings on every checked route.
- [Lighthouse](evidence/polish-4/lighthouse-retry.json): Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 0.1 s, TBT 0 ms, CLS 0, 118 KiB transferred.
- Cold visual inspection passed for [landing](evidence/polish-4/landing-mobile.png), [demo](evidence/polish-4/demo-mobile.png), [privacy](evidence/polish-4/privacy-mobile.png), and [404](evidence/polish-4/404-mobile.png). The plain first screen, usable demo, clear recovery, and cassette-era identity are intact.

No finding remains open.
