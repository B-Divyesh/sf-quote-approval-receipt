# Repair handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-polish-1`
- Repair source commit: `3222faf` (deployment configuration: `4bbf55e`)
- Live revision: `sf-quote-approval-receipt--0000016`
- Live image/build identity: `sociobotregistry.azurecr.io/sf-quote-approval-receipt:3222faf`; `/health` reports `3222faf`.
- URL: <https://quote-approval-receipt.sociobot.in>
- Verdict: **PASS** — no known review findings remain.

## What changed

- Replaced the primary demo route with `/?demo=1`, preserving `/demo` as an alias. The direct demo has the persistent isolation banner, reset, and real-start controls.
- Added a production gate that launches 20 separate Chromium processes. Each requires demo creation `201`, the immediate share read `200`, and the visible sample quote.
- Repaired production state topology. The Container App is now one replica with the existing `quote-approval-receipt-data` Azure Files share mounted at `/durable` and `DURABLE_DATA_DIR=/durable`.
- Rewrote cassette-metaphor labels into plain section and control names without changing the cassette/zine visual system.
- Removed unprovable public wording, added `quote-snapshot` and `studio-offer` claims, and made every visitor-facing promise observable in the test suite.
- Updated Open Graph and Twitter titles/descriptions with each SPA route. Added regression coverage for all public route metadata and the 390px 404 width.
- Fixed the 404 tape illustration overflow by clipping it within the recovery panel.

## Exact verification evidence

Clean clone: `/tmp/qar-polish-clean` at `4bbf55efbf187bb5761abd48686c7f5b72ca1acb` after `npm ci` (0 vulnerabilities). It uses the repository's same-commit cached target directory only to avoid recompiling Rust; source, lockfile, dependencies, and all commands were fresh-clone inputs.

- Every exact command in `.factory/claims.json` passed. The 11 browser claim commands passed individually and again together: `11 passed (10.1s)`. `npm run test:runtime-contract` and `npm run test:durable-snapshot` also passed as their own manifest commands.
- `npm test`: Rust unit checks, runtime and durable replacement checks, and browser suite passed. The final direct browser run reported `19 passed (22.4s)`.
- `npm run build`, `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `npm audit --audit-level=high` passed. Rust unit result: `4 passed`.
- ACR build `chr8` passed from a `.git`-excluded tarball. The production image uses the required multi-stage Dockerfile, `rust:1-slim`, and non-root runtime user.
- Live `npm run test:live-demo` passed: **20/20** independent Chromium launches returned demo `201`, share `200`, and visible `Half-day product shoot`.
- Live `/opt/fleet/lib/verify-url.sh` passed: 603 ms load, correct title/lang/main/H1/alt, no console errors. Evidence: `/tmp/qar-live-evidence.UNJ2Wt/verify.json`, `/tmp/qar-live-evidence.UNJ2Wt/screenshot-desktop.png`, `/tmp/qar-live-evidence.UNJ2Wt/screenshot-mobile.png`.
- Live Playwright axe scan found `0 serious/critical` issues on `/`, `/?demo=1`, `/new`, `/privacy`, `/terms`, and `/404-review-missing`. The supplied Axe CLI could not find a system Chrome binary, so the installed Playwright Chromium integration was used instead.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.5 s, TBT 0 ms, CLS 0, 118 KiB transfer. Evidence: `/tmp/qar-live-evidence.UNJ2Wt/lighthouse.json`.
- Live metadata and mobile checks passed for demo, privacy, terms, builder, and 404. Evidence screenshots: `/tmp/qar-live-evidence.UNJ2Wt/demo-mobile.png` and `/tmp/qar-live-evidence.UNJ2Wt/404-mobile.png`.

## Run and deploy

```sh
npm ci
npm test
npm run build
PORT=8080 cargo run
```

The direct sample is `http://localhost:8080/?demo=1`. The committed deployment payload is `.factory/containerapp-deploy.json`; it fixes `minReplicas` and `maxReplicas` at one and mounts the named Azure Files storage at `/durable`.

## Known gaps

None. The product intentionally has no AI workflow and no offline/PWA claim; deterministic quote capture and a backend-held private record are the product's real job.
