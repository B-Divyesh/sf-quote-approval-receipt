# Independent verification 9 — FAIL

- **Candidate commit:** `7ba2db9388b15b702b1b3a4183d720c27387ed14`
- **Live URL:** <https://quote-approval-receipt.sociobot.in>
- **Verified:** 2026-08-29 UTC
- **Decision:** **FAIL — do not release.** The candidate passes its local suite, but the live service has two active replicas with replica-local SQLite, no durable mount, and replica-local rate limiters. Fresh real and demo records intermittently return 404 from the next browser request.

## Acceptance gates

### First read — PASS

A cold 1440 × 900 browser visit returned 200 and answered all three required questions in the first viewport:

- What it does: **“Record who approved your quote.”**
- Who it is for: contractors whose clients approve by email or chat, then change expectations.
- What to click first: **“Try it with sample data”**, next to “Loads a private sample quote. Nothing enters your records.”

The sample action is one click from the first screen. Cold load had no console or page errors and requested only same-origin resources. The standard factory `verify-url.sh` also passed: title present, `lang=en`, one h1, a main landmark, no missing image alt text, no unlabeled buttons, and no console errors. Evidence: [desktop screenshot](evidence/verification-9/screenshot-desktop.png), [390px screenshot](evidence/verification-9/screenshot-mobile.png), and [verify.json](evidence/verification-9/verify.json).

### Claims — all 13 commands PASS locally

After `npm ci` from the clean requested commit, every `.factory/claims.json` command was run literally and individually through the product's local demo/test entry point:

| Claim | Result |
| --- | --- |
| `demo-sandbox` | PASS |
| `quote-snapshot` | PASS |
| `pdf-receipt` | PASS |
| `record-control` | PASS |
| `first-party-only` | PASS |
| `retention-policy` | PASS |
| `studio-offer` | PASS |
| `private-links` | PASS |
| `single-decision` | PASS |
| `network-address-privacy` | PASS |
| `rate-limits` | PASS |
| `runtime-contract` | PASS |
| `durable-snapshot` | PASS |

The manifest maps 13 unique claims to exactly one observable test each. No unlisted product/README promise was found. The live deployment nevertheless contradicts `demo-sandbox`, `rate-limits`, and `durable-snapshot`; local claim success does not make the deployed topology safe.

## Clean-checkout quality gates

- `npm ci` — PASS; 30 packages installed, 0 vulnerabilities.
- `npm test` — PASS; claims manifest, production frontend build, 4 Rust tests, runtime contract, replica-split regression, durable replacement test, and all 20 Playwright tests.
- `npm run check` — PASS.
- `npm run build` — PASS; `dist/` produced.
- `cargo fmt --check` — PASS.
- `cargo clippy --all-targets -- -D warnings` — PASS.
- `npm audit --audit-level=high` — PASS; 0 vulnerabilities.
- Release Rust binary built through `test:runtime-contract` and the other runtime scripts.
- Docker was not available in this verifier container, so an image build was not repeated. The Dockerfile and passing runtime contract cover its build args, non-root user, `PORT` startup, generated salt, and health identity.

Local production output: 27.11 kB JavaScript / **8.96 kB gzip** and 15.55 kB CSS / **4.20 kB gzip**.

## Candidate and deployment identity

The live image and health identity match the requested candidate:

