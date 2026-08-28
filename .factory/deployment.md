# Container deployment state contract

This backend uses SQLite with a durable snapshot after each committed write. It must have one writer. The factory Container Apps deployment therefore uses this exact state configuration:

- `minReplicas: 1`, `maxReplicas: 1`
- Azure Files environment storage: `quote-approval-receipt-data`
- mounted read/write at `/durable`
- container environment: `PORT=8080`, `DURABLE_DATA_DIR=/durable`

`DATA_DIR` remains the image default (`/data`) so SQLite uses local fast storage and copies each committed snapshot to the mounted share. Do not scale this SQLite deployment above one replica. The durable-snapshot claim test proves replacement-process restoration; live verification must create and read at least 20 fresh demo workspaces across new connections.
