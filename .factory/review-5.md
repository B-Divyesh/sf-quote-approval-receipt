# Adversarial first-read review 5 — PASS

- Product: Quote Approval Receipt
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Source reviewed: clean clone of `91d1a113a12d1ea2185f5a3554397eca46f83e7f`
- Live build: `fe0b3a58c7c52180ae1a46f3e2012529860c7c92`; later repository commits change QA documentation only
- Viewports: fresh 390×844 mobile and 1440×900 desktop
- Verdict: **PASS — zero findings, zero untested claims.**

## First screen

Before scrolling, both viewports answer all three required questions.

- **What it does:** records who approved a quote and provides a PDF receipt after the decision.
- **For whom:** contractors whose clients approve in email or chat and may later change expectations.
- **What to click first:** **Try it with sample data**. The adjacent result says `Loads a private sample quote. Nothing enters your records.`

The exact headline is `Record who approved your quote`. The exact audience sentence is `For contractors whose client approves by email or chat, then changes expectations later.` The headline, audience, primary action, result note, and three plain facts were fully visible at 390×844 and 1440×900. Neither viewport overflowed.

## Findings

None. No blocking or minor finding remains.

## Copy audit

Counts use visible whitespace-separated words. Hyphenated values and symbols count as one word. Headings, controls, navigation, sample labels, alternative text, and footer copy are included so that out-of-context wording is also checked. No unit exceeds 22 words. No banned marketing word, jargon that obstructs the intended reader, metaphor heading, inconsistent product term, or non-result-naming action was found. Therefore no rewrite is proposed.

### Landing page

| Words | Exact copy unit | Result |
| ---: | --- | --- |
| 4 | Skip to main content | pass |
| 3 | Quote Approval Receipt | pass — wordmark |
| 1 | Demo | pass |
| 3 | Make a link | pass — result-naming navigation |
| 1 | Privacy | pass |
| 5 | Record who approved your quote | pass — job-first H1 |
| 13 | For contractors whose client approves by email or chat, then changes expectations later. | pass |
| 5 | Try it with sample data | pass — result-naming primary action |
| 5 | Loads a private sample quote. | pass |
| 4 | Nothing enters your records. | pass — `demo-sandbox` |
| 4 | Free: 30-day link retention | pass — `retention-policy` |
| 6 | Private: delete or export each record | pass — `record-control` |
| 5 | Receipt: PDF after a decision | pass — `pdf-receipt` |
| 10 | A cassette holding quote sheets beside a red approval stamp. | pass — image alt text |
| 3 | Fix the quote. | pass |
| 4 | Name the decision maker. | pass |
| 3 | Keep the receipt. | pass |
| 3 | Example approval receipt | pass — section label |
| 8 | Turn an existing quote into an approval link | pass |
| 11 | Paste the agreed scope and totals, then make an approval link. | pass |
| 4 | Make an approval link | pass — result-naming action |
| 1 | Approved | pass — sample status |
| 2 | Quote NS-2048 | pass — sample identifier |
| 2 | Juniper Market | pass — sample client |
| 5 | Mara Chen · Brand director | pass — sample decision maker |
| 6 | 18 Aug 2026 · 14:32 UTC | pass — sample time |
| 2 | proof 92dc…41a | pass — sample proof label |
| 4 | How approval links work | pass — section label |
| 5 | How the record gets made | pass |
| 4 | 1. Enter quote details | pass |
| 7 | Enter the quote details you already agreed. | pass |
| 4 | 2. Send the link | pass |
| 8 | Share the private link with the client contact. | pass |
| 4 | 3. Keep the receipt | pass |
| 7 | Download the named decision and quote snapshot. | pass |
| 4 | What this record does | pass — section label |
| 4 | Record a quote decision | pass |
| 8 | Use this tool to record a quote decision. | pass |
| 9 | It records the quote, decision maker, decision, and time. | pass |
| 9 | We store the approver details needed for that record. | pass |
| 5 | No tracking scripts run here. | pass — `first-party-only` |
| 6 | For teams that keep records longer | pass — section label |
| 5 | Keep receipts for one year | pass — `retention-policy` |
| 5 | Studio checkout is not available. | pass — current `studio-offer` state |
| 7 | Free links keep records for 30 days. | pass — `retention-policy` |
| 4 | Enter a Studio license | pass — result-naming action |
| 7 | Named quote decisions, kept with the quote. | pass |
| 1 | Privacy | pass |
| 1 | Terms | pass |
| 4 | Built by Param Factory | pass — attribution |
| 2 | external site | pass — destination notice |
| 3 | v1.3 · fe0b3a5 | pass — live build label |

