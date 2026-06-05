# lorem-core architecture

The shared engine. All three front ends call
`generate(&GeneratorOptions) -> GeneratedText` and `list_themes()`; none of
them contain generation logic.

## Pipeline

Corpus text flows through single-purpose modules:

```
themes/*.txt ─► model ─► chain ──► walk ──► compose ─► generator
                 │ ▲                            ▲          │
                 ▼ │                            │          ▼
              interner ◄──── text          (word strings)  generate / next_item
```

| Module | Responsibility |
| --- | --- |
| `text` | pure helpers: sentence splitting, token normalization, capitalization |
| `interner` | word ↔ `u32` id table (keeps the chain compact) |
| `chain` | order-2 transition table: `next_id` (`None` = dead end), `random_start`, `is_ender` |
| `model` | the only place corpus text meets the chain: corpus → `Interner` + `Chain` |
| `walk` | sentence policy: target length, walk past it to a corpus *ender* word (no mid-phrase stops), dead-end splices and pause points become comma positions |
| `compose` | pure: ids + comma positions → capitalized, punctuated sentence |
| `generator` | orchestration: `Generator::next_item()` yields one word/sentence/paragraph; `generate()` collects a batch |
| `types` | serde types: `Mode`, `GeneratorOptions`, `GeneratedText`, `ThemeInfo` |
| `themes` | `Theme` enum + `list_themes`; corpora embedded via `include_str!` |
| `settings` | persisted defaults (JSON in the platform config dir) |

## Modes

`GeneratorOptions.count` is in units of `mode`:

- **paragraphs** — `count` paragraphs of `min/max_sentences` sentences
- **sentences** — `count` standalone sentences
- **words** — one flat lowercase run of exactly `count` words, no
  punctuation; a *continuous* chain walk, so the stream reads as running
  text (dead ends queue the second word of the spliced start pair to stay
  on real corpus transitions)

`GeneratedText.items` holds one entry per paragraph/sentence, or a single
entry in words mode.

## Invariants

- **Seeds are reproducible and JSON-safe.** Every output reports the seed
  used; auto-drawn seeds stay in u32 range so they survive a JS `number`
  round trip. Don't draw full-u64 seeds.
- **Batch ≡ stream.** `generate` is built on `Generator`, so the same seed
  produces identical output either way (pinned by an integration test).
- `GeneratorOptions` is `#[serde(default)]` — the GUI sends partial
  options; keep new fields defaultable.
- min/max pairs are clamped (`max.max(min)`) in core; front ends don't
  validate ordering.
- The classic theme opens with the canonical "Lorem ipsum…" sentence when
  `start_with_lorem` is set (on by default; words mode starts at the
  "lorem ipsum" pair instead).
- Settings never persist a seed — a saved seed would freeze every run.

## Adding a theme

1. Add a corpus: `crates/lorem-core/src/themes/<name>.txt` — plain
   sentences; reused vocabulary across sentences gives the chain more
   branching.
2. Extend `Theme` in `themes.rs`: variant + `ALL`, `id`, `display_name`,
   `description`, `corpus`, `from_str`.

All front ends discover themes through `list_themes()` / `Theme::ALL` —
no front-end changes needed. (The GUI's mock backend keeps its own small
theme list for browser-only mode; update `gui/src/backend/mock.ts` if the
theme should appear there too.)
