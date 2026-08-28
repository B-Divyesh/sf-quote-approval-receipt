# Independent product verification 2 — FAIL

- Work order: `quote-approval-receipt-verify-2`
- Candidate: `c3ad810fd955a309b6fc7af2857b4d770283caf7`
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-28 UTC
- Verdict: **FAIL — do not release**

The candidate builds and passes its local suite, and the exact candidate assets are live. Production still splits records across serving instances, however. The required one-click demo failed on every fresh attempt. A separate product defect also prevents a sender from retrieving the promised PDF after the client decides.

## Release blockers

### Critical — production state is still split across serving instances

Fresh browser contexts consistently created a demo on one live state domain and read it from another:

- 20/20 fresh `/demo` visits: `POST /api/demo` returned 201, then the immediate `GET /api/share/<new-token>` returned 404.
- Every attempt rendered **The demo could not start** and logged the failed 404 request.
- Twelve fresh HTTP/1.1 reads of one newly created token returned `404, 404, 200, 404, 200, 404, 404, 404, 200, 404, 200, 404`.
- Two other real/demo tokens likewise alternated between 200 and 404 across fresh connections.

This reproduces the earlier deployment-only defect from fresh evidence. It breaks the mandatory one-click demo and makes quote, owner, decision, receipt, export, and deletion continuity dependent on which live instance receives the request. The local `@claim:demo-sandbox` test cannot detect this because it runs one local process.

### High — the sender cannot retrieve the PDF after a decision

A fresh live record was created, approved, and then opened through its owner management key in a separate sender context.

- The management page said **Open the approval link to see the recorded receipt**.
- Its only link was **Export record as JSON**; there was no receipt or PDF link.
- Opening the approval link cold showed **A decision is already recorded** and exposed no links or receipt ID.
- The receipt API and PDF existed when addressed by the receipt UUID returned only to the deciding client.

The target user is the contractor who needs the defensible record. Requiring the client to preserve and send back an undiscoverable receipt URL does not complete that job. The JSON export is not the promised timestamped PDF workflow.

### High — public README claims remain outside the claims manifest

All declared claims have a tagged test, but `.factory/claims.json` does not list several concrete README promises:

- first boot creates a random privacy salt and needs no configuration;
- the container listens on `PORT` and `/health` reports the build SHA;
- committed SQLite changes are atomically snapshotted to durable storage;
- the image runs as a non-root user and needs only `PORT`.

The claims contract says an unlisted README claim fails review. The durability statement is especially material because live behavior currently contradicts the intended one-replica continuity.

### High — Dockerfile violates the mandatory backend build contract

The backend contract explicitly forbids pinning a Rust minor image, but the candidate uses `FROM rust:1.88-bookworm`. It must use a moving stable tag such as `rust:1-slim` or `rust:1-alpine`. No Docker or Podman daemon was available for an image build; the exact native release build passed.

## Mandatory first-read gate

The cold landing screen passes on desktop and 390px mobile:

- What: **Record who approved your quote**.
- Who: contractors whose clients approve by email or chat.
- First click: **Try it with sample data**, next to an explanation of what loads.
- At 390×844, the headline, audience sentence, sample action, and all three facts are above the fold.

The click itself fails in production as described above, so the candidate fails the overall demo gate.

## Claims run first from the clean candidate

After `npm ci`, every command in `.factory/claims.json` was run verbatim and passed locally:

| Claim | Result | Observable assertion |
| --- | --- | --- |
| `demo-sandbox` | PASS local / FAIL live | isolated demo and 16 repeat reads locally; fresh live create/read fails |
| `pdf-receipt` | PASS | extracted Unicode identity, quote, client, UTC time, hash, and consent |
| `record-control` | PASS | owner export, delete, then share 404 |
| `first-party-only` | PASS | landing and demo requests are same-origin |
| `retention-policy` | PASS | 30-day/24-hour windows and forged 365-day rejection |
| `private-links` | PASS | independent token shapes plus noindex/no-store |
| `single-decision` | PASS | concurrent decisions return 201 and 409 |
| `network-address-privacy` | PASS | address absent from receipt and export; salted hash unit test |
| `rate-limits` | PASS | write burst returns 429 and `Retry-After` |

The first literal pre-install invocation stopped at `tsc: not found`, as expected in a dependency-free clone. The locked install then completed with 31 packages and zero vulnerabilities, and all nine declared commands passed.

## Local build and runtime gates

