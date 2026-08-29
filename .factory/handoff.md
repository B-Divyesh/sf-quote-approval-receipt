# Polish round 2 handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-polish-2`
- Reviewed candidate: `f08adbf88078b184adacf6eedb081c9c234855a8`
- Functional repair: `0d501ac`; mobile follow-up: `8055c4c`
- Live revision: `sf-quote-approval-receipt--0000019`
- Live build identity: `8055c4c`
- URL: <https://quote-approval-receipt.sociobot.in>
- Verdict: **ready — every review finding is closed**

## What changed

The demo now has a real destruction lifecycle. `DELETE /api/demo/:workspace` removes its quote and decision in one transaction and persists the durable snapshot. Reset deletes the old workspace before seeding another. Leaving any in-app demo route, including **Start for real**, deletes the server workspace and clears only demo session data. A claim test proves both former share URLs return 404.

The 404 H1 now says `This page was not found`. Remaining unclear cassette metaphors were removed from receipt, error, and legal copy while the cassette-era zine visual system was preserved. The demo banner and 404 reflow at 390 px and 200% text. Route titles, social metadata, canonical URLs, focus, HTTP 404 status, legal links, and public link health were rechecked live.

The catalog description is now the verb-first 70-character line `Record who approved a quote and issue a named, timestamped PDF receipt.` The claims manifest and demo documentation describe reset and exit deletion.

## Verification

- Clean clone: `/tmp/qar-polish2-clean.uKb4BB/repo` at `0d501ac`; `npm ci` found 0 vulnerabilities. Every one of the 13 exact commands in `.factory/claims.json` passed. Logs are `/tmp/qar-polish2-clean.uKb4BB/claim-<id>.log`.
- Full local suite: `npm test` passed 4 Rust tests, runtime-only startup, durable replacement, and 19 Playwright tests. The browser suite covers the complete quote/PDF path, owner retrieval, Unicode, concurrent decisions, validation, privacy headers, keyboard focus, reduced motion, all public routes, Axe, 390 px, and 200% text.
- Static checks: `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `npm audit --audit-level=high`, and `git diff --check` passed.
- Build: `npm run build` produced `dist/`. JS is 27.01 KB raw / 8.96 KB gzip; CSS is 15.48 KB raw / 4.20 KB gzip; hero WebP is 104,994 bytes.
- Container: ACR build `chsv` passed from a `.git`-excluded context using `rust:1-slim`; image digest is `sha256:a8553148c7a0f0ffc45c7b023a632e2859755406567c00c4216f75ec97beda9d`.
- Deployment: one replica, `/durable` Azure Files mount, `DURABLE_DATA_DIR=/durable`, image `8055c4c`. Live write burst was 15×405 then 5×429 with `Retry-After: 1`; 100 concurrent health requests were 100×200 in 222 ms.
- Live demo: `npm run test:live-demo` passed 20 separate Chromium launches. Every launch observed create 201, immediate read 200, visible sample data, exit deletion, empty demo storage, and old share 404.
- Live browser/a11y/privacy: `verify-url.sh` passed at 610 ms with zero landing console errors. `npm run test:live-review` passed `/`, `/new`, `/privacy`, `/terms`, demo, and HTTP 404 with zero serious/critical Axe findings and only same-origin requests.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.6 s, TBT 90 ms, CLS 0, 118 KiB transferred.
- Evidence: `/tmp/qar-polish2-live.yP6XGH/` contains `live-review.json`, `live-demo.log`, `verify.json`, `lighthouse.json`, and desktop/mobile screenshots.

## Run and verify

```sh
npm ci
npm test
npm run build
LIVE_URL=https://quote-approval-receipt.sociobot.in npm run test:live-demo
LIVE_URL=https://quote-approval-receipt.sociobot.in EVIDENCE_DIR=/tmp/qar-live npm run test:live-review
```

## Known gaps and next steps

None. Offline operation is not claimed and is not appropriate for this backend record service. AI is not used because quote-decision capture is deterministic. Studio checkout remains conditional on Sociobot product availability as stated and tested.