Terminology is consistent: **quote**, **quote snapshot**, **approval link**, **decision maker**, **decision**, **receipt**, **Studio license**, and **demo**.

### README

Standalone URLs and command blocks are not sentences. Every prose sentence and heading is listed.

| Words | Exact sentence or heading | Result |
| ---: | --- | --- |
| 3 | Quote Approval Receipt | pass |
| 11 | Capture who approved a fixed quote and issue a timestamped receipt. | pass — `pdf-receipt` |
| 16 | Quote Approval Receipt is for small agencies and contractors who receive approvals in email or chat. | pass |
| 9 | It creates a private link for an existing quote. | pass — `private-links` |
| 15 | The client names the decision maker, approves or requests changes, and receives a PDF receipt. | pass — `pdf-receipt` |
| 3 | Try the sample | pass |
| 8 | Open `/?demo=1` after starting the service, or visit: | pass |
| 11 | The demo creates a random workspace that expires after 24 hours. | pass — `demo-sandbox`, `retention-policy` |
| 7 | It stays separate from real quote records. | pass — `demo-sandbox` |
| 9 | Resetting or leaving the demo deletes its current workspace. | pass — `demo-sandbox` |
| 6 | See `.factory/demo.md` for the sandbox contract. | pass |
| 2 | Run locally | pass |
| 10 | Requirements: Node.js 22+, npm, Rust 1.88+, and SQLite build support. | pass — necessary developer terms |
| 2 | Open `http://localhost:8080`. | pass |
| 12 | The server creates `data/quotes.sqlite3` and a random privacy salt on first boot. | pass — `runtime-contract` |
| 7 | No secret or configuration variable is required. | pass — `runtime-contract` |
| 21 | For frontend work with live reload, run the backend on port 8080 and then run `npm run dev` in another shell. | pass |
| 3 | Test and build | pass |
| 21 | `npm test` builds the frontend, runs Rust tests, starts the service, and runs the Playwright claim, mobile, rate-limit, and accessibility checks. | pass — observed |
| 5 | The frontend output is `dist/`. | pass — observed |
| 7 | The container listens on `PORT` (default `8080`). | pass — `runtime-contract` |
| 20 | It starts with no required configuration, creates a random privacy salt on first boot, and `/health` reports the build SHA. | pass — `runtime-contract` |
| 14 | Set `DATA_DIR` or `DATABASE_URL` only when you need to override the persisted SQLite location. | pass |
| 3 | Data and security | pass |
| 10 | Each real quote gets independent random approval and owner tokens. | pass — `private-links` |
| 12 | The owner token stays in that browser and authorizes export or deletion. | pass — `record-control` |
| 11 | Private pages and API responses use `noindex` and `no-store` response policies. | pass — `private-links` |
| 7 | Each fixed quote accepts one final decision. | pass — `single-decision` |
| 14 | API read and write bursts return `429` with `Retry-After` when they exceed the limit. | pass — `rate-limits` |
| 7 | Free links retain records for 30 days. | pass — `retention-policy` |
| 14 | A 365-day request succeeds only after the backend verifies a Studio license with Sociobot. | pass — `retention-policy` |
| 13 | A $29 Studio checkout on Sociobot appears only when the product is available. | pass — `studio-offer` |
| 6 | License holders can enter a license. | pass — `studio-offer` |
| 1 | Deploy | pass |
| 4 | Build the root `Dockerfile`. | pass |
| 14 | The image runs as the non-root `app` user and needs only `PORT` to start. | pass — `runtime-contract` |
| 21 | The factory Container Apps release is a strict one-replica service: its dedicated Azure Files share is mounted at `/durable` and `DURABLE_DATA_DIR=/durable`. | pass — `durable-snapshot` deployment assertion |
| 19 | The service writes a durable snapshot after each committed change, so a replacement process restores records before serving traffic. | pass — `durable-snapshot` |
| 10 | Durable storage is an optional deployment override for local use. | pass |
| 12 | The factory owns deployment, DNS, billing registration, and the production build SHA. | pass — ownership statement |
| 4 | Apply `.factory/containerapp-deploy.json` for production. | pass — instruction |
| 12 | Render the candidate image into that contract with `node scripts/deployment-contract.mjs <image> 8080`. | pass — instruction |
| 10 | The generic three-replica template is unsafe for this SQLite service. | pass — reproduced by `test:replica-split` |
| 16 | After deployment, run the live topology, real workflow, 20-demo, rate-limit, and review scripts listed in `package.json`. | pass — instruction |
| 16 | The topology check fails unless the active revision has one ready replica and mounted durable state. | pass — checked script contract |
| 1 | License | pass |
| 1 | MIT. | pass |
| 2 | See LICENSE. | pass |

