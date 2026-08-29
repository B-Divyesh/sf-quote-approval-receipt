# Polish 3 handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-polish-3`
- Repaired application commit: `309f25e5054d1672dacbf024a573352853b88e9c`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Live revision: `sf-quote-approval-receipt--0000021`
- Image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:309f25e5054d`
- Digest: `sha256:7dbdba2a28419064c9daccb031a86bc7556413518558c394bbfd2dad0a579121`

## Delivered

Closed every finding in `.factory/review-1.md`, `review-2.md`, `review-3.md`, and the earlier verification reports. The public app now runs as one mounted durable replica, and the snapshot operation excludes concurrent SQLite writes. The first-screen facts, approval-link heading, README receipt term, Privacy and Terms H1s, and payment wording are literal and consistent.

The one-click `?demo=1` path creates an isolated sample, shows its persistent banner, resets by deleting the old workspace, and deletes the active workspace plus browser pointer on Start for real. All routes have real HTTP status, route-specific metadata, one H1, focus restoration, legal links, mobile reflow, and a product-specific 404.

`.factory/claims.json` now has 13 unique claims. `test:claims-manifest` fails if an ID lacks exactly one tagged test. The Studio claim observes the exact Sociobot checkout route, both availability states, license entry, and policy copy. Untestable cross-device, Dodo, and merchant assertions were removed.

The catalog description is now: `Record who approved each quote and issue a named, timestamped PDF receipt.` (74 characters, verb first).

## Clean-clone verification

Clone: `/tmp/qar-polish3-clean.yZwPYH/repo`, checked out at `309f25e5054d1672dacbf024a573352853b88e9c`.

- `npm ci`: pass, 0 vulnerabilities.
- Every exact command in `.factory/claims.json`: 13/13 pass.
- `npm test`: pass — manifest gate, production build, 4 Rust tests, runtime replacement, 20-record durable replacement, and 20/20 Playwright tests.
- `npm run check`: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --all-targets -- -D warnings`: pass.
- `npm audit --audit-level=high`: pass.
- Build output: JS 27.11 kB raw / 8.96 kB gzip; CSS 15.55 kB raw / 4.20 kB gzip; `dist/` produced.

Run the same gates with:

```bash
npm ci
mapfile -t claim_tests < <(jq -r '.[].test' .factory/claims.json)
for claim_test in "${claim_tests[@]}"; do bash -lc "$claim_test"; done
npm test
npm run check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
```

## Deployment and cold live verification

ACR build `chu0` built the `.git`-excluded source with `BUILD_SHA=309f25e5054d1672dacbf024a573352853b88e9c`. Azure Container Apps reports revision `0000021` healthy with one active replica, `minReplicas=1`, `maxReplicas=1`, and Azure Files `quote-approval-receipt-data` mounted at `/durable`. `/health` returns the exact build SHA.

- `npm run test:live-demo`: pass — 20/20 separate Chromium processes observed create 201, immediate read 200, visible sample, empty demo storage after exit, and deleted share 404. Attempt one also proved Reset deletes the prior sample.
- `npm run test:live-review`: pass — home, builder, Privacy, Terms, and 404 status/title/H1/description/canonical/Open Graph/Twitter/legal-link checks; first-screen fit; same-origin traffic; demo deletion; zero serious/critical Axe issues; zero console errors.
- `/opt/fleet/lib/verify-url.sh`: pass — 589 ms load, `lang=en`, one H1, main landmark, complete alt text, no unlabeled buttons, no console errors.
- Link/asset crawl: every public route, robots file, sitemap, icon, social card, and external factory link resolved to 200.
- Live rate burst: 15×422, then 5×429 with `Retry-After: 1`.
- Load smoke: 100 concurrent health requests returned 100×200 in 359 ms, about 279 requests/s.
- Lighthouse 13.0.1 mobile: 100 performance, 100 accessibility, 100 best practices, 100 SEO; FCP 0.93 s, LCP 1.53 s, TBT 6 ms, CLS 0.

The full finding map is in `.factory/polish-3.md`. Durable configuration, browser output, screenshots, Lighthouse summary, and live logs are in `.factory/evidence/polish-3/`.

## Known gaps

None found after the final cold live recheck. No infrastructure, billing provider, or DNS setting was changed outside the supplied Container Apps work-order configuration.
