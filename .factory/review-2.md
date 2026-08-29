# Adversarial first-read review 2 — FAIL

- Product: Quote Approval Receipt
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Revision reviewed: `f08adbf88078b184adacf6eedb081c9c234855a8`
- Viewports: fresh 390×844 mobile and 1440×900 desktop
- Verdict: **FAIL** — two minor findings remain. This review uses the required zero-findings threshold, so minor findings prevent PASS.

## First screen

The first screen passes the comprehension gate at both sizes.

- **What it does:** records who approved a quote and gives a timestamped PDF receipt.
- **For whom:** contractors whose clients approve in email or chat and later change expectations.
- **First action:** select **Try it with sample data**. Its adjacent text says that it loads a private sample quote and does not enter records.

The exact first-screen copy is `Record who approved your quote`, `For contractors whose client approves by email or chat, then changes expectations later.`, and `Try it with sample data`. All headline, audience, action, and three facts were visible without scrolling at 390px. The mobile screen was 390px wide with no horizontal overflow; the desktop screen was 1440px wide with no overflow.

## Findings

### F-2-1 — minor — leaving the demo does not discard its browser sandbox pointer

**Location:** the persistent demo-banner link **Start for real**; `frontend/src/main.ts`, `shell()` and `newQuote()`.

**Evidence:** the demo stores its workspace at `sessionStorage["demo:workspace"]`. The Start-for-real link only calls `navigate("/new")`; neither that route nor its click handler removes the key. `renderDemo(true)` removes it only when a visitor chooses **Reset demo**. The backend workspace also remains available until its documented 24-hour expiry. There is no explicit “keep this as my data” choice.

**Why this matters:** the sandbox is separate from real records, which is good, but the required demo lifecycle says that leaving demo mode discards demo data unless the visitor explicitly chooses to keep it. A visitor who selects Start for real has left the demo while the browser retains its pointer to the old sample workspace. The existing `@claim:demo-sandbox` test checks isolation and repeated reads, not exit cleanup.

**Concrete fix:** before navigating to `/new`, remove `demo:workspace` and invalidate/delete the ephemeral workspace server-side, or present one explicit choice to keep it. Add a regression test that enters `/?demo=1`, chooses **Start for real**, then verifies no `demo:` storage key remains and that the old workspace cannot be read.

### F-2-2 — minor — the 404 H1 uses a cassette metaphor instead of identifying the error

**Location:** live `/missing-review-route`; heading text: `This page is not on the record`.

**Why this matters:** an expired approval link is a recovery moment. In a screen-reader heading list, this phrase does not plainly say that the page was not found; it relies on the surrounding eyebrow `Page not found` and product lore. This conflicts with the plain-words requirement that headings make sense out of context and avoid metaphors.

**Concrete fix:** change the H1 to `This page was not found`. Keep the cassette illustration and the useful supporting text, `The link may be mistyped, expired, or deleted.` Add a route-copy assertion for the 404 H1.

## Copy audit

Word counts use the visible reader-facing unit, with hyphenated and symbol-separated values counted as one word. No landing or README unit exceeds 22 words. No banned marketing adjective or jargon requiring a rewrite was found. The two findings above are route/demo behavior and non-landing 404 copy, respectively.

### Landing page

| Words | Copy unit | Result |
| ---: | --- | --- |
| 6 | Record who approved your quote | pass |
| 13 | For contractors whose client approves by email or chat, then changes expectations later. | pass |
| 5 | Try it with sample data | pass — result-naming action |
| 5 | Loads a private sample quote. | pass |
| 4 | Nothing enters your records. | pass — demo isolation claim |
| 4 | Free: 30-day link retention | pass — retention claim |
| 6 | Private: delete or export each record | pass — record-control claim |
| 6 | Clear: PDF receipt after a decision | pass — PDF claim |
| 3 | Fix the quote. | pass |
| 5 | Name the decision maker. | pass |
| 3 | Keep the receipt. | pass |
| 3 | Example approval receipt | pass |
| 8 | Turn an existing quote into one clear link | pass |
| 10 | Paste the agreed scope and totals, then make an approval link. | pass |
| 4 | Make an approval link | pass — result-naming action |
| 1 | Approved | pass |
| 2 | Quote NS-2048 | pass |
| 2 | Juniper Market | pass |
| 4 | Mara Chen · Brand director | pass |
| 6 | 18 Aug 2026 · 14:32 UTC | pass |
| 2 | proof 92dc…41a | pass |
| 4 | How approval links work | pass |
| 6 | How the record gets made | pass |
| 4 | 1. Enter quote details | pass |
| 7 | Enter the quote details you already agreed. | pass |
| 4 | 2. Send the link | pass |
| 8 | Share the private link with the client contact. | pass |
| 4 | 3. Keep the receipt | pass |
| 7 | Download the named decision and quote snapshot. | pass |
| 4 | What this record does | pass |
| 5 | Record a quote decision | pass |
| 9 | Use this tool to record a quote decision. | pass |
| 9 | It records the quote, decision maker, decision, and time. | pass |
| 9 | We store the approver details needed for that record. | pass |
| 5 | No tracking scripts run here. | pass — first-party-only claim |
| 6 | For teams that keep records longer | pass |
| 5 | Keep receipts for one year | pass — Studio/retention claim |
| 5 | Studio checkout is not available. | pass — Studio offer claim |
| 7 | Free links keep records for 30 days. | pass — retention claim |
| 4 | Enter a Studio license | pass — result-naming action |
| 7 | Named quote decisions, kept with the quote. | pass — product one-liner |

