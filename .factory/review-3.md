# Adversarial first-read review 3 — FAIL

- Product: Quote Approval Receipt
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Revision reviewed: `6bb800a4f4228660bbc8335d0f14f272541c4116`
- Viewports: fresh 390×844 mobile and 1440×900 desktop
- Verdict: **FAIL** — one blocking deployment regression and seven copy/claims findings remain. The required verdict is not PASS while any finding remains.

## First screen

Before scrolling, the first screen itself answers all three questions at both sizes:

- **What it does:** records who approved a quote and produces a PDF receipt.
- **For whom:** contractors whose clients approve in email or chat and later change expectations.
- **What to click first:** **Try it with sample data**. The adjacent copy says that it loads a private sample quote and does not enter the visitor's records.

The exact supporting text is `Record who approved your quote`, `For contractors whose client approves by email or chat, then changes expectations later.`, `Try it with sample data`, and `Loads a private sample quote. Nothing enters your records.` The headline, audience sentence, action, explanation, and all three facts were fully visible without scrolling at 390×844 and 1440×900. Neither viewport overflowed. The first action then failed consistently in fresh processes; see F-3-1.

## Findings

### F-3-1 — BLOCKING — the live demo again loses every newly created sample across requests

**Regression of:** F-1-1 and the critical state-split findings in `verification.md`, `verification-2.md`, and `verification-3.md`.

**Exact location and quote:** landing action **Try it with sample data** and direct `/demo`. After the click, the error page says `The demo could not start` and `This approval link was not found or has been deleted. Reset the demo to try again.`

**Evidence:** the repository's own live gate failed immediately:

```text
$ LIVE_URL=https://quote-approval-receipt.sociobot.in npm run test:live-demo
Error: attempt 1: sample read returned 404
```

A separate 20-process check then produced **0/20 usable demos**. Every `POST /api/demo` returned 201, every immediately following `GET /api/share/<new-token>` returned 404, and every page showed `The demo could not start`. Cleanup requests alternated between 204 and 404, which is further evidence that requests reach different state domains. The live `/health` endpoint reports the reviewed SHA, so this is not stale frontend code.

One earlier same-context attempt happened to work and proved that, when consecutive requests reach the same state, the sample, Reset, Start for real, storage cleanup, and same-origin request policy work. That does not make the public one-click path dependable.

**Why a first-time visitor fails:** the primary action promises a usable sample but normally lands on an error before showing the product. The same state split also makes real quote, owner, decision, receipt, export, and deletion continuity unreliable.

**Concrete fix:** put every record read and write behind one shared durable database, or enforce one correctly mounted durable replica. Make the post-deploy `npm run test:live-demo` check a release gate and require at least 20/20 fresh browser processes to observe `POST /api/demo` 201, immediate share `GET` 200, visible `Half-day product shoot`, successful exit deletion, empty demo storage, and old share 404.

### F-3-2 — minor — a landing fact uses a promotional adjective instead of naming the fact

**Exact quote/location:** first-screen fact, `Clear: PDF receipt after a decision`.

**Why it fails:** `Clear` is an unmeasured quality label. The useful information is the PDF receipt and when it is produced.

**Concrete rewrite:** `Receipt: PDF after a decision`.

### F-3-3 — minor — a landing heading uses another unmeasured adjective

**Exact quote/location:** live-preview H2, `Turn an existing quote into one clear link`.

**Why it fails:** `clear` carries no testable or actionable information.

**Concrete rewrite:** `Turn an existing quote into an approval link`.

### F-3-4 — minor — README changes the name of the PDF output

**Exact quote/location:** README introduction, `The client names the decision maker, approves or requests changes, and receives a PDF record.` The landing page and controls call the same output a `PDF receipt`.

**Why it fails:** `record` and `receipt` are separate concepts elsewhere in the product. Changing the output name makes the result less predictable.

**Concrete rewrite:** `The client names the decision maker, approves or requests changes, and receives a PDF receipt.`

### F-3-5 — minor — the Privacy H1 is a slogan rather than the page subject

**Exact quote/location:** `/privacy` H1, `Privacy that fits the record`.

**Why it fails:** `fits the record` adds mood but does not identify what privacy information the page contains. A heading list should name the page without surrounding copy.

**Concrete rewrite:** `How we store quote and approver data`.

### F-3-6 — minor — the Terms H1 contains vague promotional copy

**Exact quote/location:** `/terms` H1, `Terms for clear quote records`.

**Why it fails:** `clear` is qualitative and does not identify the governed workflow.

**Concrete rewrite:** `Terms for quote approval records`.

