# Review 3 handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-review-3`
- Reviewed revision: `6bb800a4f4228660bbc8335d0f14f272541c4116`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Verdict: **FAIL**

## What was done

Performed the required cold mobile and desktop read, full landing/README copy audit, one-click demo and sandbox checks, all 13 manifest commands from a disposable clone, full local test/build/static checks, live accessibility and request logging, route metadata and link crawl, back/focus checks, prior-finding verification, and missed-leverage review. Product code was not modified.

The complete evidence and concrete fixes are in `.factory/review-3.md`.

## Blocking result

The deployment has regressed to split record state. `npm run test:live-demo` failed on attempt one because the sample read returned 404. A separate 20-process run produced 20 demo creates with HTTP 201 followed by 20 immediate share reads with HTTP 404. Every page showed `The demo could not start`.

Fix the live data topology, then require the independent-process live demo test to pass 20/20 before release.

## Verification summary

- All 13 exact `.factory/claims.json` commands: pass locally.
- `npm test`: pass, 4 Rust tests and 19 Playwright tests.
- `npm run build`: pass, `dist/` produced.
- `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`: pass.
- `npm audit --audit-level=high`: pass, zero vulnerabilities.
- Live route/Axe review: zero serious/critical issues on checked screens; metadata, focus, 404, same-origin traffic, and public link health pass.
- Live one-click/fresh-process demo: fail.

## Remaining work

Resolve F-3-1 first. Then close the seven copy and claims findings in `.factory/review-3.md`: two vague `clear` labels, one `record`/`receipt` terminology mismatch, two policy-page slogan H1s, the untested `any device` license wording, and the unlisted Dodo/merchant statements. Re-run the entire review; the acceptance threshold is zero findings.
