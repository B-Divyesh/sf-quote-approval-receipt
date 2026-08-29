# Adversarial first-read review 4 — FAIL

- Product: Quote Approval Receipt
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Source reviewed: clean clone of `5f38e5c220c03db9a570ebd82aea6769b7209d74`
- Live build: `8f22bab8d72a2c9ea7bb6a19c44af86083fa589a`; `/health` reports `durable_snapshot: true`
- Viewports: fresh 390×844 mobile and 1440×900 desktop
- Verdict: **FAIL** — one minor but material unlisted claim remains. The required standard is zero findings.

## First screen

The first screen passes the cold-read gate at both widths, before scrolling.

- **What it does:** records who approved a quote and creates a PDF receipt.
- **For whom:** contractors whose clients approve in email or chat and may later change expectations.
- **First click:** **Try it with sample data**. The adjacent result says `Loads a private sample quote. Nothing enters your records.`

The exact headline is `Record who approved your quote`. The audience sentence is `For contractors whose client approves by email or chat, then changes expectations later.` All three facts, the action, and its result note were visible above the fold at 390px. Neither desktop nor mobile overflowed.

## Finding

### F-4-1 — minor — README makes an unlisted, untested live-global rate-limit claim

**Location / exact quote:** README, Deploy: `This one-replica contract also makes the 15-write-per-second limit global for the live service.`

**Why this is a finding:** this is a quantitative, production-topology claim that an operator may rely on. `.factory/claims.json` has `rate-limits`, but its declared promise is only `API write bursts are rate limited with retry guidance.` Its exact test starts one local server and sends 16 sequential demo writes. It does not establish that the 15-per-second allowance is global for the deployed service. `scripts/live-rate-limit-check.mjs` is useful supplementary evidence, but it is not the claim-manifest test and is not run by any command in `claims.json`.

**Concrete fix:** delete the quoted sentence; the preceding README sentence already states the useful, tested behaviour: `API write bursts return 429 with Retry-After when they exceed the limit.` If the global production guarantee must remain public, add a separate claim with an exact live-topology test that proves one serving replica and asserts the shared 15-write allowance from independently fresh clients.

## Copy audit

Counts use visible whitespace-separated words; headings, labels, controls, sample fields, and footer text are included because they are read out of context. Hyphenated and symbol-separated values count as one word. No unit exceeds 22 words. No jargon, banned marketing adjective, unclear heading, inconsistent product term, metaphor heading, or non-result-naming button was found. `F-4-1` is a claims-contract flag, not a plain-words failure.

### Landing page

| Words | Copy unit | Result |
| ---: | --- | --- |
| 4 | Skip to main content | pass |
| 3 | Quote Approval Receipt | pass — wordmark |
| 1 | Demo | pass |
| 3 | Make a link | pass — navigates to the link builder |
| 1 | Privacy | pass |
| 6 | Record who approved your quote | pass |
| 13 | For contractors whose client approves by email or chat, then changes expectations later. | pass |
| 5 | Try it with sample data | pass — result-naming action |
| 5 | Loads a private sample quote. | pass |
| 4 | Nothing enters your records. | pass — `demo-sandbox` |
| 4 | Free: 30-day link retention | pass — `retention-policy` |
| 6 | Private: delete or export each record | pass — `record-control` |
| 5 | Receipt: PDF after a decision | pass — `pdf-receipt` |
| 3 | Fix the quote. | pass |
| 4 | Name the decision maker. | pass |
| 3 | Keep the receipt. | pass |
| 3 | Example approval receipt | pass |
| 8 | Turn an existing quote into an approval link | pass |
| 10 | Paste the agreed scope and totals, then make an approval link. | pass |
| 4 | Make an approval link | pass — result-naming action |
| 1 | Approved | pass — sample status |
| 2 | Quote NS-2048 | pass — sample identifier |
| 2 | Juniper Market | pass — sample client |
| 4 | Mara Chen · Brand director | pass — sample decision maker |
| 5 | 18 Aug 2026 · 14:32 UTC | pass — sample time |
| 2 | proof 92dc…41a | pass — sample proof label |
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
| 9 | We store the approver details needed for that record. | pass — covered by the named receipt/record flow |
| 5 | No tracking scripts run here. | pass — `first-party-only` |
| 6 | For teams that keep records longer | pass |
| 5 | Keep receipts for one year | pass — `retention-policy` / `studio-offer` |
| 5 | Studio checkout is not available. | pass — current recorded state, `studio-offer` |
| 7 | Free links keep records for 30 days. | pass — `retention-policy` |
| 4 | Enter a Studio license | pass — result-naming action |
| 7 | Named quote decisions, kept with the quote. | pass — concise product one-liner, substantiated by the record and retention claims |
| 1 | Privacy | pass |
| 1 | Terms | pass |
| 4 | Built by Param Factory | pass — attribution |
| 2 | external site | pass — destination notice |
| 2 | v1.3 · 8f22bab | pass — build label |

