# Testing design

## Philosophy

Every module owns one responsibility, and every module is tested in
isolation against that responsibility. Cross-cutting behavior gets its own
integration layer. The result is a conventional pyramid: many fast unit
tests, a band of component/integration tests, a handful of end-to-end
flows.

Two conventions hold everywhere:

1. **Tests live in separate files — never inline.** Rust modules declare
   `#[cfg(test)] mod tests;` and keep the tests in a sibling
   `<module>/tests.rs` (which still reaches `pub(crate)` internals).
   TypeScript tests live under `gui/tests/`.
2. **Randomness is always seeded.** Generation is deterministic for a given
   seed, so tests assert on reproducible output rather than fuzzy
   properties wherever possible.

## The layers

| Layer | Where | Runner | Scope |
| --- | --- | --- | --- |
| Rust unit | `crates/*/src/<module>/tests.rs` | `cargo test` | One module, internals visible |
| Rust integration | `crates/lorem-core/tests/` | `cargo test` | Public API, end-to-end generation |
| TS unit | `gui/tests/unit/` | vitest | Pure logic (`options.ts`, mock backend) |
| TS component | `gui/tests/component/` | vitest + happy-dom | DOM modules against real `index.html` markup |
| E2E | `gui/e2e/` | Playwright (Chromium) | Full frontend in a real browser |

The TUI's `app.rs` tests sit between unit and component: they drive the
real key-event handler headlessly (no terminal) and assert on state.

## Seams that make this possible

- **`lorem_core::Generator`** — `generate()` is built on the incremental
  generator, so batch and streamed output are identical for a seed; one
  integration test pins exactly that.
- **The frontend `Backend` interface** (`gui/src/backend.ts`) — inside
  Tauri the real commands run; everywhere else (vitest, Playwright, plain
  `npm run dev`) a deterministic mock stands in. The mock honors the same
  contract: same seed → same output, exact word counts, classic opener.
- **Synthetic corpora** — chain/walk unit tests build chains from tiny
  hand-written id sequences (e.g. `[0,1,2,3]`), making transitions, dead
  ends, and ender behavior exactly predictable.
- **Fresh DOM lookups** — frontend modules query elements via `byId` on
  every call instead of caching at import time, so component tests can
  rebuild the DOM per test.

## Known limits

- `tauri-driver` has no macOS support, so the native shell itself is not
  end-to-end tested; Playwright covers the frontend up to the `Backend`
  seam, and the Rust commands behind it are covered by core tests.
- The mock backend is a *contract double*, not the real Markov chain — it
  proves UI behavior, not text quality.
- Visual styling is untested apart from behavior the e2e layer can observe
  (e.g. element visibility — which caught the `[hidden]` vs author
  `display` override bug; see `docs/testing/e2e.md`).
