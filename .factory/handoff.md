# Independent QA handoff — FAIL

Candidate `85c381d3cf4cda5c68a05a8e0ec26f9b203391bd` was independently verified on 2026-08-28 against <https://quote-approval-receipt.sociobot.in>.

**Release verdict: FAIL. Do not promote this candidate.**

## Blocking evidence

- The deployment reports the candidate SHA and serves byte-identical HTML/JS/CSS, but SQLite state is isolated across replicas. A normal record alternated `404, 200` on repeated reads; the demo produced 0 usable reads in 16 fresh create/read attempts. The demo and real workflow are unreliable.
- The advertised Studio checkout returns HTTP 404.
- The backend grants 365-day retention without license proof.
- PDF receipts strip non-ASCII identity characters: `José Núñez — Direção` became `Jos Nez  Direo`.
- Public and README claims are missing from `.factory/claims.json`; Studio and PDF tests do not prove their advertised outcomes.

Additional defects: invalid currency can leave approval pages stuck with a page error; 200% text causes mobile overflow; several targets are under 44×44 px; private API responses lack `no-store`; static cache/compression policy is missing.

Full commands, allowances, hashes, performance, accessibility, privacy, boundary, and severity evidence are in [`.factory/verification.md`](verification.md).

## What passed

- Every declared claim command passed locally when run first.
- `npm ci`, `npm test` (2 Rust + 9 Playwright), TypeScript, format, Clippy with warnings denied, release build, and audit passed.
- Local restart persistence, concurrent-decision conflict handling, export/delete, security headers, same-origin requests, normal 390 px layout, keyboard focus, reduced motion, and axe serious/critical checks passed.
- Lighthouse mobile: 99 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; LCP 1.7 s, CLS 0.
- Rate limits: product GET 40/s and write 15/s with `Retry-After: 1`; billing verify burst 30 with `Retry-After: 4`.

## Reproduce the primary failure

1. Open <https://quote-approval-receipt.sociobot.in> in a fresh browser.
2. Select **Try it with sample data**.
3. Observe **The demo could not start** after create succeeds and the new share read returns 404.

Before re-verification, move records to shared durable storage, enable the Sociobot checkout, enforce Studio server-side, preserve Unicode in PDFs, and repair claims coverage. No product code was modified during verification.
