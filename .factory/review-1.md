# Adversarial first-read review 1 — FAIL

- Product: Quote Approval Receipt
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Viewports: cold 390×844 mobile and 1440×900 desktop
- Verdict: **FAIL** — one blocking end-to-end defect and four additional findings remain.

## First screen

The first screen is understandable before scrolling at both viewports.

- **What it does:** records who approved a quote and gives a receipt.
- **For whom:** contractors whose clients approve in email or chat and may later change expectations.
- **First action:** click **Try it with sample data**; adjacent text says it loads a private sample quote without entering records.

The literal supporting text is `Record who approved your quote`, `For contractors whose client approves by email or chat, then changes expectations later.`, and `Try it with sample data`. The explanation is clear. The action itself is not dependable in a fresh public browser process; see F-1-1.

## Findings

### F-1-1 — BLOCKING — live sample demo loses its newly created record

**Location/evidence:** `/demo`, fresh browser processes. In 20 independent Chromium launches, `POST /api/demo` returned `201`, followed immediately by `GET /api/share/<new token>` returning `404`. Each page showed `The demo could not start` rather than the sample `Half-day product shoot`: **0/20 completed**. One initial cold check showed `This approval link was not found or has been deleted. Reset the demo to try again.`

**Why this fails:** a visitor’s first action is a one-click demo. It creates something the next request cannot read, so the visitor cannot try the product. This is the same deployment-only persistence/state-split defect in `.factory/verification-2.md` and `.factory/verification-3.md`; it is an unfixed earlier blocker. The local `@claim:demo-sandbox` test passes because it uses one local process and cannot observe the public deployment boundary.

**Concrete fix:** route all record reads and writes to one shared durable store (or one truly single serving replica), then add a deployment test that launches at least 20 separate browser processes or fresh TCP clients. Each must assert `POST /api/demo` 201, the immediately returned share URL 200, and visible `Half-day product shoot`. Keep this deployment test in addition to the local claim test.

### F-1-2 — minor — the designed 404 still overflows at the required mobile width

**Location/evidence:** live `/404-review-missing` at 390px: `document.documentElement.scrollWidth` is **392** while `innerWidth` is **390**.

**Why this fails:** this repeats the P3 known gap in the prior `.factory/handoff.md` and `.factory/verification-4.md`. A horizontal page movement is a visible mobile layout regression on the recovery route for an expired private link.

**Concrete fix:** constrain the 404 illustration/container, including shadow/transform, to the viewport. Add a 390px assertion for the 404 route: `scrollWidth <= innerWidth`.

### F-1-3 — minor — public claims have no matching claim entry and observable test

**Location/evidence:** live landing and README, compared with `.factory/claims.json`.

| Unlisted claim-like text | Location | Concrete fix |
| --- | --- | --- |
| `The link freezes that snapshot.` | Landing, “Live preview / side A” | Add a `quote-snapshot-immutable` claim and test proving later input cannot alter a created quote, or use `The link shows the quote details you enter.` |
| `This is not proposal software or a regulated signature service.` | Landing; README has the equivalent e-signature claim | Remove the untestable regulated-status assertion; use `Use this tool to record a quote decision.` |
| `New Studio purchases are paused.` | Landing, Studio status | Add a recorded-fixture claim for `/api/studio` availability, or use an actionable neutral status. |
| `The page offers the $29 checkout only while Sociobot reports that the product is available; existing license holders can restore a license at any time.` | README, Data and security | Split it and test both availability responses, displayed price, and restore action. |
| `Original generated artwork` | Landing footer | Remove this public assertion; provenance already belongs in `.factory/design.md` unless a test verifies it. |

**Why this fails:** the claims contract requires every visitor-relevant statement to have a `claims.json` entry and observable sandbox test. Existing tests cover retention, PDF content, private links, tracking, and record control, but not the statements above.

### F-1-4 — minor — decorative copy makes labels and a control unclear out of context

**Location/evidence:** landing copy contains `A SMALL RECORD FOR A COSTLY “WHO SAID YES?”`, `LIVE PREVIEW / SIDE A`, `THREE TRACKS`, `01 / Freeze`, `STUDIO TAPE`, and the text control `Have a license? Paste it`.

**Why this fails:** these labels use cassette metaphors rather than naming a section or result. A screen-reader heading list or hurried visitor cannot know what `side A`, `tracks`, `freeze`, or `tape` means. `Paste it` does not name what will happen or what must be pasted.

**Concrete fix:** retain the visual style but use `Quote approval record`, `Example approval receipt`, `How approval links work`, `1. Enter quote details`, `Studio license`, and `Enter a Studio license`. Delete the first eyebrow unless it supplies useful information.

