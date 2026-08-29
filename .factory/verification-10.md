# Independent verification 10 — FAIL

- **Candidate commit:** `1a5283f8246b96f1cb0105f6018f476fa124cdfc`
- **Live URL:** <https://quote-approval-receipt.sociobot.in>
- **Verified:** 2026-08-29 UTC
- **Decision:** **FAIL — do not release.** The source build largely works locally, but the mandatory full local suite fails and the live service for this exact commit has an unsafe, non-durable multi-replica deployment that loses a newly-created demo quote.

## First read — PASS

A cold 1440 × 900 Chromium context loaded `https://quote-approval-receipt.sociobot.in/` with HTTP 200, no page or console errors, and only same-origin browser requests. The first screen says:

- **What it does:** “Record who approved your quote”.
- **Who it is for:** contractors whose clients approve by email or chat and may change expectations later.
- **What to click first:** **Try it with sample data**; the adjacent note says it loads a private sample quote and does not enter the visitor’s records.

The action is one click from the landing screen and was visible in the initial viewport. The resulting screen has the required plain-language job, audience, and next action. Its live build marker and `/health` both identify `1a5283f8246b96f1cb0105f6018f476fa124cdfc`.

## Required claims — PASS locally, FAIL in live reality where noted

After `npm ci` from the clean candidate, I ran every command listed in `.factory/claims.json` exactly, using the repository’s demo/test entry point. All 13 individual claim commands passed; the manifest also reports 13 unique, observable mappings.

| Claim | Local result | Live result |
| --- | --- | --- |
| `demo-sandbox` | PASS | **FAIL**: 20-session live demo check failed on session 11; the just-created sample share URL returned 404. |
| `quote-snapshot` | PASS | Not releasable because the live demo record itself is intermittently unavailable. |
| `pdf-receipt` | PASS | Blocked from relying on live state by the deployment failure. |
| `record-control` | PASS | Not releaseable because the deployment failure affects record reachability. |
| `first-party-only` | PASS | PASS: cold landing and successful demo sessions requested only the product origin. |
| `retention-policy` | PASS | PASS locally. |
| `studio-offer` | PASS | PASS locally. |
| `private-links` | PASS | PASS locally; live private API response had `no-store` and `X-Robots-Tag: noindex, nofollow, noarchive`. |
| `single-decision` | PASS | PASS locally. |
| `network-address-privacy` | PASS | PASS locally. |
| `rate-limits` | PASS | PASS in the fresh live burst: 40 reads then 429, 15 invalid writes then 429; both used `Retry-After: 1`. |
| `runtime-contract` | PASS | PASS locally. |
| `durable-snapshot` | PASS | **FAIL**: live `/health` says `durable_snapshot:false`; no durable deployment is present. |

The local claims do not override the contradictory production evidence. No unlisted material promise was found in the landing page or README.

## Clean-checkout checks

- `npm ci` — PASS: 30 packages installed; `npm audit --audit-level=high` found 0 vulnerabilities.
- Every literal `claims.json` test command — PASS (13/13).
- `npm run check` — PASS.
- `npm run build` — PASS; `dist/` produced.
- `cargo fmt --check` — PASS.
- `cargo clippy --all-targets -- -D warnings` — PASS.
- `npm test` — **FAIL: 19 passed, 1 failed**. See High finding below.
- Docker image build — not run because Docker is unavailable in this verifier container. The source Dockerfile and passing runtime-contract test were inspected; this does not offset the failed full suite or live deployment failure.

Production client output is 27.11 kB JavaScript (8.96 kB gzip) and 15.55 kB CSS (4.20 kB gzip), comfortably under the stated static bundle budgets.

## Release-blocking findings

### Critical — exact live candidate is multi-replica, non-durable, and loses new demo records

Fresh evidence binds this finding to the requested live candidate:

- `GET /health` returned `{"build_sha":"1a5283f8246b96f1cb0105f6018f476fa124cdfc","durable_snapshot":false,"status":"ok"}`.
- Azure reports active revision `sf-quote-approval-receipt--0000030`, image `sociobotregistry.azurecr.io/sf-quote-approval-receipt:1a5283f8246b`, and 100% traffic.
- Its template has `minReplicas: 1`, **`maxReplicas: 3`**, only `PORT=8080`, no `DURABLE_DATA_DIR`, no volume mount, and no Azure Files volume.
- During the live test Azure reported **two ready replicas** for that active revision.
- The checked-in `npm run test:live-topology` failed immediately with `durable SQLite release must never split state or the limiter: 3 !== 1`.
- The checked-in `npm run test:live-demo` completed ten isolated browser sessions, then on session 11 received `201` from `POST /api/demo` and **404** from the immediate `GET /api/share/<new-token>`.

This is the product’s core job: the sender must be able to hand a quote to the approver. A newly-created quote that disappears before the next read is not a defensible approval record. The repository manifest correctly requires one replica, `/durable`, `DURABLE_DATA_DIR=/durable`, and Azure Files `quote-approval-receipt-data`; the active deployment does not have that contract.

### High — mandatory 390px mobile quality gate fails reproducibly

`npm test` exits non-zero. The `mobile-390` project fails `@mobile 390px pages reflow at 200% text and controls meet target size` on `/?demo=1`:

```text
/?demo=1 has undersized controls
Received: Reset demo / Start for real and page controls with 0 × 0 boxes
```

The rest of the same full run passed (19 tests), including the desktop instance of that test. Running the mobile project alone reproduced the same failure. Whether this is the async demo screen being measured before it is rendered or an actual layout-state defect, the committed quality gate is not reliable or passing; Definition of Done requires it to pass locally.

## Functional, boundary, privacy, and accessibility evidence

- Local claim and full-suite coverage passed normal creation, clean-client approval, a timestamped PDF with an international named approver, export/delete, one-final-decision concurrency, demo reset/exit, invalid currency recovery, offline recovery, keyboard skip/focus/history, route metadata, reduced motion, and serious/critical Axe checks.
- A separate release-binary boundary probe accepted 40 items at quantity `100000`, rate `10000000`, and 100% tax, then deleted the record. It rejected 41 items, zero quantity, rate `10000001`, tax 101%, unsupported currency, forged 365-day retention, and an unconsented/invalid decision with the expected 422/403 status and useful recovery message.
- In the live product, an initial page request log contained only `https://quote-approval-receipt.sociobot.in` (HTML, same-origin JS/CSS/image and `/api/studio`). No third-party browser script, analytics, or advertising request appeared.
- Live `/`, `/new`, `/privacy`, `/terms`, and a real 404 at 390px each had one `h1`, one `main`, no horizontal overflow, no console/page errors, and zero Axe serious/critical violations. The successful live demo sessions also had no mobile overflow.
- Cold live keyboard/page checks and the local keyboard regression passed. The full local reduced-motion/Axe regression passed in Chromium.
- Live private API responses carried `Cache-Control: no-store` and `X-Robots-Tag: noindex, nofollow, noarchive`. Public responses had restrictive CSP including response-header `frame-ancestors 'none'`, `nosniff`, `no-referrer`, frame denial, and restrictive permissions policy. Hashed assets used one-year immutable caching.
- The live rate-limit probe observed the documented enforcement: the 41st read and 16th invalid write from one forwarded client received **429** with **`Retry-After: 1`**. A larger 100-read/40-write burst observed 40 accepted reads / 60 limited and 15 accepted writes / 25 limited. This pass does not mitigate the critical multi-replica state loss.

This web-with-backend product has no sign-in flow, no public consumer package/CLI, and no service worker/PWA, so Entra, consumer-install, and service-worker update checks do not apply.

## Required remediation

1. Apply the checked-in Container Apps contract to the active revision: exactly one replica; mount Azure Files `quote-approval-receipt-data` at `/durable`; set `DURABLE_DATA_DIR=/durable`.
2. Redeploy the requested source and verify `/health` returns the target SHA with `durable_snapshot:true`.
3. Repeat `test:live-topology`, `test:live-workflow`, 20 isolated live demos, and the live rate-limit probe after the deployment change.
4. Repair or correctly synchronize the `mobile-390` test and demo rendering state so `npm test` passes consistently at 390px and 200% text.