Navigation labels, product names, dates, and sample-record fields were also checked. Their terms are consistent: **quote**, **approval link**, **decision maker**, **decision**, **receipt**, **Studio license**, and **demo**.

### README

| Words | Sentence or heading | Result |
| ---: | --- | --- |
| 3 | Quote Approval Receipt | pass |
| 11 | Capture who approved a fixed quote and issue a timestamped receipt. | pass — PDF claim |
| 16 | Quote Approval Receipt is for small agencies and contractors who receive approvals in email or chat. | pass |
| 9 | It creates a private link for an existing quote. | pass — private-links claim |
| 15 | The client names the decision maker, approves or requests changes, and receives a PDF record. | pass — PDF claim |
| 3 | Try the sample | pass |
| 8 | Open `/?demo=1` after starting the service, or visit: | pass |
| 11 | The demo creates a random workspace that expires after 24 hours. | pass — demo/retention claims |
| 7 | It stays separate from real quote records. | pass — demo-sandbox claim |
| 8 | See `.factory/demo.md` for the sandbox contract. | pass |
| 2 | Run locally | pass |
| 12 | Requirements: Node.js 22+, npm, Rust 1.88+, and SQLite build support. | pass |
| 3 | Open `http://localhost:8080`. | pass |
| 14 | The server creates `data/quotes.sqlite3` and a random privacy salt on first boot. | pass — runtime-contract claim |
| 7 | No secret or configuration variable is required. | pass — runtime-contract claim |
| 21 | For frontend work with live reload, run the backend on port 8080 and then run `npm run dev` in another shell. | pass |
| 3 | Test and build | pass |
| 21 | `npm test` builds the frontend, runs Rust tests, starts the service, and runs the Playwright claim, mobile, rate-limit, and accessibility checks. | pass |
| 5 | The frontend output is `dist/`. | pass |
| 7 | The container listens on `PORT` (default `8080`). | pass — runtime-contract claim |
| 20 | It starts with no required configuration, creates a random privacy salt on first boot, and `/health` reports the build SHA. | pass — runtime-contract claim |
| 16 | Set `DATA_DIR` or `DATABASE_URL` only when you need to override the persisted SQLite location. | pass |
| 3 | Data and security | pass |
| 10 | Each real quote gets independent random approval and owner tokens. | pass — private-links claim |
| 12 | The owner token stays in that browser and authorizes export or deletion. | pass — record-control claim |
| 11 | Private pages and API responses use `noindex` and `no-store` response policies. | pass — private-links claim |
| 7 | Each fixed quote accepts one final decision. | pass — single-decision claim |
| 12 | API write bursts return `429` with `Retry-After` when they exceed the limit. | pass — rate-limits claim |
| 7 | Free links retain records for 30 days. | pass — retention-policy claim |
| 14 | A 365-day request succeeds only after the backend verifies a Studio license with Sociobot. | pass — retention-policy claim |
| 8 | Studio checkout appears only when Sociobot reports availability. | pass — studio-offer claim |
| 9 | License holders can enter a license on any device. | pass — studio-offer claim |
| 1 | Deploy | pass |
| 4 | Build the root `Dockerfile`. | pass |
| 14 | The image runs as the non-root `app` user and needs only `PORT` to start. | pass — runtime-contract claim |
| 7 | For the factory Container Apps release, use one replica. | pass |
| 11 | Mount its dedicated Azure Files share at `/durable` and set `DURABLE_DATA_DIR=/durable`. | pass |
| 19 | The service writes a durable snapshot after each committed change, so a replacement process restores records before serving traffic. | pass — durable-snapshot claim |
| 10 | Durable storage is an optional deployment override for local use. | pass |
| 12 | The factory owns deployment, DNS, billing registration, and the production build SHA. | pass |
| 1 | License | pass |
| 1 | MIT. | pass |
| 2 | See LICENSE. | pass |

## Demo, claims, privacy, and structure checks