## Demo and sandbox

- The landing action reaches `/?demo=1` in one click. The first rendered product screen already contains the realistic Northstar Studio quote `NS-2048` for Juniper Market, two priced line items, totals, prefilled decision-maker fields, consent, and a final-decision warning.
- The persistent banner reads `Demo — sample data, nothing is saved to your records` and includes **Reset demo** and **Start for real**.
- Reset deleted the first workspace and created a different `demo:workspace`; the old share returned 404. Start for real deleted the replacement workspace, cleared every `demo:` session key, opened the empty quote builder, and left local storage without an owner key.
- The deployment regression check passed 20/20 separate Chromium processes. Every process observed demo create 201, immediate share read 200 with `Half-day product shoot`, and exit cleanup 404.
- A separate live sender/client check completed create, cross-context read, decision, sender receipt retrieval, and deletion. The deleted real test record returned 404.
- The browser request log across landing, demo creation, reset, and exit contained only `https://quote-approval-receipt.sociobot.in`. There is no offline product claim to test; the local offline-error regression still supplies a useful retry instruction.

## Claims gate

The clean clone was `/tmp/qar-review5-clean.GdEyKV/repo` at `91d1a113a12d1ea2185f5a3554397eca46f83e7f`. `npm ci` installed 30 packages with zero vulnerabilities. Every literal command in `.factory/claims.json` was run independently.

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-sandbox` | `npm test -- --grep @claim:demo-sandbox` | PASS |
| `quote-snapshot` | `npm test -- --grep @claim:quote-snapshot` | PASS |
| `pdf-receipt` | `npm test -- --grep @claim:pdf-receipt` | PASS |
| `record-control` | `npm test -- --grep @claim:record-control` | PASS |
| `first-party-only` | `npm test -- --grep @claim:first-party-only` | PASS |
| `retention-policy` | `npm test -- --grep @claim:retention-policy` | PASS |
| `studio-offer` | `npm test -- --grep @claim:studio-offer` | PASS |
| `private-links` | `npm test -- --grep @claim:private-links` | PASS |
| `single-decision` | `npm test -- --grep @claim:single-decision` | PASS |
| `network-address-privacy` | `npm test -- --grep @claim:network-address-privacy` | PASS |
| `rate-limits` | `npm test -- --grep @claim:rate-limits` | PASS |
| `runtime-contract` | `npm run test:runtime-contract` | PASS |
| `durable-snapshot` | `npm run test:durable-snapshot` | PASS |

The unfiltered `npm test` also passed: the manifest and deployment-contract gates, production build, four Rust tests, runtime/split/durable checks, and all 22 Playwright tests. The build produced 27.11 kB JavaScript (8.96 kB gzip) and 15.55 kB CSS (4.20 kB gzip). No landing or README claim-like statement lacks a manifest entry or directly exercised regression.

## Structure, links, accessibility, and visual identity

- `/`, `/demo`, `/?demo=1`, `/new`, `/privacy`, and `/terms` returned 200. A deliberate missing route returned a designed HTTP 404 with H1 `This page was not found` and **Return home**.
- Every checked route has `lang=en`, one H1, one main landmark, a route-specific title, description, canonical URL, Open Graph/Twitter title and description, favicon, touch icon, and the consistent header/footer with Privacy and Terms.
- Public links and assets were crawled: all six public entries, `robots.txt`, `sitemap.xml`, favicon, touch icon, social image, cassette image, and the Param Factory external link returned 200. The two `mailto:` destinations are explicit. Generated approval, management, receipt, JSON, and PDF paths were exercised by the local claims and live workflow.
- SPA navigation uses real URLs. Route transitions, browser Back, H1 focus, and the polite route announcement work. The skip link is first in keyboard order. The 390px routes do not overflow, including at 200% text, and visible controls meet the 44px target in the full suite.
- The live review found no console errors on normal routes and zero serious/critical Axe findings. Reduced motion stops animation in the local regression.
- Response headers carry CSP `frame-ancestors 'none'`, `nosniff`, `no-referrer`, frame denial, and restricted permissions. Private pages and APIs are `no-store` and `noindex`; hashed assets are compressed and immutable.
- The cassette-era zine identity is distinct rather than a generic SaaS template: warm paper, hard black rules and shadows, teal/red spot colours, torn edges, label typography, and original quote-cassette art. Its information order still follows the required site skeleton.

## Earlier findings rechecked

| Earlier finding | Current live and code confirmation |
| --- | --- |
| F-1-1 — live sample lost across requests | **Fixed.** The current `test:live-demo` passed 20/20 separate Chromium processes with 201 → 200 → 404 continuity; `/health` reports durable mode. |
| F-1-2 — 404 mobile overflow | **Fixed.** The live 390px 404 has no horizontal overflow; the 200% mobile regression passes. |
| F-1-3 — unlisted public claims | **Fixed.** The 13-entry manifest integrity gate and all exact commands pass; this fresh audit found no remainder. |
| F-1-4 — metaphor labels and unclear license control | **Fixed.** The live labels name the example receipt, approval-link steps, and Studio-license action. |
| F-1-5 — shared social metadata | **Fixed.** Home, builder, legal, demo, and 404 routes update title, description, canonical, Open Graph, and Twitter copy. |
| F-2-1 — demo pointer survived exit | **Fixed.** Reset and Start for real delete the server workspace and clear all demo session keys; both old shares returned 404. |
| F-2-2 — metaphorical 404 H1 | **Fixed.** The live and source H1 is `This page was not found`. |
| F-3-1 — deployed state split regressed | **Fixed.** Live health reports `durable_snapshot:true`; the 20-process demo and cross-context real workflow both pass. |
| F-3-2 — `Clear` first-screen fact | **Fixed.** Live copy is `Receipt: PDF after a decision`. |
| F-3-3 — `one clear link` heading | **Fixed.** Live copy is `Turn an existing quote into an approval link`. |
| F-3-4 — PDF output called a record | **Fixed.** Landing, README, and controls use `PDF receipt`. |
| F-3-5 — slogan Privacy H1 | **Fixed.** Live H1 is `How we store quote and approver data`. |
| F-3-6 — promotional Terms H1 | **Fixed.** Live H1 is `Terms for quote approval records`. |
| F-3-7 — untested cross-device license promise | **Fixed.** `on any device` remains absent; the scoped license-entry claim passes. |
| F-3-8 — unlisted Dodo/merchant claims | **Fixed.** Those statements remain absent; conditional Sociobot checkout is covered by `studio-offer`. |
| F-4-1 — untested global rate-limit claim | **Fixed.** The exact global-limit sentence remains absent and the manifest regression rejects its return. |

The earlier verification findings recorded in the polish files also remain closed: server-verified Studio retention, Unicode PDF text, supported-currency validation, reflow and touch targets, private cache/robot headers, real 404 status, startup logging, sender receipt/PDF access, non-root container contract, durable replacement, and the one-writer deployment overlay all passed their current regressions. The source implementation matches the live application build; commits after the live SHA contain only verification documentation.

## Missed leverage

No additional feature is clearly implied by the brief. The job is deterministic capture of an already agreed quote and a named decision; generated wording would weaken the fixed-snapshot contract. The product already provides PDF and JSON export. Document import could reduce typing, but the brief does not define an ingestion job, and explicit field entry keeps the exact shared snapshot reviewable. No decorative AI, embedded provider key, or Azure endpoint is present.

## What would make this perfect

Nothing remains to change under this review. Keep the checked deployment renderer and the 20-process live demo gate mandatory for future releases so the previously recurring state split cannot return.
