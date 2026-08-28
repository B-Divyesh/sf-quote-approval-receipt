# Demo sandbox

- URL: `http://localhost:8080/demo` locally or `https://quote-approval-receipt.sociobot.in/demo` in production.
- Sample: Northstar Studio quote `NS-2048` for Juniper Market. It includes a half-day product shoot and 24 edited images.
- Reset: choose **Reset demo** in the persistent yellow banner.
- Isolation: each `POST /api/demo` creates a random backend workspace with a 24-hour expiry. Its quote has `demo_workspace` set and never creates an owner key in browser storage. The browser stores only the current demo pointer under the `sessionStorage` key `demo:workspace`.
- The demo never calls billing, analytics, or AI services.

Verification starts at `/demo` in a fresh browser context. It records the sample decision, then downloads and checks the PDF receipt.