### F-1-5 — minor — Open Graph and Twitter metadata stays on home-page copy for every route

**Location/evidence:** live `/privacy`, `/terms`, `/new`, `/demo`, and the 404 correctly change title, description, canonical URL, and H1. Their `og:title`, `og:description`, `twitter:title`, and `twitter:description` remain `Quote Approval Receipt — Record quote decisions` / `Capture who approved a fixed quote and issue a timestamped PDF receipt.`

**Why this fails:** shared links to a real route describe a different page.

**Concrete fix:** update Open Graph and Twitter title/description in `pageMeta` for every route, or serve route-specific HTML metadata. Test `/privacy`, `/terms`, `/new`, `/demo`, and 404 rather than only `/`.

## Copy audit

Counts use words as a reader sees them; headings and controls are included so contextual failures are visible. No live landing unit exceeds 22 words. `†` marks F-1-3 or F-1-4. The landing text was captured after the Studio request returned its current paused state.

### Landing page

| Words | Copy unit |
| ---: | --- |
| 9† | A small record for a costly “who said yes?” |
| 5 | Record who approved your quote |
| 13 | For contractors whose client approves by email or chat, then changes expectations later. |
| 5 | Try it with sample data |
| 5 | Loads a private sample quote. |
| 4 | Nothing enters your records. |
| 4 | Free: 30-day link retention |
| 6 | Private: delete or export each record |
| 6 | Clear: PDF receipt after a decision |
| 3 | Fix the quote. |
| 4 | Name the decision maker. |
| 3 | Keep the receipt. |
| 4† | Live preview / side A |
| 8 | Turn an existing quote into one clear link |
| 6 | Paste the agreed scope and totals. |
| 5† | The link freezes that snapshot. |
| 4 | Make an approval link |
| 1 | Approved |
| 2 | Quote NS-2048 |
| 2 | Juniper Market |
| 4 | Mara Chen · Brand director |
| 6 | 18 Aug 2026 · 14:32 UTC |
| 3 | proof 92dc…41a |
| 2† | Three tracks |
| 5 | How the record gets made |
| 2† | 01 / Freeze |
| 7 | Enter the quote details you already agreed. |
| 2 | 02 / Send |
| 8 | Share the private link with the client contact. |
| 2 | 03 / Keep |
| 7 | Download the named decision and fixed snapshot. |
| 5 | What it does not do |
| 7 | Proof of a decision, not an e-signature |
| 10† | This is not proposal software or a regulated signature service. |
| 10 | It records the words, person, time, and fixed quote snapshot. |
| 9 | We store the approver details needed for that record. |
| 5 | No tracking scripts run here. |
| 2† | Studio tape |
| 6 | For teams that keep records longer |
| 5 | Keep receipts for one year |
| 5† | New Studio purchases are paused. |
| 7 | Free links keep records for 30 days. |
| 3† | Have a license? |
| 2† | Paste it. |
| 7 | Named quote decisions, kept with the quote. |
| 3† | Original generated artwork |

### README

No README sentence is over 22 words except the two marked `>22`. The table lists every prose sentence; headings, URLs, and shell commands are not sentences.

| Words | Sentence |
| ---: | --- |
| 11 | Capture who approved a fixed quote and issue a timestamped receipt. |
| 16 | Quote Approval Receipt is for small agencies and contractors who receive approvals in email or chat. |
| 9 | It creates a private link for an existing quote. |
| 15 | The client names the decision maker, approves or requests changes, and receives a PDF record. |
| 12† | This is not proposal software, payment collection, or a regulated e-signature service. |
| 8 | Open `/demo` after starting the service, or visit: |
| 11 | The demo creates a random workspace that expires after 24 hours. |
| 7 | It stays separate from real quote records. |
| 8 | See `.factory/demo.md` for the sandbox contract. |
| 12 | Requirements: Node.js 22+, npm, Rust 1.88+, and SQLite build support. |
| 14 | The server creates `data/quotes.sqlite3` and a random privacy salt on first boot. |
| 7 | No secret or configuration variable is required. |
| 21 | For frontend work with live reload, run the backend on port 8080 and then run `npm run dev` in another shell. |
| 21 | `npm test` builds the frontend, runs Rust tests, starts the service, and runs the Playwright claim, mobile, rate-limit, and accessibility checks. |
| 5 | The frontend output is `dist/`. |
| 7 | The container listens on `PORT` (default `8080`). |
| 20 | It starts with no required configuration, creates a random privacy salt on first boot, and `/health` reports the build SHA. |
| 16 | Set `DATA_DIR` or `DATABASE_URL` only when you need to override the persisted SQLite location. |
| 10 | Each real quote gets independent random approval and owner tokens. |
| 12 | The owner token stays in that browser and authorizes export or deletion. |
| 11 | Private pages and API responses use `noindex` and `no-store` response policies. |
| 7 | Each fixed quote accepts one final decision. |
| 12 | API write bursts return `429` with `Retry-After` when they exceed the limit. |
| 7 | Free links retain records for 30 days. |
| 14 | A 365-day request succeeds only after the backend verifies a Studio license with Sociobot. |
| 25† >22 | The page offers the $29 checkout only while Sociobot reports that the product is available; existing license holders can restore a license at any time. |
| 4 | Build the root `Dockerfile`. |
| 14 | The image runs as the non-root `app` user and needs only `PORT` to start. |
| 24† >22 | For the factory Container Apps release, keep exactly one replica and mount its dedicated Azure Files share at `/durable`; set `DURABLE_DATA_DIR=/durable`. |
| 19 | The service writes a durable snapshot after each committed change, so a replacement process restores records before serving traffic. |
| 10 | Durable storage is an optional deployment override for local use. |
| 12 | The factory owns deployment, DNS, billing registration, and the production build SHA. |
| 3 | MIT. See LICENSE. |