- `npm ci`: PASS; 31 packages, zero vulnerabilities.
- `npm test`: PASS; 4 Rust tests and 15 Playwright tests.
- `npm run check`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm audit --audit-level=high`: PASS; zero vulnerabilities.
- `npm run build`: PASS; `dist/` produced.
- `BUILD_SHA=c3ad810fd955a309b6fc7af2857b4d770283caf7 cargo build --release`: PASS.
- Release binary with only `PORT`: PASS. It logged `privacy_salt=generated`, then `privacy_salt=persisted` after restart; a record remained readable.
- Local invalid cases rejected: zero quantity, 41 items, unsupported currency, tax over 100%, short consent, and forged 365-day retention.
- Forty maximum-sized items were accepted. Missing owner authorization returned 401. Concurrent final decisions returned 201/409.
- Container build: not run because the verifier image has no Docker/Podman daemon.

## Live functional and backend evidence

- Invalid cases returned the expected 422 responses; forged 365-day retention returned 403.
- A 40-item boundary quote was accepted and deleted; an unauthenticated owner read returned 401.
- Concurrent decisions returned one 201 and one 409.
- A successful demo decision preserved `José Núñez — Direção`; PDF.js extracted the identity, `NS-2048`, timestamp, hash, and consent from a valid 3,200,084-byte PDF.
- A real creator flow recovered from zero-quantity validation and created a two-item taxed quote.
- 100 concurrent `/health` requests returned 100×200 in 446 ms.

## Deployment identity

- `/health` reports `c3ad810fd955a309b6fc7af2857b4d770283caf7`.
- Live and local SHA-256 hashes match exactly:
  - HTML: `548cca7d04e362b5bb8e2268b4d1fbcddf13df3a42b61a031b8cfe58a2577737`
  - JS: `ec119988d349712e9dc128c8b0e57ab5599569f6828c9bfcbd9af07de7818afa`
  - CSS: `f04147a4644f7e54722cb9a9aeb90da674963d0fdc0137eb23116cb301df15f9`

This is not a stale-code failure. The exact candidate is live with incorrect production state continuity.

## Rate limiting

- Product writes: 20-request burst → 15×201, 5×429, `Retry-After: 1`.
- Product GETs: 50-request burst → 40×404, 10×429, `Retry-After: 1`.
- Sociobot product verification: 100-request burst → 30×200, 70×429, `Retry-After: 4`.
- Sociobot CORS allowed only the tested product origin. `/health` is exempt as permitted.

## Privacy, headers, and routing

- A successful landing→demo→decision flow made only same-origin browser requests.
- Demo storage contained only `sessionStorage["demo:workspace"]`; local storage stayed empty.
- API/private responses used `Cache-Control: no-store` and `X-Robots-Tag: noindex, nofollow, noarchive`.
- CSP, `nosniff`, `no-referrer`, frame denial, and restrictive Permissions Policy were present.
- Hashed JS/CSS used one-year immutable caching and text compression.
- `/missing-page` returned a styled HTTP 404. Public internal routes and `sociobot.in` returned 200.
- `robots.txt`, `sitemap.xml`, favicon, touch icon, and social card returned 200.
- No analytics, advertising, CDN fonts, embedded keys, sign-in, service worker, library, or CLI surface applies.

## Accessibility, mobile, and performance

- `/opt/fleet/lib/verify-url.sh`: PASS; 640 ms, title/lang/main/alt checks pass, zero landing console errors.
- Axe: zero serious/critical findings on `/`, `/new`, `/privacy`, `/terms`, styled 404, mobile landing, and a successful receipt.
- The failed live demo is an application error, not an axe violation.
- Keyboard order reached skip link, wordmark, navigation, and sample action; every focused element had a visible 4 px yellow outline. Enter opened `/demo` and route focus moved to its H1.
- Reduced motion: media query active and zero running animations.
- 390px: no horizontal overflow at normal or 200% root text size; no visible interactive target below 44 px.
- Bundle: JS 25,721 bytes raw / 8,780 gzip; CSS 15,456 / 4,190; hero WebP 104,994 bytes.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.6 s, TBT 50 ms, CLS 0; 118 KiB transferred.
- Visual review: the cassette/zine system is coherent and product-specific on desktop and mobile.

## Required before re-verification

1. Ensure every live request uses one durable shared state domain; repeat at least 20 fresh browser create→read demo pairs across new connections.
2. Give the owner management screen a direct receipt and PDF link after a decision; test from a fresh sender context.
3. Add tagged claim tests for the remaining README runtime and durability statements.
4. Replace the pinned Rust minor Docker base with the mandatory stable-major tag and build the image from a `.git`-free source context.
