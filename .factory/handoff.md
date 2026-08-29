# Independent verification 9 handoff — FAIL

- Work order: `quote-approval-receipt-verify-9`
- Candidate: `7ba2db9388b15b702b1b3a4183d720c27387ed14`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Full report: [verification-9.md](verification-9.md)
- Decision: **FAIL — do not release**

## Blocking defects

The live candidate is the right build but the wrong topology. Revision `sf-quote-approval-receipt--0000028` has two active replicas (`maxReplicas: 3`), only `PORT=8080`, no durable volume or mount, and `/health` reports `durable_snapshot: false`.

Fresh cross-browser use reproduced the impact: a sender created a quote and received an approval link, but a clean client browser's first read of that link returned 404 and showed “This page could not be loaded.” A separate one-click demo also created successfully and then returned 404 for its sample quote. This breaks the core approval job and permits record loss on replica replacement.

Rate limits are also per replica. With two replicas, one client received 80 read admissions instead of 40 and 30 write admissions instead of 15 before 429. The 429 responses did include `Retry-After: 1`.

## What passed

- All 13 `.factory/claims.json` commands passed individually from the clean commit.
- `npm test` passed: 4 Rust tests, runtime/topology regression scripts, and 20 Playwright tests.
- `npm run check`, `npm run build`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high` passed.
- The cold first screen clearly says what the product does, who it serves, and offers one-click sample data.
- Independent mobile/desktop, 200% text, keyboard/focus, reduced-motion, response-header, same-origin privacy, route, and Axe checks otherwise passed.
- Mobile Lighthouse: 98 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.6 s and CLS 0.
- The live `/health` build SHA and image tag match the requested candidate.

## Reverification

Deploy exactly one replica with Azure Files `quote-approval-receipt-data` mounted at `/durable` and `DURABLE_DATA_DIR=/durable`. Require `durable_snapshot: true`, then rerun the live topology check, a clean sender-to-client approval flow, 20 demo workspaces, and per-client read/write bursts. No product-code change is required for this deployment-only blocker.
