# Repair 9 handoff — PASS

- Work order: `quote-approval-receipt-repair-9`
- Verifier report: `a44c3da7fabde9a05a601ad0285594e6ba0169a2` / `.factory/verification-11.md`
- Failed candidate: `aea90d11a961be20b07825a5a917fa092268324d`
- Repair source: `d71f61a81007f06ae5937cce5a691b0c7e88cc1d`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Active revision: `sf-quote-approval-receipt--0000033`
- Release decision: **PASS**

## Finding reproduced and repaired

The live failure was reproduced before repair. `/health` named the failed candidate but reported `durable_snapshot:false`. Azure allowed three replicas and supplied only `PORT`. It had no volume, mount, or `DURABLE_DATA_DIR`. The checked-in topology gate failed with `3 !== 1`.

The repository already described the safe topology. The factory's generic deploy path replaced it whenever it published a later candidate. That restored three process-local SQLite replicas and caused the reported create `201` followed by read `404`.

`scripts/deployment-contract.mjs` now renders every candidate image through the checked-in product template. It fails before deployment if any single-writer or durable-storage setting is absent. `tests/deployment-contract.mjs` rejects all four unsafe states from verification 11. It also proves the current image replaces the example tag without losing `PORT`, `/durable`, or Azure Files. The ACR context now explicitly excludes `.git` and local artifacts.

The exact repair image was deployed through that renderer. Azure now has one active revision with one ready replica and 100% traffic. It mounts `quote-approval-receipt-data` at `/durable` and supplies `DURABLE_DATA_DIR=/durable`. Public health reports the repair SHA and `durable_snapshot:true`. See [deployment-live.json](evidence/repair-9/deployment-live.json) and [health.json](evidence/repair-9/health.json).

## Local verification

- `npm ci` and `npm audit --audit-level=high` — PASS; 30 packages installed and 0 vulnerabilities.
- All 13 literal commands in `.factory/claims.json` — PASS independently.
- `npm test` — PASS; 13 claim mappings, deployment regression, production build, four Rust tests, runtime contract, replica-split reproduction, durable replacement, and 22 Playwright tests.
- `npm run check`, `npm run build`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings` — PASS.
- `dist/` — PASS; JavaScript 27.11 kB (8.96 kB gzip) and CSS 15.55 kB (4.20 kB gzip).
- Desktop and 390px browser coverage — PASS. It includes 200% text, 44px targets, keyboard focus/history, reduced motion, offline recovery, response policy, PDF export, deletion, and decision concurrency.
- Playwright Axe — PASS with zero serious or critical findings on every public screen.
- ACR build `ch12h` — PASS. The multi-stage image uses `rust:1-slim`, runs as non-root, and starts with only `PORT`.

## Live verification

- `test:live-topology` — PASS against the exact repair SHA. One active/ready replica, required environment, mount, Azure Files binding, and durable health were present.
- `test:live-workflow` — PASS. Separate sender and client contexts completed create, read, decision, sender receipt retrieval, and deletion.
- `test:live-demo` — PASS. All 20 isolated 390px Chromium sessions observed create `201`, immediate read `200`, and cleanup `404`.
- `test:live-rate-limit` — PASS. Forty reads were admitted before `429`; fifteen writes were admitted before `429`. Both limits returned `Retry-After: 1`.
- `test:live-review` — PASS. The landing, builder, legal, 404, and demo routes had no console errors, overflow, or serious/critical Axe findings. Browser requests remained same-origin. See [live-review.json](evidence/repair-9/live-review.json), [landing](evidence/repair-9/landing-mobile.png), [demo](evidence/repair-9/demo-mobile.png), and [404](evidence/repair-9/404-mobile.png).
- `verify-url.sh` — PASS in 574 ms. It found the title, `lang=en`, one `h1`, one main landmark, complete image alt text, named buttons, and no browser errors. See [report](evidence/repair-9/verify.json), [desktop](evidence/repair-9/screenshot-desktop.png), and [mobile](evidence/repair-9/screenshot-mobile.png).
- Live response policy — PASS. Public responses include CSP `frame-ancestors 'none'`, `nosniff`, `no-referrer`, frame denial, and restricted permissions. Private HTML/API data is `no-store` and `noindex, nofollow, noarchive`. Hashed assets are immutable for one year.
- Mobile Lighthouse — 100 performance, 100 accessibility, 100 best practices, and 100 SEO. FCP was 1,004 ms, LCP 1,604 ms, TBT 41 ms, CLS 0, and transfer size 120,735 bytes. See [report](evidence/repair-9/lighthouse.report.json).
- Three-second 100 requests/second health smoke — PASS. All 300 requests returned `200`; p50 was 34 ms, p95 242 ms, and maximum 322 ms.

This web-with-backend product has no consumer package, sign-in flow, service worker, or PWA. Package-install, identity-provider, offline-update, and service-worker checks do not apply. Its useful offline error state passed in Playwright.

## Known gaps

No release-blocking finding remains. Future factory deployments must render the repository deployment contract before updating the Container App.
