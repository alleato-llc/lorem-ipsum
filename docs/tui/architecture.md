# lorem-tui architecture

Interactive terminal UI built on ratatui. Run with `cargo run -p lorem-tui`
(interactive — don't run it headless).

## Modules

| Module | Responsibility |
| --- | --- |
| `app` | all state and key handling — no rendering; fully unit-testable without a terminal |
| `ui` | rendering only: form panel, output table, footer |
| `theme` | the palette (royal purple, mirroring the GUI) |
| `main` | terminal setup + event loop (read key → `app.on_key` → draw) |

The split is deliberate: `App` never touches a terminal, so its tests
construct state with `App::from_options(...)` (bypassing the user's
settings file) and drive `KeyCode`s through `on_key`.

## Layout

```
┌ Options ──────────┬ Output ──────────────────────────────┐
│ Theme             │  #   Words   Paragraph                │
│ Mode              │  1   62      Lorem ipsum dolor …      │
│ Count             │  2   71      …                        │
│ Min/Max sentences │                                       │
│ Min/Max words     │         (scrollable table)            │
│ Lorem opener      │                                       │
│ Seed              │                                       │
│ [Enter: Generate] │                                       │
└───────────────────┴───────────────────────────────────────┘
 Tab switch  ↑↓ navigate  ←→ adjust  Enter/g generate  s save  q quit
```

## Keys

- `Tab` — switch focus between form and output
- `↑↓` — select field / scroll table rows (`j`/`k` work in the table)
- `←→` — adjust the focused field: themes/modes cycle, numbers step
  (count steps by 10 in words mode), the opener toggles; digits type into
  Seed (blank = random)
- `Enter` / `g` — generate
- `s` — save current options as defaults (path shown in the footer)
- `q` / `Esc` — quit

Fields that don't apply to the current mode/theme render dimmed (sentence
ranges outside paragraphs mode, word ranges in words mode, the opener
off-classic) — core ignores them anyway.

## Rendering notes

- Output rows wrap their text to the column width via `textwrap`, with row
  height set to the wrapped line count — that's what makes long paragraphs
  scroll as table rows.
- Uses the `ratatui::crossterm` re-export; don't add a separate crossterm
  dependency (version skew breaks event types).
- Palette constants in `theme.rs` mirror the GUI's CSS variables
  (`--accent` `#7851a9`, `--accent-light` `#a78bdb`) — keep them in sync
  when retheming.
