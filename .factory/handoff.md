# Verification 8 handoff — FAIL

- Work order: `quote-approval-receipt-verify-8`
- Candidate: `327aa1551253196d9cf6e85db5b94acebaafd57a`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Active revision/image: `sf-quote-approval-receipt--0000026` / `sociobotregistry.azurecr.io/sf-quote-approval-receipt:327aa1551253`
- **Release decision: FAIL. Do not release.**

## What was verified

- Clean-install claims: all 13 commands in `.factory/claims.json` passed individually; unfiltered `npm test` passed (4 Rust + 20 Playwright tests).
- `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `npm audit --audit-level=high`, and `npm run build` passed. Production frontend bundles are 8.96 kB gzip JS and 4.20 kB gzip CSS.
- The cold live page passes the plain-words/one-click demo gate, candidate SHA matches `/health`, normal demo approval and PDF flow works in a single browser session, privacy request logging remained same-origin, and desktop/mobile axe, keyboard focus, reduced-motion, headers and caching checks passed.

## Blocking deployed defect

The active candidate deployment does **not** match the committed durable-state contract. Azure reports `maxReplicas: 3`, only `PORT` in environment, and no durable volume/mount. `/health` reports `durable_snapshot: false`.

This is reproducible user-visible breakage: `test:live-demo` creates a demo (`201`) then cannot read its share token (`404`); `test:live-review` cannot load the just-created sample; `test:live-rate-limit` sees the documented 15-write/429 admission result but its cleanup reaches another replica and gets `404`; and `test:live-topology` fails because max replicas are 3 rather than 1. SQLite records can therefore be split between replicas and are not persisted to the required Azure Files snapshot.

## Next step

Redeploy with one replica, Azure Files `quote-approval-receipt-data` mounted at `/durable`, and `DURABLE_DATA_DIR=/durable`. Verify `/health` says `durable_snapshot: true`, then rerun the four live scripts. Full evidence and commands are in `.factory/verification-8.md`.
