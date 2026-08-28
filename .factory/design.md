# Visual thesis: dubbed proof

Quote Approval Receipt uses a **cassette-era zine** system. A quote snapshot is treated like a mixtape master: fixed, labelled, and handed to one person for a clear response. Photocopied edges and registration marks recall the physical evidence contractors already save, while the interface stays precise enough for a serious record.

## Palette

The product is deliberately single-mode, like black ink and two spot colours on warm stock.

- `--paper: #F4E9D0` — warm receipt paper and the page background.
- `--ink: #171512` — near-black copy and outlines; 15.2:1 on paper.
- `--ink-soft: #554F45` — secondary copy; 7.0:1 on paper.
- `--signal: #D63B2C` — red grease-pencil emphasis; white is not set on it.
- `--tape: #145E58` — oxidised cassette teal for actions; paper text is 6.1:1.
- `--tape-dark: #0B413D` — action hover and pressed state.
- `--yellow: #F2C84B` — sticker and focus field, always paired with ink.
- `--success: #1D6650`, `--warning: #8A4A09`, `--danger: #A62822`.

## Type and spacing

The display face is the system `Arial Black`/`Impact` stack, set tight and uppercase where a zine would use cut headlines. Body copy uses `Courier New`, a familiar self-hosted system monospace that makes names, dates, and hashes feel recorded. No font files or third-party requests are needed.

Spacing follows an 8 px scale: 4, 8, 16, 24, 32, 48, 64, and 96. Copy stays under 70 characters. Controls are at least 44 px high. Hairlines are replaced with 2–3 px ink rules so the composition survives photocopying.

## Shape and interaction grammar

Panels are torn-paper rectangles with slightly uneven `clip-path` corners, black keylines, and hard 5 px print shadows. Fields look like label-maker strips. A red angled stamp marks immutable or completed records. Buttons move one pixel into their hard shadow when pressed. Links are always underlined.

The quote builder is arranged as a workbench, not a dashboard. The quote paper is the main object. The approval screen strips away navigation choices and makes the decision controls unmistakable. Receipt pages preserve the same fixed snapshot with a hash, approver identity, decision, consent text, and UTC time.

## Motion policy

On first view, the cassette illustration settles by 6 px over 240 ms and the red approval stamp lands once over 180 ms. Route changes use a 150 ms opacity transition. Nothing loops. With `prefers-reduced-motion: reduce`, movement and smooth scrolling are removed and state changes are instant.

## Asset plan and provenance

The hero asset is an original editorial still life: a transparent cassette containing a folded quote, red approval stamp, paper clips, and photocopy grain. It has no readable text, logo, person, or brand. It explains that the product preserves a portable record instead of replacing quote software.

- Generated with the factory image model (`factory-image`) on 2026-08-28.
- Source: `assets/src/quote-cassette.png`; production WebP: `public/quote-cassette.webp`.
- Prompt: “Cassette-era independent zine editorial illustration, overhead still life on warm cream photocopy paper. A transparent audio cassette whose reels hold a folded contractor quote sheet, a bold red rubber approval stamp beside it, black paper clips and one teal label strip. Chunky black ink outlines, risograph misregistration, halftone shadows, cut-paper collage edges, tactile toner grain, limited palette cream, near-black, oxidised teal, signal red, small mustard accents. Landscape composition, central object angled slightly, generous quiet paper around it, no people. No readable text, no letters, no logos, no watermark, no brands, no signatures, no UI screenshot.”
- The generated source is reviewed for accidental words, brands, broken geometry, and misleading product UI. Production derivatives are lossily compressed and sized with explicit dimensions.

The wordmark, favicon, status marks, dividers, and 404 cassette are hand-authored CSS/SVG geometry. No external assets are used.
