# Container deployment state contract

This backend uses the mounted SQLite file itself as its durable state boundary. It must never copy that file into each replica's local filesystem. The factory Container Apps deployment therefore uses this exact state configuration:

- `minReplicas: 1`, `maxReplicas: 1`
- Azure Files environment storage: `quote-approval-receipt-data`
- mounted read/write at `/durable`
- container environment: `PORT=8080`, `DURABLE_DATA_DIR=/durable`

`DATA_DIR` remains the image default (`/data`) for local-only runtime files. With `DURABLE_DATA_DIR=/durable`, every process opens `/durable/quotes.sqlite3` directly and stores the privacy salt beside it. Do not scale this SQLite deployment above one replica; direct durable storage additionally prevents state splitting if an old and new instance briefly overlap during rollout. The durable-state regression starts two live processes against one mounted directory and proves that a record created through one is immediately readable through the other. Live verification must create and read at least 20 fresh demo workspaces across new connections.
