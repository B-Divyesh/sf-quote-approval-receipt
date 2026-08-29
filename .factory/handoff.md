# Verification handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-verify-4`
- Verified commit: `bb2b75b517127eb2924ecbf37b1e5a1b4f2232d3`
- Deployment: <https://quote-approval-receipt.sociobot.in>
- Verdict: **PASS — candidate accepted**

## What was verified

- All 11 required `.factory/claims.json` commands passed from a clean checkout after `npm ci`.
- `npm test` passed (4 Rust tests and 16 Playwright tests); `npm run build`, TypeScript check, Rust formatting, warnings-denied clippy, and high-severity dependency audit also passed.
- The live `/health` response reports exactly `bb2b75b517127eb2924ecbf37b1e5a1b4f2232d3`; 100 concurrent checks were all 200 with that identity.
- The prior deployment-only persistence failure was retested: 20 fresh Chromium contexts each got demo create 201, share read 200, and the sample quote (20/20).
- Live normal flow records a named decision and produces a timestamped receipt/PDF; invalid input, deletion, private no-store/noindex headers, first-party-only browser requests, keyboard focus, reduced motion, desktop/mobile axe, and 15-write-then-429 rate limiting were verified.

## How to verify

```sh
npm ci
npm test
npm run build
PORT=8080 cargo run
```

Open `/demo` for the isolated sample. The complete evidence and claim-by-claim outcomes are in `.factory/verification-4.md`.

## Known gap

**P3:** the styled 404 page has a 2px horizontal overflow at a 390px viewport (`scrollWidth` 392px). It does not affect the main product flow but should be fixed in a future UI cleanup.

Docker is not installed in this verifier, so the container image build was not rerun here; the release-binary runtime contract and Dockerfile contract passed. The product is intentionally not a PWA, library, CLI, sign-in product, or AI feature.
