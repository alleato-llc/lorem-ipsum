# lorem-ipsum

Markov-chain lorem ipsum generator with three front ends sharing one core:

| Crate / dir         | What it is                                              |
| ------------------- | ------------------------------------------------------- |
| `crates/lorem-core` | Order-2 Markov chain engine + themed corpora            |
| `crates/lorem-cli`  | `lorem` CLI — JSON output                               |
| `crates/lorem-tui`  | `lorem-tui` — ratatui form + scrollable paragraph table |
| `gui/`              | Lorem Ipsum Studio — Tauri 2 + TypeScript desktop app   |

## Themes

`classic` (traditional pseudo-Latin), `tech` (startup jargon), `pirate`
(high-seas adventure), `corporate` (buzzword bingo), `cosmic` (spacefaring
prose). Each theme trains its own Markov chain from a plain-text corpus in
`crates/lorem-core/src/themes/`.

The chain is order-2 over words. Sentences keep walking past their target
length until they land on a word that ended a sentence in the corpus, so they
finish on a natural phrase; dead-end splices become commas.

Output is seedable: every result reports the `seed` it used — pass it back in
to reproduce the output exactly.

## Modes

All front ends generate in one of three modes:

- **paragraphs** (default) — `count` paragraphs of `min/max-sentences`
  sentences, each `min/max-words` words long
- **sentences** — `count` standalone sentences
- **words** — a flat run of exactly `count` lowercase words, no punctuation
  (classic theme starts it with "lorem ipsum")

## CLI

```sh
cargo run -p lorem-cli -- --theme pirate -n 2 --seed 42
cargo run -p lorem-cli -- --mode words -n 50 --theme classic
cargo run -p lorem-cli -- --mode sentences -n 5 --theme corporate
cargo run -p lorem-cli -- --list-themes
lorem --help
```

Options: `--theme`, `--mode`, `-n`/`--count`,
`--min-sentences`/`--max-sentences`, `--min-words`/`--max-words`, `--seed`,
`--lorem-start`/`--no-lorem-start`, `--save-defaults`, `--compact`.

Output is JSON (pretty by default, `--compact` for one line):

```json
{
  "theme": "pirate",
  "theme_name": "Pirate",
  "mode": "paragraphs",
  "items": ["The parrot squawked a warning…"],
  "word_count": 106,
  "sentence_count": 7,
  "seed": 42
}
```

## TUI

```sh
cargo run -p lorem-tui
```

`Tab` switches between the options form and the output table. In the form:
`↑↓` select a field, `←→` adjust it (type digits into Seed). `Enter` or `g`
generates; `q` quits.

## GUI (Tauri)

```sh
cd gui
npm install
npm run tauri dev      # development
npm run tauri build    # release build (add real icons to bundle an installer)
```

Sidebar form (theme, mode, count, sentence/word ranges, seed, classic-opener
toggle), scrollable serif paragraphs with a drop cap, live stats, and a
copy-to-clipboard button. Range fields that don't apply to the selected mode
are hidden, and the count slider retunes its bounds per mode. Dark editorial
look with a royal purple accent — palette lives in CSS variables at the top
of `gui/src/styles.css` (the TUI mirrors it via `ACCENT`/`ACCENT_LIGHT`
consts in `crates/lorem-tui/src/main.rs`).

The app icon (`gui/src-tauri/icons/icon.png`) is a generated placeholder —
Tauri refuses to compile without one. Regenerate it (e.g. after changing the
accent color) with `python3 scripts/gen_icon.py`, or replace it with real art
and run `npx tauri icon` to produce the full multi-size set before enabling
`bundle.active` in `tauri.conf.json`.

## Settings

All front ends share persisted defaults in the platform config dir
(`~/Library/Application Support/lorem-ipsum/settings.json` on macOS,
`~/.config/lorem-ipsum/` on Linux). Unset options fall back to saved
defaults, then built-ins. The seed is never persisted.

Save your current options as the new defaults with `lorem … --save-defaults`
(CLI) or `s` (TUI). The GUI loads the saved defaults at startup but has no
save control of its own.

## Adding a theme

1. Add a corpus file under `crates/lorem-core/src/themes/` — plain sentences,
   a few paragraphs is plenty; reused vocabulary across sentences gives the
   chain more branching.
2. Add a variant to `Theme` in `crates/lorem-core/src/themes.rs` and extend
   `ALL`, `id`, `display_name`, `description`, `corpus`, and `from_str`.

All three front ends pick it up automatically.

## Tests

```sh
cargo test -p lorem-core
```
