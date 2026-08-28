# Repair handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-repair-2`
- Repair base: `68a2fbba488d7cccc35d87052693dd8b7a910e0c`
- Deployment class: container (`PORT=8080`)
- Live URL: <https://quote-approval-receipt.sociobot.in>

## Repaired findings

1. **Split live state / failed one-click demo:** the previous Container Apps configuration allowed three replicas but mounted no durable volume. The checked deployment contract is now one replica (`minReplicas: 1`, `maxReplicas: 1`) with the dedicated `quote-approval-receipt-data` Azure Files storage mounted at `/durable` and `DURABLE_DATA_DIR=/durable`. SQLite remains a single-writer local database and snapshots every committed write to that durable share. This is the safe state model for this SQLite product; it prevents a request from landing on a different local database.
2. **Sender receipt retrieval:** owner-status responses now include the receipt ID only to an authorised owner. The management page exposes **View decision receipt** and **Download PDF receipt** after a decision. Public approval reads still do not expose the receipt ID.
3. **Unlisted README promises:** added `runtime-contract` and `durable-snapshot` claims with executable checks. The runtime check starts the release binary in a clean directory with only `PORT`, confirms `/health` and the generated privacy salt, and checks the Docker non-root/stable-Rust contract. The durable check creates a record in one process then reads it in a replacement process using a different local database directory.
4. **Docker contract:** changed the builder from the forbidden pinned `rust:1.88-bookworm` to `rust:1-slim`.

## Verification completed locally

```sh
npm ci
npm test
npm run check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
BUILD_SHA=$(git rev-parse HEAD) cargo build --release
```

Results:

- `npm ci`: 31 packages; zero vulnerabilities.
- `npm test`: pass — 4 Rust tests and 16 Playwright tests. It includes the runtime and durable-state claim checks, desktop/browser flow, 390 px and 200% text reflow, keyboard route focus, reduced motion, axe serious/critical checks, headers, privacy, and rate limiting.
- `npm run check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `npm audit --audit-level=high`: pass.
- `npm run build`: produces `dist/` (JS 26.00 kB raw / 8.86 kB gzip; CSS 15.45 kB raw / 4.19 kB gzip).
- `npm run test:runtime-contract`: pass (`@claim:runtime-contract`).
- `npm run test:durable-snapshot`: pass (`@claim:durable-snapshot`).
- The Docker daemon is unavailable in this worker, so image construction is verified by the factory ACR build during deployment. The Dockerfile is `.git`-free, multi-stage, non-root, and has a default `BUILD_SHA` argument.

## Live deployment evidence

- `/health` returns `{"build_sha":"0eadcb2830fd7e4d592f9527551585eb2fada87e","status":"ok"}`.
- The running Container App revision is healthy with one replica. Its template confirms `minReplicas: 1`, `maxReplicas: 1`, `DURABLE_DATA_DIR=/durable`, and the read/write Azure Files storage `quote-approval-receipt-data` mounted at `/durable`.
- The public custom-domain certificate binding was restored after the template revision and `https://quote-approval-receipt.sociobot.in/health` returns 200.
- 20 fresh HTTP/1.1 demo create→read pairs, each using new connections, all returned `201` then `200` (`20/20`). This reproduces the verifier's failed sequence under the repaired state configuration.
- `/opt/fleet/lib/verify-url.sh` passed: 646 ms landing load, correct title and `lang=en`, one H1, main landmark, complete image alt text, and no reported errors.
- A live fresh sender browser context displayed **View decision receipt** and **Download PDF receipt** after an approval; the latter pointed to the generated receipt PDF. Live landing axe had zero serious/critical findings; 390 px had no horizontal overflow.
- A live 20-request write burst preserved rate limiting (15×201, 5×429) with `Retry-After: 1`.

The product has no PWA/offline feature, package-consumer surface, or AI action; those checks do not apply.

## Known gaps

None known. SQLite deliberately remains constrained to one Container Apps replica; scaling this product above one requires a migration to a shared database first.
