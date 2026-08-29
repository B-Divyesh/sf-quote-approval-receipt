# Independent verification 6 handoff — FAIL

- Work order: `quote-approval-receipt-verify-6`
- Candidate commit: `8f22bab8d72a2c9ea7bb6a19c44af86083fa589a`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Verified: 2026-08-29 UTC
- Verdict: **FAIL — do not release**

The candidate's local product is buildable and its full claim suite passes. The deployed service is nevertheless unsafe for the product's central job: backend state is not durable or consistently shared between requests.

Fresh live evidence:

- `/health` returns the candidate SHA exactly, but `"durable_snapshot": false`.
- An independently built Vite asset with `VITE_BUILD_SHA=8f22bab8d72a2c9ea7bb6a19c44af86083fa589a` is exactly the deployed asset: SHA-256 `006333e731f10d8166ba5cab21ad2b5ff83dfe7e76ea5d95624fa90f2e6ade68`.
- Eight direct, new-client demo flows each got `POST /api/demo` `201`, then `GET /api/share/<returned token>` `404`. Cleanup was also `404` on four of eight attempts. A normal quote create followed by a decision similarly returned `201` then `404`.

This reproduces the prior state/replica failure despite the candidate's handoff claiming a durable single-replica deployment. A recipient opening a private approval link may therefore not see a quote the sender just created. The documented retention, immutable receipt, export, deletion, and one-click demo promises are not defensible in production.

## How verified

```sh
npm ci
# every command in .factory/claims.json, exactly as declared
npm test
npm run check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
VITE_BUILD_SHA=8f22bab8d72a2c9ea7bb6a19c44af86083fa589a npm run build
LIVE_URL=https://quote-approval-receipt.sociobot.in npm run test:live-demo
LIVE_URL=https://quote-approval-receipt.sociobot.in npm run test:live-rate-limit
```

All 13 local claim commands passed, as did `npm test` (4 Rust tests and 20 Playwright tests), type check, format check, Clippy with warnings denied, and high-severity audit (zero vulnerabilities). Production output is 27.10 kB JS (8.96 kB gzip) and 15.55 kB CSS (4.20 kB gzip). Docker could not be built in this verifier container because the `docker` executable is absent; the native release/runtime contract test passed.

Live: the scripted cookie-affine browser demo check happened to pass 20/20 and the fixed-client limit check observed 15 writes accepted then the 16th `429 Retry-After: 1`. Those checks do not override the direct no-cookie cross-request failure above. The cold landing request log was same-origin only; headers include self-only CSP, `nosniff`, `no-referrer`, `DENY`, `frame-ancestors 'none'`, `no-store` on private paths, and immutable caching on the hashed asset. Playwright AxeBuilder found zero serious/critical violations on `/`, `/new`, `/privacy`, `/terms`, and the 404 at 390 px; no console/page errors or horizontal overflow. The standalone Axe CLI could not launch its Selenium ChromeDriver against the supplied Playwright Chromium.

## Required next step

Restore the actual Container App state contract before another release review: exactly one serving replica, a working Azure Files durable mount at `/durable`, `DURABLE_DATA_DIR=/durable`, and `/health` reporting `durable_snapshot: true`. Then test the approval link from a separate no-cookie client and rerun live demo, persistence/replacement, and rate-limit checks.
