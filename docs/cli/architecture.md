# lorem-cli architecture

Binary name: `lorem`. JSON output by default; plain-text or JSON-Lines
streaming with `--infinite`.

## Modules

| Module | Responsibility |
| --- | --- |
| `args` | the clap surface (`Args`, `OutputFormat`) — definitions only |
| `resolve` | option resolution: flag → saved default → built-in; per-mode stream intervals |
| `stream` | `--infinite` emission: pacing, text wrapping (`LineWrap`), JSON Lines |
| `main` | dispatch only |

## Batch output

```sh
lorem --theme pirate -n 2 --seed 42        # pretty JSON document
lorem --mode words -n 50 --compact         # one-line JSON
lorem --list-themes
```

Every output reports the seed used; pass it back via `--seed` to reproduce.

## Streaming (`--infinite`)

Streams forever (ignores `-n`), one word/sentence/paragraph at a time,
paced by `--interval <ms>` (defaults: 100 words / 400 sentences /
900 paragraphs). Ends quietly on Ctrl-C or when the reader hangs up.

- `--output-format txt` (default): pure prose on stdout — words wrap at
  ~72 columns, sentences one per line, paragraphs blank-line separated.
  The seed goes to **stderr** so stdout stays pipeable.
- `--output-format json`: JSON Lines — a meta line carrying the seed
  first, then `{"index":N,"text":"…"}` per item.

```sh
lorem --infinite --theme classic --mode sentences > lorem.txt
lorem --infinite --theme cosmic --mode words | head -50
```

Implementation note: stream writes check `io::Result` instead of using
`println!`, which would panic on EPIPE.

## Saved defaults

Options left unset fall back to the shared settings file (see
[core architecture](../core/architecture.md)); `--save-defaults` persists
the resolved options. `--lorem-start` / `--no-lorem-start` exist so either
direction can override a saved value.
