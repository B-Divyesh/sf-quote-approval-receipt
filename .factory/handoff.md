# Repair handoff — release candidate

Work order: `quote-approval-receipt-repair-1`

Independent report: `aae284b2d059d1d07bf1af699c49259250389e7d`

Failed candidate: `85c381d3cf4cda5c68a05a8e0ec26f9b203391bd`

Date: 2026-08-28 UTC

## What was repaired

- Production state is no longer split between replicas. The Container App has one replica and a mounted Azure Files volume. SQLite runs on local disk; every committed mutation is copied byte-for-byte to the durable volume and restored on boot. A regression restores a fresh SQLite process from that snapshot.
- Demo creation/read continuity is covered by 16 repeated reads locally and 16 fresh live create/read pairs.
- `retention_days: 365` now requires live server-side verification with the Sociobot license endpoint. Missing, forged, invalid, unavailable, or malformed verification cannot grant Studio retention. A mock integration test proves a verified license receives 365 days.
- The public Studio offer is fail-closed. The frontend requests same-origin `/api/studio`; checkout appears only when Sociobot returns this product at USD 29. Sociobot currently returns 404, so the UI truthfully says purchases are paused and never links to the dead checkout. Existing licenses can still be restored.
- PDF output now embeds DejaVu Sans and preserves Unicode. Tests extract and compare the approver, title, quote, client, timestamp, snapshot hash, and consent text.
- Currency validation accepts the six currencies offered by the UI and rejects unsupported values before storage. Formatting also has a safe fallback for old malformed records.
- Private pages, management pages, and every API response now send `Cache-Control: no-store`; approval, receipt, and management pages send response-level `X-Robots-Tag`.
- Hashed assets use one-year immutable caching and support Brotli/gzip compression. Unknown HTML routes retain the designed screen and return HTTP 404.
- Default startup logging now prints generated/persisted configuration and build identity without requiring `RUST_LOG`.
- The 390 px layout reflows at 200% text. Header, footer, demo, and text controls meet the 44 px target. Skip-link and SPA route focus are deterministic.
- `.factory/claims.json` now lists every public/README promise and maps each to one behavioral `@claim` test. `.factory/copy-audit.md` reflects conditional Studio copy.

## Verification evidence

Clean/local:

- `npm ci`: pass, 31 packages, 0 vulnerabilities.
- `npm test`: pass, 4 Rust tests and 15 Playwright tests across desktop Chromium and 390 px mobile.
- `npm run check`: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --all-targets -- -D warnings`: pass.
- `npm audit --audit-level=high`: pass, 0 vulnerabilities.
- `BUILD_SHA=repair-local cargo build --release`: pass.
- Runtime with only `PORT`: pass. First boot logged `privacy_salt=generated`; restart logged `privacy_salt=persisted`; a record created before restart remained readable.
- Frontend: JS 25,721 bytes raw / 8,780 gzip; CSS 15,456 / 4,190; hero WebP 104,994 bytes.
- `/opt/fleet/lib/verify-url.sh`: pass; 651 ms, title/lang/main/alt checks pass, zero console errors.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.7 s, TBT 60 ms, CLS 0.
- Playwright axe: zero serious/critical findings on `/`, `/new`, `/demo`, `/privacy`, `/terms`, and the styled 404.
- Keyboard: skip link focuses `main`; Enter activates the demo link; route change focuses and announces its H1.
- Reduced motion: zero running animations.
- Text reflow: `/`, `/new`, and `/privacy` have no horizontal overflow at 390 px with 200% text.
- Response policy: unknown path 404; `/manage/*` noindex/no-store; APIs no-store; hashed JS Brotli-compressed and immutable.
- Offline/update: not applicable. This is an online backend workflow and makes no offline/PWA claim.
- Package/consumer: not applicable to the `web-with-backend` artifact.

Live production on the mounted single-replica revision:

- `/health` returned the deployed repair build SHA.
- 16/16 fresh `POST /api/demo` plus immediate `GET /api/share/<token>` pairs returned `201/200`.
- One normal record returned 200 on 16/16 share reads and 12/12 owner reads.
- A live international approval preserved `Zoë García`, `Ateliê Norte`, `Café São Bento`, and `José Núñez — Direção`; PDF.js extracted those exact strings plus time, consent, and snapshot hash.
- Unsupported currency returned 422 with recovery text. An unlicensed 365-day request returned 403.
- `/unknown-release-check` returned 404. `/manage/*` returned `X-Robots-Tag: noindex, nofollow, noarchive` and `Cache-Control: no-store`.
- `/api/studio` returned `{ "available": false, "checkout_url": null }`, matching the upstream product 404 without exposing a dead purchase link.

## Deployment configuration

- Artifact remains one Rust/axum backend serving the Vite/TypeScript frontend on `PORT=8080`.
- Azure Container App: `sf-quote-approval-receipt`, one minimum and maximum replica.
- Durable storage: environment storage `quote-approval-receipt-data`, share `sf-quote-approval-receipt-data`, mounted at `/data`.
- Runtime overrides: `DATA_DIR=/tmp/quote-approval-receipt`, `DURABLE_DATA_DIR=/data`.
- Container remains non-root. The image includes only the DejaVu font package needed for Unicode receipts.

## Known external gap

The Sociobot billing catalog still does not contain `quote-approval-receipt`; its checkout returns 404. Repository rules prohibit changing billing infrastructure. New Studio sales therefore remain visibly paused and no dead checkout is shown. Once the factory registers the product at USD 29, the existing availability check will expose checkout automatically; server verification is already enforced.
