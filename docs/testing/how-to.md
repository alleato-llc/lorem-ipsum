# Testing how-to

Quick reference. Deeper guides: [unit](unit.md) · [component](component.md)
· [e2e](e2e.md). Strategy: [design](design.md).

CI (`.github/workflows/ci.yml`) runs every layer on pushes to `main` and on
pull requests: a Rust job (workspace build + all cargo tests, with the
Tauri system libraries installed) and a frontend job (tsc, vite build,
vitest, Playwright e2e).

## Run everything

```sh
cargo test --workspace          # all Rust tests (unit + integration)
cd gui && npm test              # frontend unit + component (vitest)
cd gui && npm run test:e2e      # frontend end-to-end (Playwright)
```

## One-time setup

```sh
cd gui && npm install                    # vitest, happy-dom, playwright
cd gui && npx playwright install chromium
```

## Narrower runs

```sh
cargo test -p lorem-core                 # one crate
cargo test -p lorem-core walk            # tests matching a name
cargo test -p lorem-core --test generation   # just the integration suite
cd gui && npm run test:watch             # vitest watch mode
cd gui && npx vitest run tests/unit      # one frontend layer
cd gui && npx playwright test --headed   # watch the browser
```

## Where a new test belongs

- Logic inside one Rust module → that module's `tests.rs`
  (`src/<module>/tests.rs`, declared with `#[cfg(test)] mod tests;`).
- Behavior across core modules / the public API →
  `crates/lorem-core/tests/generation.rs`.
- Pure TS logic → `gui/tests/unit/`.
- DOM behavior of a frontend module → `gui/tests/component/`.
- A user-visible flow through the whole UI → `gui/e2e/app.spec.ts`.
