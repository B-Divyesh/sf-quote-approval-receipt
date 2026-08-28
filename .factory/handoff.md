# Handoff — Quote Approval Receipt v1

## What shipped

- Rust 2021 `axum` service with SQLite persistence, structured logs, graceful shutdown, `/health`, generated first-boot privacy salt, hourly retention cleanup, and request limits keyed from the first `X-Forwarded-For` hop.
- Quote creation for an existing scope, line items, totals, sender, client, configurable consent text, and retention choice.
- Unguessable approval links. A client supplies their name and title, then approves or requests changes. Each quote accepts one final decision.
- Timestamped receipt pages and downloadable PDF receipts. Each receipt includes the quote, decision maker, consent copy, decision time, and SHA-256 snapshot hash.
- Browser-held owner keys for record status, JSON export, and permanent deletion.
- Isolated 24-hour demo workspaces at `/demo`, seeded with a Northstar Studio quote. The demo is resettable and does not write browser owner keys.
- $29 one-time Studio UI, hosted Sociobot checkout link, license return capture, daily verification cache, and paste-to-restore flow. A valid cached license exposes 365-day retention; free use provides 30 days.
- Cassette-era zine design, original generated hero art, responsive 390 px layout, keyboard-native controls, reduced-motion behavior, designed loading/error/empty states, `/privacy`, `/terms`, and a styled 404 route.
- Complete metadata, social card, favicon, sitemap, robots rules, CSP and other response headers, plus `noindex` headers for approval and receipt paths.

## How to run

```sh
npm install
npm run build
PORT=8080 cargo run
```

The container build is `docker build --build-arg BUILD_SHA=<sha> .`. It starts as a non-root user and needs only `PORT`; mount `/data` for persistence.

## Verification

- `npm test`: passed. Two Rust unit tests and nine Playwright checks passed, including all five claim tests, the 390 px project, and axe serious/critical checks.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo build --release`: passed.
- `npm audit`: 0 vulnerabilities.
- `verify-url.sh`: passed against the release binary; one `<h1>`, `lang`, main landmark and alt text present, with no console errors. Measured load was 571 ms on the local runner.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100. LCP 1.8 s, CLS 0, total blocking time 0 ms.
- Production bundle: 8.56 KB gzipped JavaScript, 4.05 KB gzipped CSS. Hero WebP: 104 KB.
- Load smoke: 100 concurrent health requests completed in 402 ms on the local runner.
- Deep links return 200. Approval pages return `X-Robots-Tag: noindex, nofollow, noarchive`.

## Known gaps and next steps

- The factory must register the paid product and confirm its $29 price before release. No product ID is hardcoded.
- A container engine was not installed in the worker, so the Dockerfile was reviewed but not executed. Both stages were verified independently with `npm run build` and `cargo build --release`.
- This product intentionally does not claim signature-law compliance, write proposals, collect payment, or authenticate user accounts.
