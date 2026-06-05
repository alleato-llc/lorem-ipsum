# GUI architecture (Lorem Ipsum Studio)

Tauri 2 desktop app: Rust backend in `gui/src-tauri` (a workspace member),
vanilla TypeScript + Vite frontend in `gui/src`. No frontend framework.

```sh
cd gui
npm install            # once
npm run tauri dev      # native app (Vite on fixed port 1420)
npm run dev            # plain browser — runs on the mock backend
```

## Backend (`src-tauri`)

Three commands, each a one-line delegation to lorem-core: `generate`,
`themes`, `settings`. No logic lives here on purpose — the boundary stays
trivially correct.

## Frontend modules

| Module | Responsibility |
| --- | --- |
| `types.ts` | backend data types (mirror of core's serde types) |
| `backend.ts` | the `Backend` interface + runtime selection |
| `backend/tauri.ts` | real `invoke` implementation |
| `backend/mock.ts` | deterministic in-browser fake (seeded PRNG, same contract) |
| `options.ts` | pure form logic: mode config, clamping, seed parsing, option building |
| `dom.ts` | `byId` element lookup (queried fresh per call — enables component tests) |
| `form.ts` | options form DOM glue |
| `output.ts` | output pane: render, stats, copy |
| `main.ts` | wiring: fetch themes/settings, init modules, first generation |

### The backend seam

`backend.ts` checks for `__TAURI_INTERNALS__`: present → real commands,
absent → mock. This single seam gives plain-browser dev mode, component
tests, and Playwright e2e (see [testing/e2e.md](../testing/e2e.md)) —
`tauri-driver` has no macOS support, so the mock is the e2e boundary.

## UI behavior

- The count slider relabels and retunes per mode (paragraphs 1–12,
  sentences 1–30, words 5–200); inapplicable range fields hide.
- Saved defaults populate the form at startup (saving happens in the CLI
  or TUI — the GUI deliberately has no save control).
- Words mode renders as a flat lowercase run: no indent, no drop cap
  (`#output.words-mode` styles).

## Styling gotchas

- Palette lives in CSS variables at the top of `styles.css`
  (`--accent` royal purple for fills, `--accent-light` for text on dark);
  the TUI mirrors these in `crates/lorem-tui/src/theme.rs`.
- The output pane scroll depends on the `grid-template-rows: 100%` /
  `min-height: 0` chain through `#app`/`#content`/`#output` — flex/grid
  items default to `min-size: auto` and won't shrink to let overflow
  engage. Don't remove those declarations.
- `[hidden] { display: none !important; }` is load-bearing: author
  `display` rules otherwise override the UA's `[hidden]` handling (found
  by e2e).
- The app icon (`src-tauri/icons/icon.png`) is required at *compile* time
  by `tauri::generate_context!`. It's a placeholder from
  `python3 scripts/gen_icon.py`; replace with real art + `npx tauri icon`
  before enabling `bundle.active` in `tauri.conf.json`.
