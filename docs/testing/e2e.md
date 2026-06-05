# End-to-end tests

Playwright drives the full frontend in a real Chromium:
`gui/e2e/app.spec.ts`.

## How it runs

```sh
cd gui
npx playwright install chromium   # once
npm run test:e2e
```

`playwright.config.ts` starts `vite` on port **1421** (separate from
Tauri's 1420 so a running `tauri dev` doesn't collide). The browser has no
`__TAURI_INTERNALS__`, so `backend.ts` automatically selects the
**deterministic mock backend** (`src/backend/mock.ts`) — same seed → same
output, real themes list, the core's contract in miniature.

## What's covered

- Startup: themes load, an initial generation renders, stats show a seed,
  Copy enables.
- Theme selection updates the description and disables the opener checkbox
  off-classic.
- Words mode hides both range field pairs, retunes the count slider, and
  renders the single flat run with words-mode typography.
- **Seed reproducibility through the whole UI loop**: type a seed,
  generate twice, byte-identical output.
- Paragraph count follows the slider.

## Why the mock seam is the boundary

`tauri-driver` (the WebDriver bridge for real Tauri windows) supports
Linux and Windows only — there is no macOS support. So the native shell
isn't e2e-driven here. The chain of trust is:

1. Playwright proves the frontend behaves correctly against the `Backend`
   interface.
2. Rust integration tests prove `generate`/`themes`/`settings` behave
   correctly behind that interface.
3. The Tauri commands between them are three one-line delegations.

## The bug this layer caught

The component tests asserted `el.hidden === true` and passed — but in a
real browser the sentence-range fields stayed visible. Author CSS
(`.field-pair { display: grid }`) overrides the UA's `[hidden] {
display: none }` rule. The fix is a guard in `styles.css`:

```css
[hidden] {
  display: none !important;
}
```

Keep it: only a real rendering engine can verify visibility, which is
exactly why this layer exists.