Rewrite the long checkout sentence as `Studio checkout is available only when Sociobot reports availability. Existing license holders can restore a license.` Rewrite the long deployment instruction as `For the factory Container Apps release, use one replica. Mount its dedicated Azure Files share at `/durable` and set `DURABLE_DATA_DIR=/durable`.`

## Demo, sandbox, claims, and privacy checks

- Direct `/demo` has the visible `Demo — sample data, nothing is saved to your records` banner, `Reset demo`, and `Start for real`. In a same-browser-context check, the sample displayed and Reset produced a new sample; storage contained only `sessionStorage["demo:workspace"]`, not an `owner:` real-record key.
- The independent-process check fails as F-1-1 and takes precedence over that same-connection result.
- Browser request logs for a successful landing/demo flow contained only the product origin. No third-party runtime script, font, analytics, or advertising request was observed.
- From fresh clone `/tmp/qar-review-8xQLKG`, `npm ci` completed with 0 vulnerabilities. Every exact test command in `.factory/claims.json` passed: `demo-sandbox`, `pdf-receipt`, `record-control`, `first-party-only`, `retention-policy`, `private-links`, `single-decision`, `network-address-privacy`, `rate-limits`, `test:runtime-contract`, and `test:durable-snapshot`.
- `npm test` also passed locally: 16 Playwright tests passed. This confirms the local implementation but not the live distributed record path.

## Structure and history checks

- Landing has one H1, valid title, description, canonical, favicon, touch icon, social card, language, main landmark, skip link, footer Privacy/Terms, and a product-specific cassette/zine visual system. It is not a generic SaaS template.
- Internal public links (`/`, `/demo`, `/new`, `/privacy`, `/terms`, assets, sitemap, robots) and the Param Factory external link returned 200 when crawled. Titles, H1s, descriptions, canonicals, route focus, and back navigation work in the SPA check.
- `robots.txt` and `sitemap.xml` are present. The 404 returns HTTP 404 and has a return-home control, subject to F-1-2.
- The brief does not imply an AI-assisted step: preserving a fixed decision record needs deterministic capture, not generated content. Export and JSON record control already exist. No decorative AI feature or embedded provider key was found.

| Earlier record | Current confirmation |
| --- | --- |
| Verification 2/3: deployed state split breaks demo | **Unfixed/regressed — F-1-1.** The same `201` then `404` pattern occurred in every separate-process attempt. |
| Verification 2: sender cannot retrieve receipt/PDF | Source has management receipt/PDF links and `@regression:owner-receipt-link`; the fresh-clone full suite passes it. Live end-to-end confirmation is blocked by F-1-1, so local evidence is not deployment proof. |
| Verification 2: README runtime/durable claims unlisted | Current manifest includes `runtime-contract` and `durable-snapshot`, and exact commands pass. Remaining unlisted copy is F-1-3. |
| Verification 2: pinned Rust base image | Source uses `rust:1-slim`; this source finding is fixed. |
| Handoff/verification 4: 404 2px mobile overflow | **Unfixed — F-1-2.** Live measurement remains 392px at a 390px viewport. |

## What would make this perfect

Make the public demo reliably read the record it just creates across independent browser connections, and prove that at deployment level. Then remove or test each remaining claim, replace cassette metaphors with section/result names, correct per-route social metadata, and remove the two-pixel 404 overflow. Re-run this entire review with a fresh public browser process.
