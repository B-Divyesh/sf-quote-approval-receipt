# Verification 12 — PASS

- Candidate commit: `fe0b3a58c7c52180ae1a46f3e2012529860c7c92`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Verified: 2026-08-29 UTC
- Release decision: **PASS**

## First read

Cold on a 390 px browser, the landing page says: **“Record who approved your
quote.”** It identifies the audience and situation: “For contractors whose
client approves by email or chat, then changes expectations later.” The first
action is the visible **“Try it with sample data”** link; its adjacent note says
it loads a private sample and that nothing enters the visitor's records. The
first screen also shows three concrete facts: 30-day free retention, per-record
export/deletion, and a PDF after a decision. This meets the plain-words and
one-click demo acceptance requirement.

## Clean local verification

`npm ci` installed 30 packages with no audit vulnerabilities. The clean
`npm test -- --grep @claim:demo-sandbox` run passed its complete prerequisite
chain (claims manifest, deployment contract, production build, Rust tests,
runtime contract, replica regression, durable-state test) and the selected
demo claim. The remaining browser claim tests were then run against the shipped
demo harness; all passed:

| Claim IDs | Result |
| --- | --- |
| `demo-sandbox`, `quote-snapshot`, `pdf-receipt`, `record-control`, `first-party-only` | PASS |
| `retention-policy`, `studio-offer`, `private-links`, `single-decision`, `network-address-privacy` | PASS |
| `rate-limits`, `runtime-contract`, `durable-snapshot` | PASS |

`npm test` then passed in full: 13 unique claim mappings, the deployment
regression check, the production `dist/` build, four Rust unit tests, runtime,
replica and durable-state checks, and 22 Playwright tests. `npm run check`,
`cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings` passed.

The exact frontend production build passed and produced `dist/` with 27.11 kB
JavaScript (8.96 kB gzip) and 15.55 kB CSS (4.20 kB gzip), within the 200 kB /
50 kB budgets. A local Docker build could not be run because this verifier
container has no `docker` executable; the checked `Dockerfile` is multi-stage,
uses `rust:1-slim`, has a default `BUILD_SHA`, and runs the final image as a
non-root user. The live service supplied independent production evidence below.

## Live independent verification

- `/health` reported the requested build SHA
  `fe0b3a58c7c52180ae1a46f3e2012529860c7c92` and durable snapshot mode.
- `npm run test:live-topology` passed: one active revision, one ready replica,
  100% traffic, mounted `/durable` Azure Files storage, and public durable
  health signal.
- `npm run test:live-workflow` passed across separate sender/client contexts:
  create a quote, read its fixed snapshot, make a named approval, retrieve the
  receipt as owner, and delete the record. This confirms the brief's core job
  end to end on the deployment.
- Manual UI coverage confirmed native required-field recovery (“Please fill out
  this field”) and created the expected private-link-ready screen for a
  zero-value item/tax boundary quote. The automated flow exercised a normal
  non-zero quote, named external approver, PDF receipt, export/deletion, and a
  concurrent second decision returning conflict.
- The demo review passed at `/?demo=1` through the real page: sample quote
  loads, its banner is visible, Reset replaces its workspace, and Start for
  real deletes the demo workspace (its old approval URL returned 404).
- `npm run test:live-demo` passed 20/20 isolated 390 px Chromium sessions:
  each observed demo creation `201`, immediate private sample read `200`, and
  exit cleanup `404`. The first session also verified Reset cleanup.
- Read limiting admitted exactly 40 concurrent requests; request 41 returned
  `429` with `Retry-After: 1`. Invalid writes admitted exactly 15; request 16
  returned the same `429` and retry guidance.
- Desktop and 390 px checks passed. At 390 px there was no horizontal overflow,
  the primary target measured 304 × 51 px, and the full suite verified 200%
  text reflow and 44 px controls. Keyboard-only testing reached the visible
  skip link, moved focus to `main`, and activated the demo with Enter. Reduced
  motion had no running animations.
- Playwright Axe found zero serious or critical violations on all public
  routes. `verify-url.sh` passed with `lang=en`, one H1, main landmark, no
  missing alt text or unlabeled buttons, and no browser errors.
- Browser request logging across landing and demo found only
  `https://quote-approval-receipt.sociobot.in`; no third-party runtime request
  loaded. The deployment returned CSP with `frame-ancestors 'none'`, `nosniff`,
  `no-referrer`, frame denial and restricted permissions. Hashed JS/CSS are
  one-year immutable cached. Private-route noindex/no-store behavior passed the
  claim test.

Evidence: [live review JSON](evidence/verification-12/live-review.json),
[desktop landing](evidence/verification-12/landing-desktop.png),
[mobile landing](evidence/verification-12/landing-mobile-manual.png), and
[URL verifier report](evidence/verification-12/verify-url/verify.json).

## Defects

No Critical, High, Medium, or Low product defects found.

## Notes

- This is a web-with-backend product, not a library, CLI, PWA, or sign-in
  product. Consumer install, service-worker/offline-reload, and Entra identity
  checks do not apply.
- A Docker daemon/binary is unavailable in this disposable verification
  environment, so only that local image-build command was not executable. It
  is not a product failure given the exact live build identity, runtime
  contract, and topology evidence.
