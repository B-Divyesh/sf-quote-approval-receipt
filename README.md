# Quote Approval Receipt

Capture who approved a fixed quote and issue a timestamped receipt.

Quote Approval Receipt is for small agencies and contractors who receive approvals in email or chat. It creates a private link for an existing quote. The client names the decision maker, approves or requests changes, and receives a PDF record.

This is not proposal software, payment collection, or a regulated e-signature service.

## Try the sample

Open `/demo` after starting the service, or visit:

https://quote-approval-receipt.sociobot.in/demo

The demo creates a random 24-hour workspace with a Northstar Studio sample quote. Demo state stays separate from real quote records. See [`.factory/demo.md`](.factory/demo.md) for the sandbox contract.

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

The container listens on `PORT` (default `8080`). Set `DATA_DIR` or `DATABASE_URL` only when you need to override the persisted SQLite location. `/health` reports the build SHA.

## Data and security

Each real quote gets an unguessable approval token and a separate owner token. The owner token stays in that browser and authorizes export or deletion. Approval and receipt routes send `noindex` headers. The service stores a salted one-way network-address hash with each decision.

All API routes are rate limited. The server trusts the first `X-Forwarded-For` value supplied by the factory ingress. It returns `429` and `Retry-After` when a client exceeds the limit.

Free links retain records for 30 days. A $29 one-time Studio license enables 365-day retention. Checkout and license verification use only the Sociobot billing API.

## Deploy

Build the root `Dockerfile` and mount persistent storage at `/data`. The image runs as a non-root user and needs only `PORT`. The factory owns deployment, DNS, billing registration, and the production build SHA.

## License

MIT. See [LICENSE](LICENSE).
