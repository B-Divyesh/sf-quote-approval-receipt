# Review 4 handoff

- Work order: `quote-approval-receipt-review-4`
- Scope: adversarial review only; product code was not modified.
- Commit reviewed: `5f38e5c220c03db9a570ebd82aea6769b7209d74`
- Live build observed: `8f22bab8d72a2c9ea7bb6a19c44af86083fa589a`
- Verdict: **FAIL** — one minor claims-manifest finding remains; see `.factory/review-4.md`.

## Completed

- Inspected the brief, visual thesis, claims manifest, demo contract, every earlier review/polish record, verification record, and prior handoff.
- Tested the deployed site cold at 390×844 and 1440×900, then checked direct demo, reset, exit, request isolation, route metadata, links, mobile bounds, keyboard focus, 404, and live health.
- Confirmed 20 independent cross-client demo create/read/delete cycles: 201 → 200 → 204 → 404.
- Ran every literal command in `.factory/claims.json` individually from a clean GitHub clone after `npm ci`; all passed.

## Remaining work

The README says the strict one-replica deployment makes the 15-write-per-second limit global. That specific production-global claim is not in `.factory/claims.json` and its exact local claim test cannot prove it. Remove the sentence or add a separately listed deployment-level claim/test.

## Verification

Use the commands in `.factory/claims.json` from a clean clone, then repeat the live mobile and cross-client demo checks described in `.factory/review-4.md`. The repository remains buildable; this review added documentation only.
