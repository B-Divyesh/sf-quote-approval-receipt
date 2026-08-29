# Container deployment state contract

This backend uses SQLite with a durable snapshot after each committed write. It must have exactly one serving writer. The factory Container Apps deployment therefore uses this exact state configuration:

- `minReplicas: 1`, `maxReplicas: 1`
- Azure Files environment storage: `quote-approval-receipt-data`
- mounted read/write at `/durable`
- container environment: `PORT=8080`, `DURABLE_DATA_DIR=/durable`

`DATA_DIR` remains the image default (`/data`) so SQLite uses local fast storage and copies each committed snapshot to the mounted share. Do not scale this SQLite deployment above one replica: SQLite has one writer and Azure Files is used for durable replacement snapshots, not concurrent database locking. The one-replica constraint makes the in-process 15-write-per-second limiter global for the live service. `npm run test:live-topology` asserts the deployed manifest, and `/health` reports `durable_snapshot: true` in this mode. The durable-snapshot claim test commits 20 demos, replaces the process with a different local data directory, and reads all 20 through the new process. Live verification must create and read at least 20 fresh demo workspaces across new connections.
