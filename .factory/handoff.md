# Verification 10 handoff — FAIL

- Work order: `quote-approval-receipt-verify-10`
- Candidate commit: `1a5283f8246b96f1cb0105f6018f476fa124cdfc`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Active revision: `sf-quote-approval-receipt--0000030`
- Release decision: **FAIL — do not release**

## What was verified

- Installed from the clean candidate with `npm ci`.
- Ran every literal command in `.factory/claims.json`: all 13 local claim checks passed.
- Ran `npm run check`, `npm run build`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, a release-binary boundary/recovery probe, live headers/privacy/Axe/mobile checks, and live topology/demo/rate checks.
- `npm test` is **not passing**: 19 tests pass and the 390px, 200%-text demo touch-target check fails reproducibly.

## Release blockers

1. **Critical — unsafe deployed state.** The live `/health` identifies this candidate but returns `durable_snapshot:false`. Azure has `maxReplicas: 3`, no durable mount/volume and no `DURABLE_DATA_DIR`; two replicas were ready during verification. `test:live-topology` failed (`3 !== 1`) and `test:live-demo` failed on attempt 11 when an immediate read of a new demo quote returned 404.
2. **High — local quality gate fails.** The `mobile-390` 200%-text test on `/?demo=1` finds zero-sized controls and fails both as part of `npm test` and when run alone.

The currently observed live per-client limiter admits 40 reads and 15 invalid writes, then returns 429 with `Retry-After: 1`; privacy headers, same-origin browser requests, and serious/critical Axe checks on public pages passed. These passes cannot compensate for the broken core demo flow and failed full test suite.

## Next steps

Apply `.factory/containerapp-deploy.json` to the active deployment (one replica, Azure Files mounted at `/durable`, `DURABLE_DATA_DIR=/durable`), verify `durable_snapshot:true`, repair the mobile test/rendering state, then rerun the complete local suite and live topology/workflow/demo/rate checks. Full evidence is in [verification-10.md](verification-10.md).
