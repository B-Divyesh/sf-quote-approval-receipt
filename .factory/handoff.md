# Verification handoff 11 — FAIL

Candidate `aea90d11a961be20b07825a5a917fa092268324d` at <https://quote-approval-receipt.sociobot.in> **FAILS release**.

The local source is buildable and its full suite passes: all 13 claims commands, `npm test` (22 Playwright tests plus Rust/runtime/durability checks), type checks, formatting, Clippy, and production Vite build. The live deployment matches the candidate SHA but does not meet its backend persistence contract.

Critical evidence: `/health` returns `durable_snapshot:false`; the checked-in live topology gate observes `maxReplicas: 3` instead of 1; and fresh mobile Chromium observed `POST /api/demo` return 201 followed by an immediate matching `GET /api/share/<token>` 404. The checked-in live-demo check reproduced that 201-to-404 loss on a fresh repeat. A link that disappears immediately cannot provide a defensible approval record.

The live rate-limit check passed (40 reads then 429/`Retry-After: 1`; 15 writes then 429/`Retry-After: 1`), and the static routes passed accessibility, keyboard focus, mobile reflow, headers, and same-origin request checks. These do not offset the persistence failure.

Deploy the configured one-replica Azure Files setup (`/durable`, `DURABLE_DATA_DIR=/durable`), verify health reports `durable_snapshot:true`, then repeat live topology, workflow, 20-session demo, and rate-limit tests. Full evidence is in [verification-11.md](verification-11.md).
