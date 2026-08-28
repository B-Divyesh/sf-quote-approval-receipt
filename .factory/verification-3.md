# Independent product verification 3 — FAIL

- Work order: `quote-approval-receipt-verify-3`
- Candidate commit: `df5ec08b4abf5767daa20f6956b68632acad75c8`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-28 UTC
- Verdict: **FAIL — do not release**

The candidate is deployed (its live health identity and all built frontend assets exactly match the checkout), and the complete local suite passes. The live deployment nevertheless fails the required one-click demo and cannot reliably retain or retrieve quote records across fresh requests. This reproduces the previously reported deployment-only state split from new evidence.

## Release blocker

### Critical — live state is split between serving instances

The web backend uses instance-local state for at least some requests. A record created successfully on one live request is frequently missing from the next fresh request.

- A new browser demo was successfully created, approved, and downloaded as a PDF in one context. Reading that same existing demo approval link in 20 new browser contexts gave **7× 200** and **13× 404**. The 404 pages rendered `This page could not be loaded`.
- The required one-click test was then repeated at the real entry point: 20 fresh browser contexts each opened `/demo`. All 20 `POST /api/demo` calls returned **201**, but only **4/20** follow-up `GET /api/share/<new-token>` calls returned 200 and rendered `Review and decide on this quote`. The other **16/20** returned **404** and rendered `The demo could not start`.
- This breaks the mandatory sample demo as well as normal quote, owner management, approval, receipt, export, and deletion continuity whenever a later request reaches a different instance.

This is a release blocker under the demo-sandbox and end-to-end requirements. The previous repair handoff says the deployed app is one replica with `/durable`; fresh live evidence proves that configuration is not effective for this candidate's public request path.

## Mandatory first-read gate

**PASS for copy; FAIL overall because the sample action is unreliable.** A cold desktop browser page plainly stated:

- What it does: `Record who approved your quote`.
- Who it is for: `For contractors whose client approves by email or chat, then changes expectations later.`
- First action: `Try it with sample data`, immediately explained as loading a private sample quote without entering real records.

The first screen also shows the three required facts (30-day free retention, export/delete control, and a PDF receipt). The first action failed in 16 of 20 fresh live contexts as documented above, so it does not satisfy the usable-demo acceptance gate.

## Claims gate — run first from the clean checkout

`.factory/claims.json` exists and contains 11 claims. After `npm ci` (30 packages added; zero vulnerabilities), every listed command was executed verbatim. All passed locally; none can override the live failure above.

| Claim | Exact command | Local result |
| --- | --- | --- |
| `demo-sandbox` | `npm test -- --grep @claim:demo-sandbox` | PASS — isolated sample and repeated read |
| `pdf-receipt` | `npm test -- --grep @claim:pdf-receipt` | PASS — PDF snapshot, time, Unicode identity and consent |
| `record-control` | `npm test -- --grep @claim:record-control` | PASS — export, deletion, then inaccessible share |
| `first-party-only` | `npm test -- --grep @claim:first-party-only` | PASS — same-origin landing/demo resources |
| `retention-policy` | `npm test -- --grep @claim:retention-policy` | PASS — exact free/demo expiry and forged Studio rejection |
| `private-links` | `npm test -- --grep @claim:private-links` | PASS — independent tokens and private headers |
| `single-decision` | `npm test -- --grep @claim:single-decision` | PASS — concurrent 201/409 result |
| `network-address-privacy` | `npm test -- --grep @claim:network-address-privacy` | PASS — salted address not exported or printed |
| `rate-limits` | `npm test -- --grep @claim:rate-limits` | PASS — local 429 with retry guidance |
| `runtime-contract` | `npm run test:runtime-contract` | PASS — only `PORT`, generated salt, build identity |
| `durable-snapshot` | `npm run test:durable-snapshot` | PASS — replacement process restores durable state |

The live `demo-sandbox` promise is false despite its local test passing.

## Local quality gates