### F-3-7 — minor — the README's cross-device license claim is not listed or tested

**Exact quote/location:** README, `License holders can enter a license on any device.`

**Why it fails:** `studio-offer` tests license entry in one browser context. Its claim text does not include, and its test does not exercise, the `any device` promise.

**Concrete fix:** use `License holders can enter a license.` Or add the cross-device wording to `claims.json` and restore the same fixture license successfully in two independent clean contexts.

### F-3-8 — minor — payment-party statements are absent from the claims manifest

**Exact quotes/locations:** `/privacy`: `Sociobot/Dodo handles Studio checkout when new licenses are available.` `/terms`: `Sociobot/Dodo is the merchant of record when checkout is available.`

**Why it fails:** these are material payment and merchant claims. `studio-offer` checks the displayed price, conditional checkout link, and license form; it does not verify the checkout destination's payment handler or merchant of record.

**Concrete fix:** remove the unverified Dodo/merchant statements. If the relationship must be stated, add one claim entry and a recorded checkout-contract test that verifies the provider and merchant information without spending money.

## Copy audit

Counts use visible, whitespace-separated reader words; hyphenated terms count as one. Headings and controls are included because the review must check them out of context. Sample names, dates, and status labels are also included for completeness. No unit exceeds 22 words and no banned plain-words term appears. Rows marked `flag` map to findings above.

### Landing page

| Words | Copy unit | Result |
| ---: | --- | --- |
| 4 | Skip to main content | pass — keyboard action |
| 3 | Quote Approval Receipt | pass — wordmark |
| 1 | Demo | pass — navigation |
| 3 | Make a link | pass — navigation |
| 1 | Privacy | pass — navigation |
| 5 | Record who approved your quote | pass |
| 13 | For contractors whose client approves by email or chat, then changes expectations later. | pass |
| 5 | Try it with sample data | pass — result-naming action |
| 5 | Loads a private sample quote. | pass |
| 4 | Nothing enters your records. | pass — `demo-sandbox` |
| 4 | Free: 30-day link retention | pass — `retention-policy` |
| 6 | Private: delete or export each record | pass — `record-control`, `private-links` |
| 6 | Clear: PDF receipt after a decision | **flag — F-3-2** |
| 3 | Fix the quote. | pass |
| 4 | Name the decision maker. | pass |
| 3 | Keep the receipt. | pass |
| 3 | Example approval receipt | pass |
| 8 | Turn an existing quote into one clear link | **flag — F-3-3** |
| 10 | Paste the agreed scope and totals, then make an approval link. | pass |
| 4 | Make an approval link | pass — result-naming action |
| 1 | Approved | pass — sample status |
| 2 | Quote NS-2048 | pass — sample identifier |
| 2 | Juniper Market | pass — sample client |
| 4 | Mara Chen · Brand director | pass — sample approver |
| 5 | 18 Aug 2026 · 14:32 UTC | pass — sample time |
| 2 | proof 92dc…41a | pass — sample proof |
| 4 | How approval links work | pass |
| 5 | How the record gets made | pass |
| 4 | 1. Enter quote details | pass |
| 7 | Enter the quote details you already agreed. | pass |
| 4 | 2. Send the link | pass |
| 8 | Share the private link with the client contact. | pass |
| 4 | 3. Keep the receipt | pass |
| 7 | Download the named decision and quote snapshot. | pass |
| 4 | What this record does | pass |
| 4 | Record a quote decision | pass |
| 8 | Use this tool to record a quote decision. | pass |
| 9 | It records the quote, decision maker, decision, and time. | pass |
| 9 | We store the approver details needed for that record. | pass |
| 5 | No tracking scripts run here. | pass — `first-party-only` |
| 6 | For teams that keep records longer | pass |
| 5 | Keep receipts for one year | pass — `retention-policy` |
| 5 | Studio checkout is not available. | pass — live `/api/studio` returned unavailable; `studio-offer` covers both states |
| 7 | Free links keep records for 30 days. | pass — `retention-policy` |
| 4 | Enter a Studio license | pass — action names its result |
| 7 | Named quote decisions, kept with the quote. | pass |
| 1 | Privacy | pass — footer link |
| 1 | Terms | pass — footer link |
| 4 | Built by Param Factory | pass — attribution |
| 2 | external site | pass — destination notice |
| 1 | v1.2 | pass — version |

Terminology is otherwise consistent: **quote**, **quote snapshot**, **approval link**, **decision maker**, **decision**, **receipt**, **Studio license**, and **demo**.

### README

