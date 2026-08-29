# Quote Approval Receipt

Capture who approved a fixed quote and issue a timestamped receipt.

Quote Approval Receipt is for small agencies and contractors who receive approvals in email or chat. It creates a private link for an existing quote. The client names the decision maker, approves or requests changes, and receives a PDF receipt.

## Try the sample

Open `/?demo=1` after starting the service, or visit:

https://quote-approval-receipt.sociobot.in/?demo=1

The demo creates a random workspace that expires after 24 hours. It stays separate from real quote records. Resetting or leaving the demo deletes its current workspace. See [`.factory/demo.md`](.factory/demo.md) for the sandbox contract.

## Run locally

Requirements: Node.js 22+, npm, Rust 1.88+, and SQLite build support.

```sh
npm install
npm run build
PORT=8080 cargo run
```

Open `http://localhost:8080`. The server creates `data/quotes.sqlite3` and a random privacy salt on first boot. No secret or configuration variable is required.

For frontend work with live reload, run the backend on port 8080 and then run `npm run dev` in another shell.

## Test and build

```sh
npm test
npm run build
docker build --build-arg BUILD_SHA=local -t quote-approval-receipt .
```

`npm test` builds the frontend, runs Rust tests, starts the service, and runs the Playwright claim, mobile, rate-limit, and accessibility checks. The frontend output is `dist/`.

The container listens on `PORT` (default `8080`). It starts with no required configuration, creates a random privacy salt on first boot, and `/health` reports the build SHA. Set `DATA_DIR` or `DATABASE_URL` only when you need to override the persisted SQLite location.

## Data and security

Each real quote gets independent random approval and owner tokens. The owner token stays in that browser and authorizes export or deletion. Private pages and API responses use `noindex` and `no-store` response policies.

Each fixed quote accepts one final decision. API read and write bursts return `429` with `Retry-After` when they exceed the limit.

Free links retain records for 30 days. A 365-day request succeeds only after the backend verifies a Studio license with Sociobot. A $29 Studio checkout on Sociobot appears only when the product is available. License holders can enter a license.

## Deploy

Build the root `Dockerfile`. The image runs as the non-root `app` user and needs only `PORT` to start. The factory Container Apps release is a strict one-replica service: its dedicated Azure Files share is mounted at `/durable` and `DURABLE_DATA_DIR=/durable`. The service writes a durable snapshot after each committed change, so a replacement process restores records before serving traffic. Durable storage is an optional deployment override for local use. The factory owns deployment, DNS, billing registration, and the production build SHA.

Apply `.factory/containerapp-deploy.json` for production. After deployment, run the live topology, real workflow, 20-demo, rate-limit, and review scripts listed in `package.json`. The topology check fails unless the active revision has one ready replica and mounted durable state.

## License

MIT. See [LICENSE](LICENSE).
