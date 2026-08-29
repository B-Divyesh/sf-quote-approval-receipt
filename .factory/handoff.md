# Polish round 4 handoff

- Work order: `quote-approval-receipt-polish-4`
- Repair commit: `cfac15d79d708086b8324d8ca2902defe96c4c50`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Deployed revision: `sf-quote-approval-receipt--polish4`
- Image: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:repair-8`

## What changed

- Removed the final unlisted, untestable README assertion that one replica makes the 15-write allowance global.
- Removed the matching assertion from the operator deployment guide and added a manifest-gate regression that rejects both phrases.
- Updated the catalog sentence to `Record quote approvals and issue a named, timestamped PDF receipt.`
- Reapplied the declared one-replica Azure Files deployment contract. Existing product repairs remain present: isolated one-click demo, reset/exit deletion, fixed quote/PDF workflow, sender receipt access, metadata/routing/404, mobile layout, and literal copy.

## Verification

- Clean GitHub clone at `cfac15d79d708086b8324d8ca2902defe96c4c50`: `npm ci` had zero vulnerabilities; every command in `.factory/claims.json` passed individually.
- `npm test` passed: 4 Rust tests and 20 Playwright tests. `npm run build`, `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high` passed.
- ACR build `chwu` succeeded from a `.git`-excluded source archive. The deployed health response is `{"build_sha":"cfac15d79d708086b8324d8ca2902defe96c4c50","durable_snapshot":true,"status":"ok"}`.
- Live: `test:live-topology` passed; `test:live-demo` completed 20/20 fresh-process create `201` → read `200` → cleanup `404` cycles; `test:live-rate-limit` observed 15 writes then a `429` with `Retry-After: 1`; `test:live-review` passed all checked routes with zero serious/critical Axe violations.
- `verify-url.sh` passed with no console errors, one H1, `lang=en`, main landmark, and complete image alt text. Lighthouse scored 100 Performance, 100 Accessibility, 100 Best Practices, and 100 SEO; LCP 0.1 s, TBT 0 ms, CLS 0.
- Evidence and screenshots are in `.factory/evidence/polish-4/`; the finding-by-finding receipt is [`.factory/polish-4.md`](polish-4.md).

## Run and deploy

```sh
npm ci
npm test
npm run build
PORT=8080 cargo run
```

The container starts with only `PORT`; production uses one replica with the `/durable` Azure Files mount and `DURABLE_DATA_DIR=/durable`.

## Known gaps

None.
