# Contributing

Thanks for helping out! This document covers the workflow and the
conventions the codebase holds itself to. Architecture and testing details
live under [docs/](docs/) — this page links rather than repeats.

## Setup

Prerequisites: Rust (stable), Node 22+, and on Linux the Tauri system
libraries (webkit2gtk, gtk — see the apt list in
[.github/workflows/ci.yml](.github/workflows/ci.yml)). macOS needs no
extra system packages.

```sh
git clone git@github.com:alleato-llc/lorem-ipsum.git
cd lorem-ipsum
cargo build --workspace        # lorem-gui pulls ~400 crates the first time
cd gui && npm install          # frontend + test tooling
npx playwright install chromium   # once, for e2e
```

Day-to-day commands are in the [README](README.md#quick-start) and
[docs/testing/how-to.md](docs/testing/how-to.md).

## Ground rules

1. **Generation logic lives in `lorem-core`.** The CLI, TUI, and GUI are
   thin shells; if a behavior change touches a front end's generation
   output, it belongs in core so all three pick it up.
2. **One file, one responsibility.** Modules stay small and focused —
   match the existing decomposition
   ([core](docs/core/architecture.md) · [cli](docs/cli/architecture.md) ·
   [tui](docs/tui/architecture.md) · [gui](docs/gui/architecture.md)).
3. **Tests are separate files, never inline.** Rust modules declare
   `#[cfg(test)] mod tests;` with the tests in a sibling
   `<module>/tests.rs`; frontend tests live under `gui/tests/` and
   `gui/e2e/`. New code comes with tests at the right layer — see
   ["where a new test belongs"](docs/testing/how-to.md#where-a-new-test-belongs).
4. **Don't break the invariants** listed in
   [docs/core/architecture.md](docs/core/architecture.md#invariants) —
   notably: seeds stay reproducible and JSON-safe (u32 range), options
   stay `#[serde(default)]`-compatible, settings never persist a seed.
5. **Update the docs with the change.** Behavior changes update the
   matching `docs/` page (and `CLAUDE.md`, the agent-facing summary).
   Doc-only pushes intentionally skip CI.

## Making changes

- Branch from `main`; keep PRs focused on one concern.
- Commit messages: imperative subject line, body explaining *why* when it
  isn't obvious (match `git log`'s existing style).
- Before opening a PR, run what CI runs:

  ```sh
  cargo test --workspace
  cd gui && npx tsc --noEmit && npx vite build && npm test && npm run test:e2e
  ```

- CI (see [docs/cicd.md](docs/cicd.md)) must be green; a force-push to
  your branch cancels the superseded run automatically.

## Common tasks

- **Adding a theme** — corpus file + one enum variant; recipe in
  [docs/core/architecture.md](docs/core/architecture.md#adding-a-theme).
  Remember the GUI mock backend keeps its own small theme list for
  browser-only mode.
- **Changing the palette** — the GUI's CSS variables
  (`gui/src/styles.css`) and the TUI's `theme.rs` constants mirror each
  other; change both.
- **Touching CI** — expect one slow run after changing cargo env flags
  (cache key invalidation); details in [docs/cicd.md](docs/cicd.md).

## License

MIT (see [LICENSE](LICENSE)). By contributing you agree your contributions
are licensed under the same terms.