- `npm run check`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm audit --audit-level=high`: PASS; zero vulnerabilities.
- `npm run build`: PASS; produced `dist/`.
- `npm test`: PASS; 4 Rust tests and 16 Playwright tests.
- Production frontend bundle: JS 26,009 bytes raw / 8,860 gzip; CSS 15,456 bytes raw / 4,190 gzip. Both are within the stated budgets; the 104,994-byte WebP hero is also within budget.
- The required runtime and durable-state checks pass locally. Docker/Podman is unavailable in this verifier, so an image build could not be run; source inspection found a multi-stage, non-root Dockerfile with `rust:1-slim`, default `BUILD_SHA`, no `.git` dependency, and `PORT=8080`.

Representative live UI flow on a single successful route did work: invalid quantity `0` produced the native `Value must be greater than or equal to 0.01.` validation message; corrected two-item/taxed quote creation, approval by `José Núñez` / `Direção`, PDF download (200 `application/pdf`), owner receipt/PDF actions, export (200), and deletion (204) all succeeded. That success is not reliable across fresh routes because of the critical fault.

## Live deployment, privacy, and security evidence

- `GET /health` returned `{"build_sha":"df5ec08b4abf5767daa20f6956b68632acad75c8","status":"ok"}`. A 100-request health concurrency check returned 100×200, all with that SHA.
- Live and local SHA-256 hashes are identical: HTML `0469701228d552269d765330973cffbdce6a0e7bcd7b80e7cb5304de51d91e1e`; JS `f77d68b639179cc8fc24ea13e64201f437e50d63cf318df670bc73a74108cf0b`; CSS `f04147a4644f7e54722cb9a9aeb90da674963d0fdc0137eb23116cb301df15f9`.
- A Playwright request log through landing, demo, and decision recorded only the product origin; no analytics, ads, CDN fonts, or third-party runtime script loaded. Demo local storage was empty; session storage held only demo/receipt pointers.
- Live headers included CSP with self-only scripts/styles, `nosniff`, `no-referrer`, frame denial, and restrictive permissions. Hashed JS/CSS use `Cache-Control: public, max-age=31536000, immutable`. Private API responses use `X-Robots-Tag: noindex, nofollow, noarchive` and `Cache-Control: no-store` (verified on a live private 404 response).
- `verify-url.sh` passed: 643 ms cold load, valid title and `lang=en`, one H1, main landmark, image alt text, and zero console errors.

## Accessibility, responsive, and interaction checks

- Independent Playwright axe scans found zero serious/critical issues on `/`, `/new`, `/privacy`, `/terms`, and styled `/missing-page`.
- Keyboard starts at the skip link; its visible focus is a 4 px yellow outline, and Enter moves focus to `<main>`.
- At 390×844 the landing page had no horizontal overflow (`scrollWidth=390`, `innerWidth=390`).
- With `prefers-reduced-motion: reduce`, the page reported zero running animations.

## Rate limiting

The live service eventually rate limits API writes and supplies retry guidance, but the observed allowance exposes the same multi-instance boundary: one client sent 60 concurrent invalid `POST /api/quotes` requests and received **30×422 then 30×429**, all 429s with `Retry-After: 1`. The source/local allowance is 15 writes per second, while the public deployment accepted 30 invalid writes before limiting, consistent with two independently limited serving instances. This is secondary evidence for the critical split-state finding.

PWA/offline-update, sign-in/Entra, library/CLI consumer, and AI checks do not apply: this product exposes none of those features.

## Required before re-verification

1. Route every public request for a record to one shared durable database, or run exactly one correctly durable backend instance. Do not rely on per-instance SQLite state behind a load balancer.
2. Repeat the live test from fresh browser contexts: at least 20 `/demo` visits must each be `POST 201` followed by share `GET 200`; then read the same approval/owner/receipt state through fresh contexts.
3. Recheck rate limiting after fixing deployment topology; one client should observe the documented per-client write allowance, not an aggregate multiplied by replica count.