Standalone URLs and code blocks are not sentences; every heading and prose sentence is listed.

| Words | Sentence or heading | Result |
| ---: | --- | --- |
| 3 | Quote Approval Receipt | pass |
| 11 | Capture who approved a fixed quote and issue a timestamped receipt. | pass |
| 16 | Quote Approval Receipt is for small agencies and contractors who receive approvals in email or chat. | pass |
| 9 | It creates a private link for an existing quote. | pass |
| 15 | The client names the decision maker, approves or requests changes, and receives a PDF record. | **flag — F-3-4** |
| 3 | Try the sample | pass |
| 8 | Open `/?demo=1` after starting the service, or visit: | pass |
| 11 | The demo creates a random workspace that expires after 24 hours. | pass — `demo-sandbox`, `retention-policy` |
| 7 | It stays separate from real quote records. | pass — `demo-sandbox` |
| 9 | Resetting or leaving the demo deletes its current workspace. | pass locally / false for the current split live deployment — F-3-1 |
| 8 | See `.factory/demo.md` for the sandbox contract. | pass |
| 2 | Run locally | pass |
| 10 | Requirements: Node.js 22+, npm, Rust 1.88+, and SQLite build support. | pass |
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
| 12 | API write bursts return `429` with `Retry-After` when they exceed the limit. | pass — `rate-limits` |
| 7 | Free links retain records for 30 days. | pass — `retention-policy` |
| 14 | A 365-day request succeeds only after the backend verifies a Studio license with Sociobot. | pass — `retention-policy` |
| 8 | Studio checkout appears only when Sociobot reports availability. | pass — `studio-offer` |
| 9 | License holders can enter a license on any device. | **flag — F-3-7** |
| 1 | Deploy | pass |
| 4 | Build the root `Dockerfile`. | pass |
| 14 | The image runs as the non-root `app` user and needs only `PORT` to start. | pass — `runtime-contract` |
| 9 | For the factory Container Apps release, use one replica. | pass as an instruction; the live behavior contradicts the intended topology — F-3-1 |
| 11 | Mount its dedicated Azure Files share at `/durable` and set `DURABLE_DATA_DIR=/durable`. | pass as an instruction |
| 19 | The service writes a durable snapshot after each committed change, so a replacement process restores records before serving traffic. | pass locally — `durable-snapshot`; live continuity fails — F-3-1 |
| 10 | Durable storage is an optional deployment override for local use. | pass |
| 12 | The factory owns deployment, DNS, billing registration, and the production build SHA. | pass — ownership statement |
| 1 | License | pass |
| 1 | MIT. | pass |
| 2 | See LICENSE. | pass |

## Demo, sandbox, claims, and privacy

The first successful same-context run opened `/?demo=1` in one click and immediately showed the realistic Northstar Studio `NS-2048` quote for Juniper Market, line items, totals, and the decision form. Its persistent banner said `Demo — sample data, nothing is saved to your records` and provided **Reset demo** and **Start for real**. Reset changed the workspace and made the old share return 404. Start for real cleared all `demo:` session keys and made the replacement share return 404. Local storage remained empty. The request log contained only `https://quote-approval-receipt.sociobot.in`.

That successful connection-local path is outweighed by F-3-1: the required fresh-process check failed 20/20 and the repository's live gate failed on attempt one.

### Exact claim commands from a clean clone

