# Unit tests

One module, tested in isolation, internals visible.

## Rust

Every module declares its tests as a separate sibling file:

```rust
// at the bottom of src/walk.rs
#[cfg(test)]
mod tests;
```

```text
src/walk.rs          # the module
src/walk/tests.rs    # its tests (use super::*; reaches pub(crate) items)
```

Run with `cargo test -p <crate>`. Current suites:

| Crate | Module | What it proves |
| --- | --- | --- |
| lorem-core | `text` | tokenization, normalization, capitalization |
| lorem-core | `interner` | id allocation, round-trips, lookups |
| lorem-core | `chain` | transitions, dead ends, enders, starts |
| lorem-core | `model` | corpus → vocab+chain, `find_start` |
| lorem-core | `walk` | length targets, ender stops, splice commas |
| lorem-core | `compose` | capitalization, comma placement, period |
| lorem-core | `generator` | opener once, word-walk continuity, counters |
| lorem-core | `types` | `Mode` parsing, defaults, partial deserialization |
| lorem-core | `settings` | round-trip, seed never persisted |
| lorem-cli | `resolve` | flag → saved default → built-in precedence |
| lorem-cli | `stream` | `LineWrap` column budgeting |
| lorem-tui | `app` | key handling, clamping, field dimming, focus |

### Techniques

- **Tiny synthetic corpora.** Chain and walk tests build
  `Chain::from_sentences(&[vec![0, 1, 2, 3]])` so every transition is
  hand-checkable. Don't unit test against the real theme corpora — they're
  data, not logic.
- **Seeded RNG.** Always `StdRng::seed_from_u64(n)`; never unseeded
  randomness in tests.
- **TUI without a terminal.** Build state with `App::from_options(...)`
  (avoids reading the user's settings file) and feed `KeyCode`s through
  `on_key`; assert on state.

## TypeScript

`gui/tests/unit/*.test.ts`, run by `npm test`. Only pure logic belongs
here — `options.ts` (clamping, seed parsing, option building) and the mock
backend's contract. If a test needs the DOM, it's a
[component test](component.md).