- The primary landing action opened `/?demo=1` directly into the realistic Northstar Studio quote `NS-2048` for Juniper Market. The first product screen showed `Half-day product shoot`, line items, totals, a named decision-maker form, and a final-decision warning.
- The persistent banner read `Demo — sample data, nothing is saved to your records`; **Reset demo** generated a new isolated sample pointer. During demo use, local storage was empty and session storage contained only `demo:workspace`. This confirms no demo operation wrote a real owner record. F-2-1 remains because the exit path retains that isolated pointer.
- Landing and successful demo browser request logs contained only `quote-approval-receipt.sociobot.in` requests (HTML, same-origin assets, `/api/studio`, `/api/demo`, and `/api/share/*`). No third-party scripts, fonts, analytics, advertising, billing, or AI request was observed.
- The landing/README claim-like statements above each map to a `claims.json` entry. No unlisted landing or README claim was found. The manifest has 13 entries, including the standalone runtime and durable-state contracts.

### Claim commands from a fresh clone

Fresh clone: `/tmp/qar-review-clean.XLh8Hj` at `f08adbf88078b184adacf6eedb081c9c234855a8`; `npm ci` completed with zero vulnerabilities. Every exact manifest command passed:

| Claim | Exact command | Result |
| --- | --- | --- |
| demo-sandbox | `npm test -- --grep @claim:demo-sandbox` | PASS |
| quote-snapshot | `npm test -- --grep @claim:quote-snapshot` | PASS |
| pdf-receipt | `npm test -- --grep @claim:pdf-receipt` | PASS |
| record-control | `npm test -- --grep @claim:record-control` | PASS |
| first-party-only | `npm test -- --grep @claim:first-party-only` | PASS |
| retention-policy | `npm test -- --grep @claim:retention-policy` | PASS |
| studio-offer | `npm test -- --grep @claim:studio-offer` | PASS |
| private-links | `npm test -- --grep @claim:private-links` | PASS |
| single-decision | `npm test -- --grep @claim:single-decision` | PASS |
| network-address-privacy | `npm test -- --grep @claim:network-address-privacy` | PASS |
| rate-limits | `npm test -- --grep @claim:rate-limits` | PASS |
| runtime-contract | `npm run test:runtime-contract` | PASS |
| durable-snapshot | `npm run test:durable-snapshot` | PASS |

- The public desktop and 390px pages have one H1, `main`, `lang=en`, accessible skip navigation, visible focus, an original cassette/zine visual system, and no generic SaaS-template layout. The supplied 404 is styled, returns HTTP 404, and gives a Return-home action; F-2-2 is its only copy defect.
- Every crawled public/internal link returned 200: `/`, `/demo`, `/new`, `/privacy`, `/terms`, `robots.txt`, `sitemap.xml`, favicon, apple touch icon, social card, and the Param Factory link. The deliberately missing path returned HTTP 404.
- Titles, descriptions, canonical URLs, Open Graph/Twitter title/description, favicon, and apple-touch icon were present on landing, demo, builder, privacy, terms, and 404. Back navigation and route-to-H1 focus are covered by the browser suite and confirmed by source routing.
- The brief does not imply a useful AI step: the core job is deterministic preservation of an approval decision. Existing PDF and JSON export controls cover the obvious output need. No embedded provider key or decorative AI feature was found.

## Earlier findings rechecked

| Earlier finding | Current result |
| --- | --- |
| State split breaks fresh demo/read flow (verification, verification-2, verification-3, F-1-1) | **Fixed.** 20 independent live browser launches each had `POST /api/demo` 201, immediate share `GET` 200, and visible `Half-day product shoot` (20/20). |
| Sender cannot recover receipt/PDF (verification-2) | **Fixed in code and suite.** `@regression:owner-receipt-link` creates, decides, opens the owner page in a fresh context, and downloads `approval-receipt.pdf`. |
| Unlisted runtime/durable and public claims (verification-2, review-1 F-1-3) | **Fixed.** All material landing/README statements map to the current 13-entry claims manifest; the copy audit above found no remainder. |
| Pinned Rust image (verification-2) | **Fixed.** Dockerfile uses `rust:1-slim`; runtime contract inspects and starts the non-root container contract. |
| Mobile 404 overflow (verification-4, review-1 F-1-2) | **Fixed.** Live 390px measurement was 390/390; the browser regression covers it. |
| Cassette labels and unclear license control (review-1 F-1-4) | **Fixed on landing.** The current labels name the preview, approval-link steps, and Studio license action. |
| Route social metadata (review-1 F-1-5) | **Fixed.** Live route checks found route-specific titles and descriptions for Open Graph and Twitter. |
| Unicode PDF, retention authorization, invalid currency, targets/reflow, private headers, 404 status, and cache policies (earlier verification) | **Fixed in current source/tests.** The relevant tagged tests and regression checks are present in `tests/product.spec.ts`; live checks above confirm the browser-facing parts. |

## What would make this perfect

Discard the demo workspace when the visitor selects **Start for real**, and make the 404 H1 say plainly that the page was not found. Add the two focused regressions, then rerun this full fresh-context review. At that point the product has a credible PASS path.
