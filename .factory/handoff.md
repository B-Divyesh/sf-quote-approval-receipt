# Verification 5 handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-verify-5`
- Verified candidate: `adb2748c32faa2da6af03ce8d4b33137c5062dac`
- Live URL: <https://quote-approval-receipt.sociobot.in>
- Verdict: **FAIL — do not release**

## Verification result

No product code was changed by this verification. The deployed app is exactly the candidate: live `/health`, the footer build ID, and a SHA-256 comparison of the live JavaScript asset match `adb2748…`.

Local build, runtime, durability, accessibility, and claim checks pass after `npm ci`. The production backend does not: a required `/?demo=1` run receives `POST /api/demo` 201 followed immediately by `GET /api/share/<returned-token>` 404, rendering “The demo could not start”. The product’s own `npm run test:live-demo` now fails at its first attempt with that same 404.

The live server also accepts 20 writes from one fixed forwarded client address despite its documented 15-write/sec allowance; no response is 429 and no `Retry-After` header is sent. These are release-blocking deployment defects.

## How verified

On the clean candidate checkout:

- `npm ci`; every manifest claim command; `npm test`; `npm run check`; `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `npm audit --audit-level=high`: PASS locally.
- `npm run build` with candidate Vite build ID: PASS, `dist/` produced. Live JS SHA-256 matched exactly.
- Live landing: plain-language first screen, first-party-only request log, headers, mobile 390 px, keyboard/static suite, reduced motion, and axe serious/critical checks: PASS.
- Live demo and rate limit: FAIL as described above.

Run local gates with:

```bash
npm ci
mapfile -t claim_tests < <(jq -r '.[].test' .factory/claims.json)
for claim_test in "${claim_tests[@]}"; do bash -lc "$claim_test"; done
npm test
npm run check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
```

## Next steps

Do not release. Correct the production state topology/durable mount so a record written on one API request is readable on the next, and ensure the limiter is shared/effective across every serving instance. Then rerun:

```bash
LIVE_URL=https://quote-approval-receipt.sociobot.in \
EXPECTED_BUILD_SHA=adb2748c32faa2da6af03ce8d4b33137c5062dac \
npm run test:live-demo
```

It must complete 20/20 create/read/cleanup flows. Recheck the fixed-client burst: the first 15 writes may succeed, but the next must return `429` and `Retry-After: 1`. See `.factory/verification-5.md` for full evidence.
