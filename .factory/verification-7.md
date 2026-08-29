# Independent verification 7 — PASS

## Scope

- Candidate commit: `8f22bab8d72a2c9ea7bb6a19c44af86083fa589a`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29
- Result: **PASS**

This verification used the researched brief and factory work order as the
acceptance contract. No product code was modified.

## First-read result

Cold live load, desktop browser, HTTP `200`, no cached state:

> **Record who approved your quote.** For contractors whose client approves
> by email or chat, then changes expectations later. **Try it with sample
> data** — loads a private sample quote and does not enter your records.

It plainly answers what it does (records a named quote approval), who it is
for (contractors dealing with off-portal approval), and what to click first
(the visible, one-click sample action). The first screen also shows the three
plain facts: 30-day free retention, per-record export/deletion, and a PDF
receipt after a decision. This passes the plain-words and demo-sandbox gate.

## Required claims

From a clean candidate checkout, `npm ci` completed successfully. Every
exact command listed in `.factory/claims.json` was invoked from the demo
entry-point test setup and completed successfully after dependency install.

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

The initial attempt before `npm ci` stopped at `tsc: not found`. That is the
expected missing clean-clone dependency state, not a claim failure after the
required installation step. A later full `npm test` passed all claims and all
non-claim tests.

## Local quality gates

Passed:

```text
npm ci
npm test
VITE_BUILD_SHA=8f22bab8d72a2c9ea7bb6a19c44af86083fa589a npm run build
npm run check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
```

`npm test` passed 4 Rust tests, the only-`PORT` runtime contract, durable
replacement test (20 committed records), and 20 Playwright tests. The exact
candidate frontend build generated `index-3X5an3I2.js` at 27.10 kB / 8.96 kB
gzip and CSS at 15.55 kB / 4.20 kB gzip. The WebP hero is 104,994 bytes.

The verifier environment contains no Docker-compatible builder (`docker`,
`podman`, and `buildah` are absent), so the Docker image build could not be
executed here. The Dockerfile was inspected, and its frontend/backend build
arguments were reproduced directly. `BUILD_SHA` was compiled into the release
binary and `VITE_BUILD_SHA` into the frontend.

## Deployment identity and backend checks

Live `/health` returned:

```json
{"build_sha":"8f22bab8d72a2c9ea7bb6a19c44af86083fa589a","durable_snapshot":true,"status":"ok"}
```

The SHA-256 of the live `index-3X5an3I2.js` exactly matched the output of the
candidate source built with that candidate `VITE_BUILD_SHA`. This is fresh
evidence that live is the candidate.

`npm run test:live-topology` passed the required one-replica, durable-state
deployment contract. With the exact live URL and expected SHA:

- `npm run test:live-demo`: 20/20 fresh Chromium demo contexts each created
  a workspace (`201`), read its share (`200`), and removed it on exit (`404`).
- `npm run test:live-rate-limit`: 15 concurrent writes succeeded; the 16th
  was `429` with `Retry-After: 1`; all test workspaces were deleted.
- `npm run test:live-review`: routes, metadata, mobile layout, same-origin
  demo, cleanup, and Axe checks passed.

The observed documented server-side write allowance is therefore **15 requests
per client address per one-second window**, with **`Retry-After: 1`** on the
next request.

## End-to-end and negative-path exercise

In a fresh live `/demo` browser context at 390 px:

1. Clicking **Record this decision** with missing required fields gave the
   announced recovery message: “Complete each required field, then record the
   decision.”
2. Filling `Taylor Okafor`, `Operations director`, selecting **Approve quote**,
   and confirming consent produced a receipt at `/receipt/<id>?demo=1` with
   the name, title, approved decision, fixed quote, UTC time, consent, hash,
   and downloadable PDF.
3. **Start for real** removed the demo storage pointer and made the old share
   endpoint return `404`.

The full claim suite additionally exercised malformed currency (`422`),
forged 365-day retention (`403`), concurrent final decisions (`201` + `409`),
private-token shape/independence, receipt PDF text including international
names, owner export/delete, no network-address disclosure, offline failures,
and durable restart recovery.

## Privacy, headers, caching, and bundle checks

The fresh browser request log through landing, demo, decision, receipt, and
exit contained only the product origin. There were no console errors or page
errors. The only fetched landing resources were first-party HTML, hashed JS,
hashed CSS, `/api/studio`, and the first-party cassette image.

Observed live response policy:

- CSP: self-only script/style/image defaults, self plus Sociobot only for
  declared connections, and `frame-ancestors 'none'` as a response header.
- `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`,
  `X-Frame-Options: DENY`, and a restrictive permissions policy.
- Private approval/API responses: `X-Robots-Tag: noindex, nofollow, noarchive`
  and `Cache-Control: no-store`.
- Hashed JS/CSS: `Cache-Control: public, max-age=31536000, immutable`.

This supports the stated no-analytics/no-third-party-runtime claim and the
private-link indexing/retention protections.

## Accessibility and responsive review

The live review covered landing, builder, demo, privacy, terms, and 404 at
390 px. There was no horizontal overflow, including at 200% text zoom. The
first keyboard Tab focused the skip link, which visibly renders a 4 px mustard
focus outline. Keyboard navigation, Enter activation, history/back behavior,
route heading focus, labels, form alert, semantics, and reduced-motion behavior
passed. Playwright AxeBuilder reported zero serious or critical findings on all
audited public screens. At reduced motion, the running animation count was 0.

Fresh visual evidence:

- `.factory/evidence/verification-7/landing-desktop.png`
- `.factory/evidence/verification-7/landing-mobile.png`
- `.factory/evidence/verification-7/demo-mobile.png`
- `.factory/evidence/verification-7/404-mobile.png`
- `.factory/evidence/verification-7/live-review.json`

## Defects

### P3 — non-reproduced mobile test flake

The first full `npm test` after clean setup reported
`@mobile 390px pages reflow at 200% text and controls meet target size` as
failed. A focused rerun passed, as did the next complete `npm test` run. The
fresh live 390 px/200% audit also found no overflow. No product behavior
failure was reproducible, so this does not block acceptance; the test should
nevertheless be monitored for recurrence.

No P0, P1, or P2 defects were found.
