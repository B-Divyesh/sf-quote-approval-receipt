# Polish round 1 — finding closure

- Candidate reviewed: `bb2b75b517127eb2924ecbf37b1e5a1b4f2232d3`
- Repair code: `3222faf`; deployment configuration: `4bbf55e`
- Live check: <https://quote-approval-receipt.sociobot.in>
- Evidence directory: `/tmp/qar-live-evidence.UNJ2Wt`

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1; verification, verification-2, verification-3 state split | Deployed one Container Apps replica with the existing Azure Files share mounted at `/durable` and `DURABLE_DATA_DIR=/durable`; retained atomic durable snapshots. Added `scripts/live-demo-check.mjs`. | Live revision `0000016`; `/health` build `3222faf`; `npm run test:live-demo` 20/20 independent Chromium processes created demo `201`, read share `200`, and showed `Half-day product shoot`. |
| F-1-2; verification-4 P3 404 overflow | Added clipping to the 404 recovery panel and a 390px regression assertion. | Browser test `each public route updates social metadata and the 404 stays within a 390px viewport`; live `scrollWidth <= innerWidth`; screenshot `/tmp/qar-live-evidence.UNJ2Wt/404-mobile.png`. |
| F-1-3 snapshot claim | Replaced the ambiguous preview sentence with `Paste the agreed scope and totals, then make an approval link.` and added the observable `quote-snapshot` claim. | `@claim:quote-snapshot` passed from clean clone. |
| F-1-3 untestable regulated-service statement | Removed the regulated-status assertion and used `Use this tool to record a quote decision.` | Landing copy audit and clean-clone browser suite. |
| F-1-3 Studio availability, price, and restore statement | Replaced the paused-purchase wording with neutral availability status; added recorded available/unavailable response coverage and a real license restore flow. | `@claim:studio-offer` passed from clean clone. |
| F-1-3 generated-artwork footer assertion | Removed the public provenance assertion; provenance remains in `design.md`. | Landing footer check in browser suite. |
| F-1-4 decorative labels and unclear restore control | Renamed labels to `Example approval receipt`, `How approval links work`, numbered actions, and `Enter a Studio license`; removed the decorative first eyebrow. | `.factory/copy-audit.md`; live first-screen screenshot `/tmp/qar-live-evidence.UNJ2Wt/screenshot-mobile.png`. |
| F-1-5 shared social metadata | `pageMeta` now updates Open Graph and Twitter title/description alongside title, description, canonical, and robots. | Browser route metadata test; live checks on demo, privacy, terms, builder, and 404. |
| Demo-sandbox requirement | Made `/?demo=1` the first-screen action and direct entry; `/demo` stays an alias. Existing banner/reset/start-real controls remain isolated in session storage and backend workspace. | `@claim:demo-sandbox` passed in clean clone; live demo screenshot `/tmp/qar-live-evidence.UNJ2Wt/demo-mobile.png`. |
| verification sender receipt/PDF retrieval | Preserved the already-fixed owner management receipt and download actions. | `@regression:owner-receipt-link` passed in the 19-test clean-clone suite. |
| verification claims, runtime, and durable-state coverage | Preserved and re-ran the runtime/durable claims; every public claim has exactly one tagged test or standalone command. | All 13 manifest entries passed from clean clone; 4 Rust unit tests passed. |
| verification Docker base contract | Preserved `rust:1-slim`, multi-stage build, non-root runtime, and build-arg SHA. | Successful ACR build `chr8` from a `.git`-excluded tarball. |
| verification checkout retention, Unicode PDF, currency, mobile targets, headers, 404 status, startup logging | Rechecked prior source repairs; no regression was introduced. | Clean-clone `npm test`/19 browser tests, Rust tests, cache/header test, mobile reflow test, runtime contract, and live 404 HTTP 404 check. |

## Final live quality checks

- `verify-url.sh`: pass, zero console errors; screenshot paths above.
- Playwright axe: zero serious/critical on all public routes listed in the handoff.
- Lighthouse mobile: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; report `/tmp/qar-live-evidence.UNJ2Wt/lighthouse.json`.
- No external browser requests were observed in the first-party claim flow. The application contains no AI feature because the brief's fixed-record workflow is deterministic.
