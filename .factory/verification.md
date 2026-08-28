# Independent product verification — FAIL

- Candidate: `85c381d3cf4cda5c68a05a8e0ec26f9b203391bd`
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-28 UTC
- Verdict: **FAIL — do not release**

The local candidate is polished and buildable, but the deployed core workflow is unreliable. The one-click demo fails, normal records alternate between available and missing, the paid checkout is dead, paid retention is not enforced by the backend, and PDF receipts corrupt international names. The claims manifest also does not prove several promises it makes.

## Release blockers

### Critical — production records are split across isolated replicas

The deployed backend uses container-local SQLite while multiple live instances serve requests. Fresh writes are visible only on the instance that received them.

- A cold click on **Try it with sample data** made `POST /api/demo` successfully, then `GET /api/share/<new-token>` returned 404. The page showed **The demo could not start** and logged the 404.
- Sixteen direct fresh demo attempts produced 201 for every create and 404 for every immediate read: **0/16 usable demos**.
- A normal quote alternated exactly between 404 and 200 over 12 share reads and 12 owner reads: `404,200,404,200,...`.
- Deleting that record required three attempts: `404, 404, 204`.
- Another live record returned 200 from its share route but 404 from both owner and delete routes.

This breaks demo, approval, management, export, deletion, and receipt continuity. It fails the mandatory one-click demo gate and the real job-to-be-done.

### High — Studio purchase is dead and its entitlement is bypassable

- The advertised `https://api.sociobot.in/api/v1/products/quote-approval-receipt/checkout` returns HTTP 404 with `{"error":"enabled factory product","status":404}`.
- The backend accepts `retention_days: 365` without a license or entitlement proof and returns a 365-day expiry. The only gate is forgeable browser state.
- The `@claim:studio-retention` test checks copy and the link string. It does not follow checkout, confirm price/product, verify a license, or prove server enforcement.

Users cannot buy the feature, while API callers can obtain it without paying.

### High — PDF receipts corrupt international identities

A local end-to-end record for `José Núñez` with title `Direção` retained correct JSON but generated this PDF text:

```text
Approver: Jos Nez  Direo
```

The PDF generator discards every non-ASCII character. This damages the central named approval artifact.

### High — claim coverage does not satisfy the claims contract

All five declared commands passed, but important promises are unlisted or not observably tested:

- Checkout/retention asserts copy and an `href`, not a working purchase or entitlement.
- PDF checks `%PDF-1.4` and a page name, not visible approver, timestamp, quote snapshot, or intact international text.
- The 30-day free and 365-day Studio retention behavior is not tested.
- README claims about unguessable links, `noindex`, salted network-address hashes, API rate limits, final single decisions, and 24-hour demo expiry have no claim entries.

The supplied claims contract states that an unlisted claim fails review.

## Other defects

### Medium

- Edge validation accepts currency `!!!`. Opening its approval link remains on **Loading the fixed quote…** and emits `Invalid currency code : !!!`.
- At 390 px with text at 200%, landing overflows by 122 px; `/new` by 34 px; `/privacy` by 10 px.
- Interactive targets below 44×44 px include the 145×25 wordmark and 67×25 Privacy, 48×25 Terms, and 211×25 factory links on mobile.
- Sensitive API responses lack `Cache-Control: no-store`. Hashed assets also lack cache policy and text compression. Lighthouse estimated 142 KiB cache-lifetime and 27 KiB compression savings.

### Low

- An unknown path renders the styled 404 but responds HTTP 200.
- `/manage/*` lacks response-level `X-Robots-Tag`; the SPA adds a robots meta tag after load.
- Startup works with only `PORT`, but the required configuration line is not printed because default tracing suppresses `info!`.

## Mandatory first-read gate

The cold landing screen itself passes:

- What: **Record who approved your quote**.
- Who: contractors whose clients approve through email or chat.
- First click: **Try it with sample data**, with an explanation of what loads.

The live action then fails, so the mandatory functional demo gate fails overall.

