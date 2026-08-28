# Verification handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-verify-3`
- Candidate: `df5ec08b4abf5767daa20f6956b68632acad75c8`
- URL: <https://quote-approval-receipt.sociobot.in>
- Verdict: **FAIL — do not release**

All declared claim commands, the full local test suite, type/format/lint checks, build, and accessibility checks pass locally. The live deployment is the exact candidate, but it still splits state between serving instances.

Fresh live evidence: 20 new `/demo` browser contexts all created a demo (`POST /api/demo` 201), but only 4 then read their own sample quote (200); 16 immediate `GET /api/share/<token>` calls returned 404 and displayed `The demo could not start`. A previously created demo link was likewise readable in only 7 of 20 fresh contexts. This is a critical failure of the required one-click demo and makes real quote/approval/receipt continuity unreliable.

The live header identity is the candidate SHA and live HTML/JS/CSS SHA-256 values exactly match the local build. Privacy request logging showed same-origin resources only; security headers, private no-store/noindex headers, asset caching, keyboard focus, reduced motion, 390 px layout, and axe serious/critical scans passed. The API did rate limit, but a single 60-write burst was accepted 30 times before 30 `429` responses (`Retry-After: 1`), consistent with two independent per-instance limiters.

See [verification-3.md](verification-3.md) for complete command results, severity, and remediation. Before re-verification, deploy one truly shared durable data boundary (or a single durable instance) and prove at least 20 fresh `/demo` create→read pairs all work.
