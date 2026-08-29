# Independent product verification 4 — PASS

- Work order: `quote-approval-receipt-verify-4`
- Candidate commit: `bb2b75b517127eb2924ecbf37b1e5a1b4f2232d3`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Verdict: **PASS — candidate is releasable**

This is fresh verification against the deployed candidate. It specifically repeats the prior deployment-only state-split regression; it did not recur.

## Mandatory first-read and demo gate

Cold, state-free Chromium load of `/` returned 200 with title `Quote Approval Receipt — Record quote decisions` and one H1, `Record who approved your quote`.

- **What it does:** records who approved a quote and gives a receipt.
- **For whom:** contractors whose clients approve by email or chat and later change expectations.
- **What to click first:** the visible one-click `Try it with sample data` link, explained beside the link as loading a private sample quote without entering real records.

The landing screen also gives the required three facts: 30-day free retention, per-record export/deletion, and PDF receipt after a decision. This passes the plain-words and one-click-demo gates.

The earlier live failure was independently rechecked with 20 fresh browser contexts. Each `/demo` visit had `POST /api/demo` **201**, its own `GET /api/share/<token>` **200**, and displayed `Half-day product shoot`: **20/20 successful**, with no state split.

## Claims gate — run first from the clean checkout

`.factory/claims.json` exists with 11 claims. After `npm ci` (30 packages added, audit clean), every exact command in the manifest was run. The resulting Playwright last-run record is `{"status":"passed","failedTests":[]}`.

| Claim | Required command | Result |
| --- | --- | --- |
| demo-sandbox | `npm test -- --grep @claim:demo-sandbox` | PASS |
| pdf-receipt | `npm test -- --grep @claim:pdf-receipt` | PASS |
| record-control | `npm test -- --grep @claim:record-control` | PASS |
| first-party-only | `npm test -- --grep @claim:first-party-only` | PASS |
| retention-policy | `npm test -- --grep @claim:retention-policy` | PASS |
| private-links | `npm test -- --grep @claim:private-links` | PASS |
| single-decision | `npm test -- --grep @claim:single-decision` | PASS |
| network-address-privacy | `npm test -- --grep @claim:network-address-privacy` | PASS |
| rate-limits | `npm test -- --grep @claim:rate-limits` | PASS |
| runtime-contract | `npm run test:runtime-contract` | PASS |
| durable-snapshot | `npm run test:durable-snapshot` | PASS |

## Local build and checks

- `npm run build`: PASS; produced `dist/`.
- `npm test`: PASS — 4 Rust tests and 16 Playwright tests. It includes end-to-end quote creation/approval, Unicode PDF receipt extraction, export/delete, concurrent final-decision conflict, invalid currency recovery, response policies, keyboard navigation, 390px/200% text reflow, reduced motion, and axe checks.
- `npm run check`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- Production assets: JS 26,009 bytes raw / 8,860 bytes gzip; CSS 15,456 bytes raw / 4,190 bytes gzip. Both are within budget. The hero WebP is 104,994 bytes.
- The exact container build could not be executed because Docker is not installed in this verifier. The passing runtime contract inspects the Dockerfile and starts the release binary using only `PORT`; source inspection confirms multi-stage build, `rust:1-slim`, default `BUILD_SHA`, non-root `app`, and port 8080.

## End-to-end product evidence

On the live sample, blank submission exposed native required-choice/consent validation. Correcting it with `Samira Patel`, `Operations director`, a note, `Request changes`, and consent produced the timestamped receipt page with the fixed `NS-2048` snapshot, named decision maker, note, consent, SHA-256 snapshot hash, and downloadable PDF. No page or JavaScript console errors occurred during the normal flow.

The full automated suite also verified: an invalid currency yields 422 with actionable wording; a 365-day retention request without a verified Studio license yields 403; two concurrent decisions yield exactly 201 and 409; an owner can export then delete a record, after which its approval route returns 404.

Live private-route verification created and removed a QA-only record: create 201, approval page 200 with `Cache-Control: no-store` and `X-Robots-Tag: noindex, nofollow, noarchive`, API share response with the same privacy policy, delete 204, then share 404.

## Deployment, privacy, headers, and backend checks

- `GET /health` returned `{"build_sha":"bb2b75b517127eb2924ecbf37b1e5a1b4f2232d3","status":"ok"}`. A 100-request concurrent health check returned 100/100 HTTP 200 with only that SHA.
- A browser request log covering cold landing, demo load, and a recorded decision contained only `quote-approval-receipt.sociobot.in` requests. There were no analytics, advertising, third-party scripts, fonts, or CDN requests.
- Browser-visible landing headers include self-only script/style CSP, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `X-Frame-Options: DENY`, restrictive permissions policy, and `frame-ancestors 'none'`. Hashed JS is `public, max-age=31536000, immutable`.
- A single client (fixed `X-Forwarded-For`) received 15 successful `POST /api/demo` responses, then requests 16–20 were **429** with `Retry-After: 1`. This confirms the documented write allowance and retry guidance live.
- The runtime and durable-snapshot claims pass locally; the 20 fresh live demo contexts above provide fresh persistence-boundary evidence. The product has no sign-in, PWA/service worker, public package, CLI, or AI feature, so Entra, offline-update, consumer-install, and live-model checks do not apply.

## Accessibility, responsive, and visual checks

Independent live Playwright + axe scans at desktop and 390px found zero serious/critical violations on `/`, `/new`, `/demo`, `/privacy`, `/terms`, and the styled 404. Each has one H1; normal routes had no JavaScript/page errors or horizontal overflow. Keyboard smoke testing confirmed the skip link receives first focus, Enter moves focus to `main`, and sample-demo navigation moves focus to the route H1. The product test suite also confirms 200% text reflow and zero running animations under `prefers-reduced-motion: reduce`.

The live visual treatment matches the documented cassette-era zine thesis: original cassette/stamp illustration, warm-paper/ink/teal/red palette, self-hosted system stacks, and no third-party asset loading.

## Defects

- **P3 — 404 mobile overflow:** At 390px `/missing-page` has `document.documentElement.scrollWidth` 392px versus viewport 390px, producing a 2px horizontal overflow. Core landing, builder, demo, policy, and receipt flows have no overflow. This is not release-blocking but should be corrected in the 404 layout.

No P0, P1, or P2 findings. The previous deployment-only state-split failure is resolved for this candidate.
