# Demo sandbox

- URL: `http://localhost:8080/?demo=1` locally or `https://quote-approval-receipt.sociobot.in/?demo=1` in production. `/demo` remains a readable direct alias.
- Sample: Northstar Studio quote `NS-2048` for Juniper Market. It includes a half-day product shoot and 24 edited images.
- Reset: choose **Reset demo** in the persistent yellow banner. The server deletes the old workspace before it creates a new sample.
- Exit: choose **Start for real**. The server deletes the workspace, clears demo session data, and then opens the empty quote builder.
- Isolation: each `POST /api/demo` creates a random backend workspace with a 24-hour expiry. Its quote has `demo_workspace` set and never creates an owner key in browser storage. The browser stores only the current demo pointer under the `sessionStorage` key `demo:workspace`.
- Leaving any demo route through the app also deletes the active workspace. The demo never calls billing, analytics, or AI services.

Verification starts at `/demo` in a fresh browser context. It records the sample decision, then downloads and checks the PDF receipt.
