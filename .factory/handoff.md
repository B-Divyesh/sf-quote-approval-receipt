# Independent verification handoff — FAIL

- Work order: `quote-approval-receipt-verify-2`
- Candidate: `c3ad810fd955a309b6fc7af2857b4d770283caf7`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-28 UTC
- Verdict: **FAIL — do not release**
- Full evidence: [verification-2.md](verification-2.md)

## Release blockers

1. **Critical — live records are split across serving instances.** All 20 fresh browser demo attempts returned `POST /api/demo` 201 followed by `GET /api/share/<new-token>` 404. Twelve new-connection reads of one token alternated between 404 and 200. The mandatory demo and core record continuity are unreliable.
2. **High — senders cannot retrieve the promised PDF.** After a decision, the owner page has only JSON export and incorrectly tells the sender to open the approval link. That link only says a decision already exists and exposes no receipt/PDF link.
3. **High — README claims are missing from `.factory/claims.json`.** Unlisted claims include no-config first boot, build identity, durable snapshot behavior, and non-root/`PORT` runtime behavior.
4. **High — the Dockerfile violates the backend contract.** It pins `rust:1.88-bookworm`; the contract requires an unpinned stable-major Rust base.

## What passed

- All nine declared claim commands passed locally after `npm ci`.
- Full `npm test`: 4 Rust and 15 Playwright tests passed.
- Type check, formatting, Clippy with denied warnings, dependency audit, Vite build, and exact-SHA release build passed.
- Release runtime started with only `PORT`, persisted a local record across restart, and reported the candidate SHA.
- Live identity hashes match local HTML, JS, and CSS exactly.
- Validation, 40-item boundary, owner authorization, 201/409 concurrency, Unicode PDF extraction, 429 enforcement, private headers, keyboard, reduced motion, 390px/200% reflow, and axe checks passed where production state routing did not intervene.
- Lighthouse mobile: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; LCP 1.6 s, CLS 0, 118 KiB transfer.

## Rate allowances observed live

- Product writes: 15 requests/second, then 429 with `Retry-After: 1`.
- Product GETs: 40 requests/second, then 429 with `Retry-After: 1`.
- Sociobot license verification: 30-request burst, then 429 with `Retry-After: 4`.

## Re-run

```sh
npm ci
npm test
npm run check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
BUILD_SHA=c3ad810fd955a309b6fc7af2857b4d770283caf7 cargo build --release
```

Before another verification, fix live state continuity and owner PDF access, expand the claims manifest, and correct the Docker base tag. No product code was modified during this verification.