Terminology remains consistent: **quote**, **quote snapshot**, **approval link**, **decision maker**, **decision**, **receipt**, **Studio license**, and **demo**.

### README

| Words | Sentence or heading | Result |
| ---: | --- | --- |
| 3 | Quote Approval Receipt | pass |
| 11 | Capture who approved a fixed quote and issue a timestamped receipt. | pass — `pdf-receipt` |
| 16 | Quote Approval Receipt is for small agencies and contractors who receive approvals in email or chat. | pass |
| 9 | It creates a private link for an existing quote. | pass — `private-links` |
| 15 | The client names the decision maker, approves or requests changes, and receives a PDF receipt. | pass — `pdf-receipt` |
| 3 | Try the sample | pass |
| 8 | Open `/?demo=1` after starting the service, or visit: | pass |
| 11 | The demo creates a random workspace that expires after 24 hours. | pass — `demo-sandbox` / `retention-policy` |
| 7 | It stays separate from real quote records. | pass — `demo-sandbox` |
| 9 | Resetting or leaving the demo deletes its current workspace. | pass — `demo-sandbox` |
| 8 | See `.factory/demo.md` for the sandbox contract. | pass |
| 2 | Run locally | pass |
| 12 | Requirements: Node.js 22+, npm, Rust 1.88+, and SQLite build support. | pass |
| 3 | Open `http://localhost:8080`. | pass |
| 14 | The server creates `data/quotes.sqlite3` and a random privacy salt on first boot. | pass — `runtime-contract` |
| 7 | No secret or configuration variable is required. | pass — `runtime-contract` |
| 21 | For frontend work with live reload, run the backend on port 8080 and then run `npm run dev` in another shell. | pass |
| 3 | Test and build | pass |
| 21 | `npm test` builds the frontend, runs Rust tests, starts the service, and runs the Playwright claim, mobile, rate-limit, and accessibility checks. | pass — observed |
| 5 | The frontend output is `dist/`. | pass — observed |
| 7 | The container listens on `PORT` (default `8080`). | pass — `runtime-contract` |
| 20 | It starts with no required configuration, creates a random privacy salt on first boot, and `/health` reports the build SHA. | pass — `runtime-contract` |
| 16 | Set `DATA_DIR` or `DATABASE_URL` only when you need to override the persisted SQLite location. | pass |
| 3 | Data and security | pass |
| 10 | Each real quote gets independent random approval and owner tokens. | pass — `private-links` |
| 12 | The owner token stays in that browser and authorizes export or deletion. | pass — `record-control` |
| 11 | Private pages and API responses use `noindex` and `no-store` response policies. | pass — `private-links` |
| 7 | Each fixed quote accepts one final decision. | pass — `single-decision` |
| 12 | API write bursts return `429` with `Retry-After` when they exceed the limit. | pass — `rate-limits` |
| 7 | Free links retain records for 30 days. | pass — `retention-policy` |
| 14 | A 365-day request succeeds only after the backend verifies a Studio license with Sociobot. | pass — `retention-policy` |
| 13 | A $29 Studio checkout on Sociobot appears only when the product is available. | pass — `studio-offer` |
| 6 | License holders can enter a license. | pass — `studio-offer` |
| 1 | Deploy | pass |
| 4 | Build the root `Dockerfile`. | pass |
| 14 | The image runs as the non-root `app` user and needs only `PORT` to start. | pass — `runtime-contract` |
| 20 | The factory Container Apps release is a strict one-replica service: its dedicated Azure Files share is mounted at `/durable` and `DURABLE_DATA_DIR=/durable`. | pass — `durable-snapshot` manifest assertion |
| 19 | The service writes a durable snapshot after each committed change, so a replacement process restores records before serving traffic. | pass — `durable-snapshot` |
| 13 | This one-replica contract also makes the 15-write-per-second limit global for the live service. | **flag — F-4-1** |
| 10 | Durable storage is an optional deployment override for local use. | pass |
| 12 | The factory owns deployment, DNS, billing registration, and the production build SHA. | pass — ownership statement |
| 1 | License | pass |
| 1 | MIT. | pass |
| 2 | See LICENSE. | pass |

## Demo, sandbox, privacy, and end-to-end checks

