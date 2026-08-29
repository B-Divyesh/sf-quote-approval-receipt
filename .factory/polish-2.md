# Polish round 2 — cumulative finding closure

- Candidate reviewed: `f08adbf88078b184adacf6eedb081c9c234855a8`
- Functional repairs: `0d501ac`, `8055c4c`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Live revision/build: `sf-quote-approval-receipt--0000019` / `8055c4c`
- Final clean-claim evidence: `/tmp/qar-polish2-final-clean.7yl07d/` at `ecff4e5`
- Live evidence: `/tmp/qar-polish2-live.yP6XGH/`

## Adversarial review findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 — deployed state split | Preserved the atomic durable snapshot and restored the required one-replica `/durable` Azure Files deployment. Extended the independent-browser check to delete each sample after reading it. | `live-demo.log`: 20/20 separate Chromium processes saw create 201, read 200, sample visible, and cleanup 404. Live revision `0000019`; `/health` build `8055c4c`. |
| F-1-2 — 404 mobile overflow | Kept `.not-found` clipping and the 390 px regression. The live preflight also found and fixed a separate hidden demo-banner overflow, then added demo to the 200% reflow matrix. | Test `each public route updates social metadata and the 404 stays within a 390px viewport`; `@mobile`; `404-mobile.png`; `live-review.json` reports `overflow: false`. |
| F-1-3 — unlisted public claims | Preserved the tested snapshot, Studio, PDF, retention, privacy, runtime, and durability claims. Expanded `demo-sandbox` to cover reset and exit deletion. | All 13 manifest commands passed from the clean clone; individual `claim-*.log` files. `@claim:demo-sandbox` observes both deleted workspaces returning 404. |
| F-1-4 — decorative/unclear labels | Preserved the plain landing labels and changed remaining route lore: `Quote decision receipt`, `Request error`, and direct legal wording. | `.factory/copy-audit.md`; `landing-mobile.png`; full browser suite. |
| F-1-5 — shared social metadata | Preserved per-route title, description, canonical, Open Graph, and Twitter updates. | Test `each public route updates social metadata…`; live `live-review.json` covers home, builder, privacy, terms, and 404. |
| F-2-1 — demo pointer retained on exit | Added transactional `DELETE /api/demo/:workspace`. Reset deletes before reseeding; all demo route exits delete the workspace and clear demo-only session keys. | `@claim:demo-sandbox`; `live-demo.log` 20/20; `live-review.json` shows `deletedStatus: 404`, `demoKeys: []`; `demo-mobile.png`. |
| F-2-2 — metaphorical 404 H1 | Replaced `This page is not on the record` with `This page was not found`; retained the product-specific cassette illustration. | Route-copy assertion in `tests/product.spec.ts`; live `404-mobile.png`; live HTTP 404 and exact H1 in `live-review.json`. |

## Earlier independent-verification findings

Earlier reports used severity headings rather than finding IDs. These stable labels map every recorded defect.

| Finding | Change/recheck | Evidence |
| --- | --- | --- |
| V-1-Critical / V-2-Critical / V-3-Critical — split production state | One durable writer with atomic snapshots and Azure Files; 20 independent create/read/delete cycles. | `test:durable-snapshot`; `live-demo.log`; deployed scale/mount recorded in handoff. |
| V-1-High-Studio — dead/bypassable paid retention | Backend verifies Studio before 365-day retention; unavailable checkout is not shown; restore remains available. | `@claim:studio-offer`, `@claim:retention-policy`; clean claim logs. |
| V-1-High-PDF — international text corruption | Unicode-capable DejaVu PDF output retained. | `@claim:pdf-receipt` extracts `José Núñez — Direção`, time, hash, consent, and quote snapshot. |
| V-1-High-Claims / V-2-High-README-claims | Thirteen claim entries cover every material landing/README promise, including runtime and durable replacement. | Every exact manifest command passed from clean clone. |
| V-1-Medium-Validation | Unsupported currency and invalid numeric/item bounds are rejected with actionable responses. | Browser test `invalid currencies fail cleanly…`; Rust/full integration suite. |
| V-1-Medium-Reflow-targets | Responsive guardrails, 44 px targets, 390 px and 200% text coverage retained; demo added to the matrix. | `@mobile`; `landing-mobile.png`, `demo-mobile.png`, `404-mobile.png`. |
| V-1-Medium-Cache/privacy | Private API/routes remain `no-store`/`noindex`; hashed assets remain compressed and immutable. | `@claim:private-links`; `invalid currencies…response policies`; live review and header checks. |
| V-1-Low-404-status | Unknown routes return a styled real HTTP 404. | Browser response test; live `/missing-review-route` returned 404. |
| V-1-Low-manage-robots | Management, approval, receipt, and API responses retain response-level noindex. | `@claim:private-links`; live header audit. |
| V-1-Low-startup-log | Startup reports generated/persisted privacy salt and build identity without exposing the salt. | `npm run test:runtime-contract`; clean `claim-runtime-contract.log`. |
| V-2-High-owner-PDF | Sender management shows direct receipt and PDF controls after a decision. | `@regression:owner-receipt-link` opens a fresh sender context and downloads `approval-receipt.pdf`. |
| V-2-High-Docker | Moving `rust:1-slim`, default build arg, non-root runtime, no `.git` dependency retained. | Runtime contract; ACR build `chsv` from `.git`-excluded source. |
| V-3-rate-limit-topology evidence | Single live limiter allowance restored. | Live burst: 15×405, 5×429, `Retry-After: 1`; one deployed replica. |
| V-4-P3 — 404 2 px overflow | 404 container clips its illustration/shadow and is regression-tested at 390 px. | `@mobile`; `404-mobile.png`; live `scrollWidth <= innerWidth`. |

## Final live checks

- The first screen shows the job, audience, action, result note, and three facts above the fold at 390×844.
- `/`, `/new`, `/privacy`, `/terms`, `/?demo=1`, `/demo`, assets, robots, sitemap, and the factory link return 200; the missing route returns 404.
- Axe found zero serious/critical issues on every checked route. Focus, reduced motion, labels, H1 count, landmarks, alt text, and 44 px targets pass locally.
- Browser traffic through landing and demo used only `https://quote-approval-receipt.sociobot.in`.
- Lighthouse mobile: 99 Performance, 100 Accessibility, 100 Best Practices, 100 SEO.
- Visual inspection of the three screenshots confirms the cassette/zine identity remains intact on mobile.