The clean clone was `/tmp/qar-review3-clean.flyaeX/repo` at the reviewed SHA. The repository has no committed npm lockfile, so `npm ci` was unavailable; the documented `npm install` completed with zero vulnerabilities. Every command in `.factory/claims.json` was then run verbatim.

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-sandbox` | `npm test -- --grep @claim:demo-sandbox` | PASS locally; **claim false on live deployment — F-3-1** |
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

The claim-like sentences not adequately represented by the manifest are F-3-7 and F-3-8. No offline claim is made. The live request log confirms the first-party-only promise for landing and the one same-context successful demo flow.

## Structure, accessibility, and quality gates

- `/`, `/new`, `/privacy`, `/terms`, and the designed 404 returned the expected 200/404 statuses, route-specific titles, descriptions, canonical URLs, Open Graph/Twitter copy, and one H1. `/demo` returned HTTP 200 but rendered the error title and H1 because of F-3-1.
- The home title is `Quote Approval Receipt — Record quote decisions`, under 60 characters and in the required product/job pattern. Policy and app routes use the documented route-name/product pattern.
- The favicon, 180×180 apple-touch icon, and 1200×630 product-specific social image returned 200. `robots.txt` and `sitemap.xml` returned 200; the sitemap lists `/`, `/demo`, `/new`, `/privacy`, and `/terms`.
- All genuine public links found on the public pages returned 200. The two `mailto:` links are explicit. The synthetic missing route returned the intended 404.
- History navigation worked: **Make a link** moved focus to the `/new` H1, and Back restored `/` and focused its H1. The skip link is first in keyboard order.
- The live review found zero serious/critical Axe violations on the checked public routes, no landing console error, no horizontal overflow at 390px, and no third-party request. `verify-url.sh` passed with `lang=en`, one H1, a main landmark, alt text, and zero console errors.
- The cassette-era zine design matches `.factory/design.md`: warm paper, black keylines, teal/red spot colours, original cassette artwork, torn edges, and hard shadows. It is recognisable and is not a generic SaaS card/gradient template.
- The full clean-clone `npm test` passed: 4 Rust tests and 19 Playwright tests. `npm run build` produced `dist/`; JS is 27.01 KB raw / 8.96 KB gzip and CSS is 15.48 KB raw / 4.20 KB gzip. `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high` passed.
- Live spot checks returned 422 with useful supported-currency guidance, rejected forged 365-day retention with 403, kept private pages `noindex`/`no-store`, and served hashed JavaScript Brotli-compressed with immutable caching.

## Earlier findings rechecked

| Earlier finding | Current live and code result |
| --- | --- |
| F-1-1 / verification 1–3: split production state breaks demo and records | **REGRESSED — BLOCKING F-3-1.** Local durable and demo tests pass, but live fresh processes are 0/20. |
| Verification 1: dead Studio purchase and forgeable 365-day retention | Fixed in code and current behavior. Live checkout is accurately hidden while unavailable; a forged live 365-day request returned 403. |
| Verification 1: Unicode PDF corruption | Fixed in source and the exact `pdf-receipt` test, which extracts `José Núñez — Direção`. Reliable live end-to-end use remains blocked by F-3-1. |
| Verification 1 / 2: incomplete claims and README runtime/durable claims | The previously named runtime, durability, PDF, retention, privacy, and control gaps have tests and pass. New residual claim gaps are F-3-7 and F-3-8. |
| Verification 1: invalid currency | Fixed live and locally. Live `!!!` returned 422 with the supported currencies. |
| Verification 1: 200% reflow and undersized targets | Fixed in source and the passing mobile regression. Live 390px routes checked by the review did not overflow. |
| Verification 1: private/cache headers and compressed immutable assets | Fixed. Live private responses are `no-store`/`noindex`; hashed JS is Brotli-compressed and immutable. |
| Verification 1: 404 status, management robots, startup logging | Fixed. The styled missing route returns 404; private routes have response headers; `runtime-contract` passes. |
| Verification 2: sender cannot retrieve receipt/PDF | The source fix remains and `@regression:owner-receipt-link` passes in a fresh sender context. Public reliability is still subject to F-3-1. |
| Verification 2: pinned Rust minor image | Fixed. `Dockerfile` uses `rust:1-slim`; the runtime contract checks the non-root image contract. |
| F-1-2 / verification 4: 404 mobile overflow | Fixed live and in the 390px regression (`scrollWidth === innerWidth`). |
| F-1-3: snapshot, Studio, regulated-service, and artwork claims | The named fixes remain. F-3-7 and F-3-8 are separate residual claim wording outside those exact repaired sentences. |
| F-1-4: cassette metaphor labels and unclear license control | Fixed on the landing and app routes. New policy-heading issues are F-3-5 and F-3-6. |
| F-1-5: shared social metadata | Fixed live on every checked route. |
| F-2-1: demo pointer/workspace retained on exit | Fixed in source, locally, and in the one successful live same-context run. The demo cannot be relied on at all because F-3-1 has independently regressed. |
| F-2-2: metaphorical 404 H1 | Fixed live and in source: `This page was not found`. |

## Missed leverage

No additional AI feature is justified. The core job is deterministic evidence capture; generated wording would weaken the fixed-record promise. The product already exports JSON and PDF. Importing a quote document could save re-entry, but the brief does not require a document-ingestion workflow and the current explicit fields make the captured snapshot reviewable before sharing. No decorative AI, provider key, or Azure endpoint is present.

## What would make this perfect

Restore one durable live state domain and keep the 20-process live demo check as a mandatory deployment gate. Then remove the two `clear` labels, call the PDF output a `receipt` everywhere, replace the Privacy and Terms slogans with literal headings, and either remove or test the cross-device and payment-party claims. Re-run this complete review from a fresh public browser context; PASS requires zero remaining findings.