- Fresh mobile `/?demo=1` immediately showed the Northstar Studio quote `NS-2048` for Juniper Market, the `Half-day product shoot` and edited-image line items, totals, prefilled sample decision-maker fields, consent, and final-decision warning. It is product use, not a mock landing state.
- The persistent banner was exactly `Demo — sample data, nothing is saved to your records`, with **Reset demo** and **Start for real**. Reset generated a new `demo:workspace`; Start for real removed the session key, deleted its share endpoint (404), and opened the empty quote builder. Browser local storage remained empty throughout.
- Independent cross-client API evidence was 20/20 usable cycles: `POST /api/demo` 201, next-client `GET /api/share/<token>` 200 containing `Half-day product shoot`, `DELETE /api/demo/<workspace>` 204, then share 404. This confirms the formerly blocking cross-request state split is not present in this live release.
- A Playwright request log covering landing, demo creation, reset, and exit contained only `https://quote-approval-receipt.sociobot.in`. No analytics, advertising, third-party runtime script, font, billing, or AI request loaded. Demo mode stored only the `demo:workspace` session key and never created an owner key.
- The brief implies deterministic recording, not generated content. PDF/JSON output is present, so no missing AI step, decorative AI feature, provider key, or omitted import/export expectation was found.

## Claims gate

From a clean clone at `5f38e5c220c03db9a570ebd82aea6769b7209d74`, `npm ci` installed 30 packages with zero vulnerabilities. Each literal command declared by `.factory/claims.json` was run individually. All passed.

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
| rate-limits | `npm test -- --grep @claim:rate-limits` | PASS — insufficient for F-4-1's extra global assertion |
| runtime-contract | `npm run test:runtime-contract` | PASS |
| durable-snapshot | `npm run test:durable-snapshot` | PASS |

`F-4-1` is the only landing/README claim-like statement not adequately covered by the manifest. There is no offline claim to test.

## Structure, accessibility, and visual identity

- `/`, `/demo`, `/new`, `/privacy`, and `/terms` returned 200. A deliberate missing route returned a designed HTTP 404 with `This page was not found` and a Return-home link.
- Each checked route has one H1, `<main>`, `lang=en`, route-specific title, description, canonical, Open Graph/Twitter title and description, favicon, and social image. The title pattern is plain and under 60 characters.
- Landing, demo, builder, privacy, terms, 404, both icon files, both WebP files, `robots.txt`, `sitemap.xml`, and the Param Factory link all resolved. Explicit `mailto:` links were not treated as HTTP links.
- The skip link is first in keyboard order. SPA navigation and Back moved focus to the destination H1. Mobile and desktop had no layout overflow; normal routes generated no page or console errors. (Chromium reports the expected failed-resource message for the deliberate document 404, so it was excluded from the normal-route console check.)
- The cassette/zine system is visibly distinct: warm paper, black keylines, teal/red spot colour, torn edges, hard print shadows, and original cassette/quote art. It follows the common information order without resembling a generic SaaS template.

## Earlier findings rechecked

| Earlier finding | Current live/code confirmation |
| --- | --- |
| F-1-1; V-1/V-2/V-3 critical; F-3-1 — split state broke demo/recipient flow | **Fixed.** `/health` reports durable state; 20 independently addressed create/read/delete cycles were all 201/200/204/404. |
| F-1-2; V-4 P3 — 404 overflow | **Fixed.** 390px `scrollWidth` equals 390 on the live 404. |
| F-1-3; V-1/V-2 claims gaps | **Partly regressed as F-4-1 only.** Prior public/runtime/durability claims map to tests; the new live-global limit statement does not. |
| F-1-4 — cassette metaphors/unclear license control | **Fixed.** Current section labels and `Enter a Studio license` name their subjects/results. |
| F-1-5 — route social metadata | **Fixed.** Every checked public route changes OG/Twitter title and description with its route metadata. |
| F-2-1 — demo pointer survived exit | **Fixed.** Start for real deleted the workspace and cleared every demo session key. |
| F-2-2 — metaphorical 404 H1 | **Fixed.** H1 is `This page was not found`. |
| F-3-2/F-3-3 — vague `Clear` copy | **Fixed.** Landing uses `Receipt: PDF after a decision` and `approval link`. |
| F-3-4 — PDF output term mismatch | **Fixed.** README uses `PDF receipt`. |
| F-3-5/F-3-6 — privacy/terms slogan headings | **Fixed.** Current H1s name data storage and approval-record terms. |
| F-3-7 — untested cross-device license promise | **Fixed.** The unsupported `on any device` wording is absent. |
| F-3-8 — untested Dodo/merchant claims | **Fixed.** Those payment-party assertions are absent; conditional Sociobot checkout is covered by `studio-offer`. |
| V-1 Studio, Unicode PDF, validation, cache/robots, start logging; V-2 owner PDF/Docker; V-3 rate-limit topology | **Fixed/confirmed.** Current exact local claim suite passes; current live checks confirmed durable demo continuity and same-origin delivery. F-4-1 remains the one missing claims-manifest mapping. |

## What would make this perfect

Remove the one untested global-rate-limit assertion, or promote it to a separately listed and deployment-verifiable claim. Then rerun this full fresh-context review. With that one statement resolved, this review found no remaining usability, demo, privacy, claims, routing, accessibility, or visual-identity defect.
