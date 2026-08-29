# Independent product verification 6 — FAIL

- Work order: `quote-approval-receipt-verify-6`
- Candidate commit: `8f22bab8d72a2c9ea7bb6a19c44af86083fa589a`
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Verdict: **FAIL — do not release**

## Decision

The deployed frontend and health identity are genuinely this candidate, but the live backend cannot reliably carry a quote from one request to the next. That breaks the smallest useful product: a contractor cannot safely send a private approval link and expect the recipient to find the fixed quote.

`GET /health` returned:

```json
{"build_sha":"8f22bab8d72a2c9ea7bb6a19c44af86083fa589a","durable_snapshot":false,"status":"ok"}
```

The `false` durable mode contradicts the README, `.factory/deployment.md`, and the prior repair handoff's required release topology. A production build made from this checkout with `VITE_BUILD_SHA` set to the candidate SHA generated `/assets/index-3X5an3I2.js`; it has the same SHA-256 as the asset served by the URL:

```text
006333e731f10d8166ba5cab21ad2b5ff83dfe7e76ea5d95624fa90f2e6ade68
```

This is fresh deployment evidence, not a stale frontend.

## First-read gate

**PASS for the cold first screen.** It answers all required questions in plain words:

- What: “Record who approved your quote”.
- For whom: contractors whose clients approve by email or chat and later change expectations.
- First action: “Try it with sample data”, with “Loads a private sample quote. Nothing enters your records.”

It also presents the required one-click action and facts: free 30-day retention, export/delete, and a PDF receipt. The live demo's core API state failure below means the overall candidate still fails the demo-sandbox acceptance gate.

## Release-blocking defects

### Critical — live backend state is split or ephemeral

Fresh direct no-cookie evidence, using a new forwarded client address for every iteration:

```text
iteration  create  immediate share read  cleanup
1          201     404                   404
2          201     404                   204
3          201     404                   404
4          201     404                   204
5          201     404                   404
6          201     404                   204
7          201     404                   404
8          201     404                   204
```

Every `GET /api/share/<token returned by the immediately preceding create>` should be 200. It was 404 in 8/8 cases. Four deletes also missed the created workspace. Separately, a representative real quote create was 201, followed immediately by a valid decision submission that returned 404; its cleanup using the returned owner token succeeded.

This is consistent with requests being served by different local SQLite instances. It is especially damaging here: the sender and external approver necessarily use different browsers/connections. `durable_snapshot: false` proves the checked durable persistence boundary is absent. The live `test:live-demo` script passed 20 cookie-affine Chromium runs, which is insufficient; it did not catch the no-cookie, cross-client use case the product exists to serve.

Required repair: deploy the actual contract (`minReplicas=maxReplicas=1`, Azure Files mounted at `/durable`, `DURABLE_DATA_DIR=/durable`) and require health to return `durable_snapshot: true`. Re-test sender creation and recipient approval from independently fresh clients, plus a replacement-process persistence check.

## Claims gate — all local commands passed

`.factory/claims.json` exists, has 13 complete and uniquely mapped observable claims, and every listed command was run exactly after `npm ci` from this clean checkout.

| Claim | Result |
| --- | --- |
| demo-sandbox | PASS locally; **FAIL live cross-client** |
| quote-snapshot | PASS locally; **blocked live** |
| pdf-receipt | PASS locally; **blocked live** |
| record-control | PASS locally; **blocked live** |
| first-party-only | PASS |
| retention-policy | PASS locally; **not reliable live** |
| studio-offer | PASS |
| private-links | PASS locally; **blocked live** |
| single-decision | PASS locally; **blocked live** |
| network-address-privacy | PASS |
| rate-limits | PASS locally and live |
| runtime-contract | PASS locally; **contradicted by live durable health** |
| durable-snapshot | PASS locally; **contradicted by live durable health** |

## Local quality gates

- `npm ci`: PASS; 30 packages, zero audit vulnerabilities.
- `npm test`: PASS; manifest, production build, 4 Rust tests, runtime contract, durable replacement, 18 desktop plus 2 mobile Playwright tests.
- `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high`: PASS.
- Exact `npm run build`: PASS and produced `dist/`.
- Each literal command in the claims manifest: PASS.
- Production bundle: JS 27.10 kB / 8.96 kB gzip; CSS 15.55 kB / 4.20 kB gzip; generated hero WebP 104,994 bytes.
- Docker image build: not run because this supplied verifier has no `docker` executable. The release binary and only-`PORT` runtime contract did pass.

## Live functional, privacy, accessibility, and delivery checks

- Backend concurrency/identity: 100 concurrent health requests returned 100×200 in 1,194 ms, one candidate SHA, and only `durable_snapshot: false`.
- Demo script: `LIVE_URL=… EXPECTED_BUILD_SHA=8f22… npm run test:live-demo` passed 20 cookie-affine browser contexts; it does not invalidate the direct 8/8 recipient-flow failure.
- Rate allowance: `npm run test:live-rate-limit` passed. The observed allowance is 15 writes per client per second; request 16 returned `429` and `Retry-After: 1`.
- Privacy request log: cold landing loaded only the product origin (HTML, CSS, JS, image, and `/api/studio`); no analytics, ads, CDN, font, or third-party runtime request. The demo-flow privacy claim passes locally; no cross-origin call was observed live.
- Headers: `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `X-Frame-Options: DENY`, restrictive Permissions Policy, self-only CSP with `frame-ancestors 'none'`; private routes are `noindex`/`no-store`; deployed hashed JS is `public, max-age=31536000, immutable`.
- Responsive/a11y: Playwright AxeBuilder found zero serious/critical violations on landing, builder, Privacy, Terms, and 404 at 390 px; no horizontal overflow, console errors, or page errors. Local suite also covered keyboard skip link, route focus, 200% text, 44 px targets, and reduced motion. The standalone Axe CLI was attempted but failed to start because its Selenium ChromeDriver is incompatible with the preinstalled Playwright Chromium; equivalent Playwright Axe coverage passed.
- Candidate match: health SHA, Vite asset filename, and Vite asset SHA-256 all match the candidate. `/`, `/new`, `/privacy`, and `/terms` return 200; unknown route returns the designed 404.

## Scope notes

This is a backend product, not a library/CLI or PWA; pack/install and service-worker checks are not applicable. No sign-in is required, so no identity-provider integration is applicable. No product source was modified during verification.
