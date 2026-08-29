# Independent product verification 5 — FAIL

- Work order: `quote-approval-receipt-verify-5`
- Candidate commit: `adb2748c32faa2da6af03ce8d4b33137c5062dac`
- URL: <https://quote-approval-receipt.sociobot.in>
- Date: 2026-08-29 UTC
- Verdict: **FAIL — do not release**

## Decision

The candidate is genuinely deployed: its footer and `/health` report `adb2748`, and the live JavaScript asset `index-C_vh9NT7.js` has the exact same SHA-256 as a production Vite build of this checkout with `VITE_BUILD_SHA=adb2748…`. Local tests, build, static checks, and the landing UI are good. The real deployed backend is not reliable, however: a demo created on one request cannot be read by the next request. This fails the mandatory one-click demo and the core approval-record workflow.

## First-read gate

The cold landing copy passes its plain-language test:

- **What:** “Record who approved your quote”.
- **For whom:** contractors whose clients approve by email or chat and later change expectations.
- **First click:** “Try it with sample data”, with the adjacent result “Loads a private sample quote. Nothing enters your records.”

It also shows the three required facts: 30-day free retention, per-record export/deletion, and a PDF receipt. The first click itself fails live, so the overall first-read/demo gate is **FAIL**.

## Release blockers

### Critical — live demo state is not available to the next request

Fresh Chromium at the required entry point, `/?demo=1`, observed:

1. `POST /api/demo` → **201**.
2. The immediate `GET /api/share/y79MUlA1G0109wlxshGz56FAkOIlOUnE` → **404**.
3. The rendered page said **“The demo could not start”** and “This approval link was not found or has been deleted.”

The repository’s own live verifier reproduced it immediately:

```text
LIVE_URL=https://quote-approval-receipt.sociobot.in \
EXPECTED_BUILD_SHA=adb2748c32faa2da6af03ce8d4b33137c5062dac \
npm run test:live-demo

Error: attempt 1: sample read returned 404
```

An independent eight-iteration direct check produced `POST /api/demo` **201** followed by `GET /api/share/<returned-token>` **404** on **8/8** attempts. Deleting just-created demo workspaces also intermittently returned 404. This is fresh evidence of a production state boundary/replica routing failure, not a stale front-end deploy. It makes normal quote creation, approval, management, receipt, export, and deletion continuity unsafe as well as breaking the demo.

### High — live write-rate allowance is not enforced

The documented source allowance is 15 non-GET API requests per second per forwarded client address. I sent 20 sequential `POST /api/demo` requests with one fixed `X-Forwarded-For: 198.51.100.77`. All 20 were **201**; none was **429** and none carried `Retry-After`.

The local `@claim:rate-limits` test passes, but the mandatory live backend check fails. The likely same multi-instance/state split means the per-process limiter is not an effective client-wide limit.

## Claims gate

`.factory/claims.json` exists and lists 13 uniquely mapped, observable claim tests. The literal first pre-install attempt stopped at `tsc: not found`, which is expected in a dependency-free clone; after `npm ci` (30 packages, zero reported vulnerabilities), every manifest command was exercised from the demo-capable local entry point. They pass locally, including the runtime and durable-snapshot commands.

| Claim | Local result | Live result where applicable |
| --- | --- | --- |
| demo-sandbox | PASS | **FAIL**: fresh demo creates 201 then share 404 |
| quote-snapshot | PASS | blocked by unreliable record continuity |
| pdf-receipt | PASS | blocked by unreliable record continuity |
| record-control | PASS | blocked by unreliable record continuity |
| first-party-only | PASS | landing request log is same-origin |
| retention-policy | PASS | not sufficient to establish live persistence |
| studio-offer | PASS | local recorded-response test |
| private-links | PASS | local private-header check |
| single-decision | PASS | local concurrent 201/409 check |
| network-address-privacy | PASS | local hash/export/PDF check |
| rate-limits | PASS | **FAIL**: 20/20 live writes accepted |
| runtime-contract | PASS | local release binary starts with only `PORT` |
| durable-snapshot | PASS | local replacement test; live boundary contradicts it |

## Local quality gates

- `npm ci`: PASS.
- `npm test`: PASS (manifest, production frontend build, four Rust tests, runtime/durable tests, and 20 Playwright tests).
- `npm run check`: PASS.
- `npm run build`: PASS; produces `dist/`.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm audit --audit-level=high`: PASS.
- Explicit `npm run test:runtime-contract` and `npm run test:durable-snapshot`: PASS.
- Production frontend output: JS 27.10 kB raw / 8.96 kB gzip; CSS 15.55 kB raw / 4.20 kB gzip, within the stated budget.
- `docker build …`: not run because this verifier image has no `docker` executable. The native release build and runtime-contract test pass; source inspection confirms the required multi-stage, non-root `rust:1-slim` Dockerfile with default `BUILD_SHA` and no `.git` dependency.

## Live UI, privacy, headers, and accessibility

- Cold landing response: 200, correct title, `lang=en`, one H1 and main landmark. It gives a visible one-click sample action.
- At 390 px: no horizontal overflow on the landing; reduced-motion context reported zero animations.
- Independent live axe scan on the cold landing: **zero serious/critical** violations. No console or page errors.
- The first-party request log on the landing contained only `https://quote-approval-receipt.sociobot.in`; no analytics, advertising, third-party script, font, or CDN request was observed. The demo-flow privacy assertion cannot be completed because the live demo fails before use.
- Response headers include self-only CSP, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `X-Frame-Options: DENY`, restrictive Permissions-Policy, and `frame-ancestors 'none'`. The hashed JS asset has `Cache-Control: public, max-age=31536000, immutable`.

## Required next step

Fix the deployed backend topology before release: route all API requests for a record to one durable shared state store (or deploy the documented single replica with its durable mount effectively attached), then rerun `npm run test:live-demo` and the live fixed-client burst. Acceptance requires 20/20 demo create/read successes and a 429 plus `Retry-After` after the 15-write allowance.