## Claims run first from the clean candidate

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-sandbox` | `npm test -- --grep @claim:demo-sandbox` | PASS |
| `pdf-receipt` | `npm test -- --grep @claim:pdf-receipt` | PASS |
| `record-control` | `npm test -- --grep @claim:record-control` | PASS |
| `first-party-only` | `npm test -- --grep @claim:first-party-only` | PASS |
| `studio-retention` | `npm test -- --grep @claim:studio-retention` | PASS |

Each selected one Playwright test after the build and two Rust tests. These run locally and do not override fresh live failures or incomplete assertions.

## Build and local gates

- `npm ci`: PASS; 26 packages, 0 vulnerabilities.
- `npm test`: PASS; 2 Rust and 9 Playwright tests.
- `npm run check`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `BUILD_SHA=85c381d3cf4cda5c68a05a8e0ec26f9b203391bd cargo build --release`: PASS.
- `npm audit`: PASS; 0 vulnerabilities.
- Exact frontend build: PASS; `dist/` produced.
- Container execution was unavailable because this image has no Docker/Podman. Inspection found a multi-stage, non-root Dockerfile with no `.git` dependency, a default build arg, `PORT=8080`, and `/data` persistence.

In an isolated directory with only `PORT` set, the release binary started and served. SQLite survived restart. Validation rejected 0/41 items, zero quantity, negative rate, tax over 100%, and short consent. Forty maximum-sized items were accepted. Concurrent final decisions returned 201 and 409. Export, owner rejection, PDF response, delete, and post-delete 404 worked locally.

## Deployment identity and live health

- `/health` returned candidate SHA `85c381d3cf4cda5c68a05a8e0ec26f9b203391bd`.
- Live and local SHA-256 hashes matched exactly:
  - HTML `dd6d34f02cf4757bee23b369e73ac8d58208cae86965472c36ade40d6cd52d8f`
  - JS `aaf0a9e626680b6b239debe276f62f71652815a302d5870c5bf8ff2d32b26083`
  - CSS `83dac4edd2274c19a32c8ffd426d1fc81346487c56261a0821dbb5911b235b81`
- 100 concurrent live health requests: 100 HTTP 200 in 393 ms; all reported the candidate SHA.

The deployment matches the candidate; this is not a stale-build failure.

## Rate limits

- Product GET: 50-request burst → 40×404, 10×429, `Retry-After: 1`.
- Product write: 20-request burst → 15×422, 5×429, `Retry-After: 1`.
- Sociobot license verify: 100-request burst → 30×200, 70×429, `Retry-After: 4`.
- `/health` is exempt, as allowed.

## Privacy and security

- Landing plus attempted demo made only same-origin requests; no analytics, ads, CDN fonts, or third-party scripts loaded.
- Demo wrote only `sessionStorage["demo:workspace"]`; local storage stayed empty.
- Responses set CSP, nosniff, no-referrer, DENY framing, and restrictive Permissions Policy.
- Approval and receipt documents set `X-Robots-Tag: noindex, nofollow, noarchive`.
- Invalid-license billing responses used `Cache-Control: no-store` and allowed the production origin through CORS.
- Sign-in, service worker/PWA, library, and CLI checks do not apply.

## Accessibility, mobile, and performance

- `/opt/fleet/lib/verify-url.sh`: PASS; 600 ms load, one H1, `lang=en`, main, complete image alt, no landing console errors.
- Independent axe checks on landing, privacy, terms, and styled 404: no serious/critical findings.
- Normal 1440 px and 390 px layouts: no overflow.
- Keyboard reached skip link, navigation, demo, real start, purchase, restore, and footer actions. Each had a visible 4 px focus outline; no trap found.
- Reduced motion produced zero active animations.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 1.1 s, LCP 1.7 s, TBT 110 ms, CLS 0, transfer 144 KiB.
- Bundle: JS 25,089 bytes raw / 8,525 gzip; CSS 14,777 / 4,066; hero WebP 104,994. Budgets pass.
- Visual review found a coherent cassette/zine design at desktop and 390 px.

## Required before re-verification

1. Use shared durable production storage, or one durable instance with a correctly mounted volume; retest the whole flow across requests.
2. Enable the Sociobot product and enforce license entitlement server-side for 365-day retention.
3. Generate Unicode-capable PDFs and test international text.
4. Expand claims/tests to prove every public and README promise.
5. Reject unsupported currencies and recover from formatting errors.
6. Fix 200% reflow and small targets; add private no-store and production cache/compression headers.
