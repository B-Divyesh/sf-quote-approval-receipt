# Review handoff — Quote Approval Receipt

- Work order: `quote-approval-receipt-review-1`
- Reviewer action: read-only live and repository audit; only this handoff and `.factory/review-1.md` changed.
- Verdict: **FAIL**.

## What was done

- Checked the cold landing at 390px and desktop, the direct demo route, demo storage/banner/reset path, public metadata, routes, links, mobile 404, privacy requests, history records, source, README, and claims manifest.
- Cloned the repository into a fresh temporary directory, ran `npm ci`, every exact claims-manifest command, and `npm test`. All local commands passed.
- Reproduced a live deployment failure with 20 independent Chromium processes: every process received `201` from `/api/demo`, then `404` from its returned `/api/share/<token>` route. The review records this as blocking F-1-1.

## How to verify

Read `.factory/review-1.md` for evidence and the full finding list. Locally:

```sh
npm ci
npm test
```

For the release gate, test the live URL with separate browser processes (not merely isolated contexts) and require all 20 `/demo` create-then-read pairs to show `Half-day product shoot`.

## Known gaps / next steps

Fix F-1-1 before any release claim. Also address the persisted 404 mobile overflow, unlisted claims, non-informative copy labels, and per-route OG/Twitter metadata listed in the review. No product code was modified by this review.