- Active revision: `sf-quote-approval-receipt--0000028`
- Image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:7ba2db9388b1`
- Traffic: 100% to the latest revision
- `GET /health`: `{"build_sha":"7ba2db9388b15b702b1b3a4183d720c27387ed14","durable_snapshot":false,"status":"ok"}`

Thus this is not a stale-build failure. It is the current candidate running with an unsafe deployment configuration.

## Release-blocking findings

### Critical — live quote and demo records are split and non-durable

Fresh Azure inspection found:

- `minReplicas: 1`, **`maxReplicas: 3`**;
- **2 active replicas** on revision `0000028` during verification;
- only `PORT=8080` in the container environment;
- no `DURABLE_DATA_DIR`;
- no volume mount and no Azure Files volume;
- public health reports **`durable_snapshot: false`**.

This violates the committed deployment contract, which requires exactly one replica, `/durable` mounted from `quote-approval-receipt-data`, and `DURABLE_DATA_DIR=/durable`.

Fresh user-visible reproductions:

1. In a desktop sender browser, a representative EUR quote with two items and 20% tax was accepted and the UI displayed **“Your approval link is ready”** with a valid 32-character public token.
2. Opening that link in a separate clean client browser immediately showed **“This page could not be loaded”**. The underlying share API returned 404, so the client could not review or decide on the newly created quote.
3. In a separate 390px browser, `POST /api/demo` created workspace `W5kL4g3oQaRHOB9CLIXEO6Pk`; the immediately following `GET /api/share/niYEF1…` returned 404 and logged `Failed to load resource: the server responded with a status of 404`.
4. Ten explicit cleanup requests for that demo workspace all returned 404. The creating replica's state was no longer reachable from those requests.

The checked-in `npm run test:live-topology` independently failed with `3 !== 1` for `maxReplicas`. A 20-session `test:live-demo` and one `test:live-review` run happened to pass earlier in the same verification, demonstrating that the defect is intermittent and routing-dependent, not absent.

This product exists to preserve defensible approval evidence. A deployment that intermittently loses a new quote between the sender and approver, and loses all records when a local replica is replaced, cannot ship.

### High — deployed per-client request allowances are multiplied by replica count

The server source allows 40 API reads or 15 API writes per client per second. Each active replica holds its own in-memory bucket:

- The checked-in live write test happened to observe the intended **15 successes, then 429**, with `Retry-After: 1`.
- A fresh 50-request read burst from one forwarded address produced **50 accepted requests and no 429**.
- A fresh 100-request read burst produced **80 accepted requests and 20 responses of 429** — two independent 40-request buckets.
- A fresh 40-request invalid-write burst, which created no records, produced **30 requests accepted by the limiter and 10 responses of 429** — two independent 15-request buckets.
- All observed 429 responses included `Retry-After: 1`.

The live allowance is therefore 80 reads and 30 writes while two replicas are active, rather than the documented 40/15 per client. It may rise again if the configured third replica starts. This independently fails the mandatory deployed rate-limit contract.

## Functional and edge-case coverage

The local end-to-end suite passed normal approval, request changes, international names, PDF text extraction, sender export/delete, a fresh sender receipt download, single-decision concurrency (`201` plus `409`), demo reset/exit, invalid currency, offline error recovery, and private response policies.

An additional local release-binary boundary probe passed:

- accepted minimum supported values;
- accepted 40 items, quantity 100,000, rate 10,000,000, and 100% tax;
- rejected 41 items, zero quantity, over-limit rate, tax above 100%, unsupported currency, forged 365-day retention, missing consent, invalid decision, one-character approver name, malformed email, and a 1,001-character note;
- every rejection used 422 or 403 with a specific recovery message;
- created boundary records were deleted.

The full real live workflow could not proceed beyond sharing because the newly created quote was unavailable to the clean approver context. That is the critical finding above.

## Accessibility, mobile, keyboard, and motion

- Independent Axe scans on `/`, `/new`, `/privacy`, `/terms`, the 404 route, and the demo found **0 serious/critical violations**.
- Every checked route has `lang=en`, one h1, and one main landmark.
- 390px layout has no horizontal overflow, including after 200% root text sizing.
- Interactive targets pass 44px sizing; 20px radio glyphs sit inside 358 × 52/70px clickable labels.
- Keyboard Tab starts on **“Skip to main content”**. Its visible focus is a 4px solid yellow outline, and Enter moves focus to `<main>`.
- Under `prefers-reduced-motion: reduce`, the media query matches, scroll behavior is `auto`, and no animation remains running.
- Cold public pages had no console or page errors. The demo emitted a 404 console error only when the replica-split failure occurred.

## Privacy, headers, routing, and links

- Cold landing and successful live review/demo request logs contained only `https://quote-approval-receipt.sociobot.in`; no analytics, advertising, CDN font, or third-party runtime script request occurred.
- Private API responses have `Cache-Control: no-store` and `X-Robots-Tag: noindex, nofollow, noarchive`.
- Responses include CSP with `frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `X-Frame-Options: DENY`, and a restrictive Permissions-Policy.
- Public `/`, `/demo`, `/new`, `/privacy`, `/terms`, `robots.txt`, and `sitemap.xml` return 200; an unknown route returns a real 404. The footer's external Sociobot link returns 200.
- `/privacy` and `/terms`, README, MIT license, demo contract, design thesis, and handoff files exist.
- The product does not require sign-in, is not a library/CLI, and does not register a service worker, so Entra, consumer-package, and PWA update/offline-reload checks do not apply.

## Performance and caching

Fresh throttled mobile Lighthouse:

- Performance **98**
- Accessibility **100**
- Best Practices **100**
- SEO **100**
- FCP **1.0 s**, LCP **1.6 s**, TBT **150 ms**, CLS **0**, Speed Index **1.0 s**
- Total transfer **120,766 bytes**, with no third-party resources

Live transfer sizes were about 9.4 kB JavaScript, 4.6 kB CSS, and 105 kB for the hero WebP. Hashed JS/CSS use `Cache-Control: public, max-age=31536000, immutable` and compressed responses.

## Required remediation

Redeploy the exact candidate with the repository's stated contract:

1. set `minReplicas: 1` and `maxReplicas: 1`;
2. mount Azure Files storage `quote-approval-receipt-data` at `/durable`;
3. set `DURABLE_DATA_DIR=/durable`;
4. confirm `/health` reports the exact candidate SHA and `durable_snapshot: true`;
5. rerun `test:live-topology`, multi-context real create/read/decision/delete, 20 isolated demos, and both 40-read/15-write limit probes.

Do not accept the release until durable cross-request state and deployment-wide request allowances both pass.
