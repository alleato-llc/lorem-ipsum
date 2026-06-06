# CI/CD

One GitHub Actions workflow, `.github/workflows/ci.yml`, runs every test
layer. There is no CD/release pipeline yet (see [Releases](#releases)).

## Triggers

- Pushes to `main` and all pull requests.
- **Doc-only changes skip CI entirely** (`paths-ignore`: `docs/**`,
  `**.md`) — a documentation push produces no run, and that's expected.
- A new push to the same ref **cancels any still-running build**
  (`concurrency` with `cancel-in-progress`), so rapid pushes don't queue.

## Jobs

Both run on `ubuntu-latest`, in parallel.

### Rust tests (~49s warm)

1. Install Tauri's system libraries (webkit2gtk, gtk, appindicator,
   librsvg…) — required because `cargo test --workspace` compiles
   `lorem-gui`, and `tauri::generate_context!` won't build without them.
2. Stable toolchain (`dtolnay/rust-toolchain`).
3. `Swatinem/rust-cache` — caches the cargo registry and `target/`.
4. `cargo test --workspace` (builds everything it needs; no separate
   build step).

### Frontend tests (~49s warm)

1. Node 22 with the npm cache keyed on `gui/package-lock.json`.
2. `npm ci`, `tsc --noEmit`, `vite build`, `npm test` (vitest unit +
   component).
3. Playwright: the Chromium browser is cached keyed on the
   `@playwright/test` version from the lockfile. On a cache hit only the
   system dependencies are (re)installed; on a miss
   `playwright install --with-deps chromium` does both.
4. `npm run test:e2e`.

## Build environment tuning

```yaml
env:
  CARGO_INCREMENTAL: 0        # incremental artifacts are useless on throwaway runners
  CARGO_PROFILE_DEV_DEBUG: 0  # faster cold compiles, much smaller cache artifact
```

Changing either invalidates the rust-cache key — expect one slow run after
touching them.

## What's cached, what isn't

| Cost | Cached? | Notes |
| --- | --- | --- |
| Cargo registry + target dir | ✅ rust-cache | cold ~2m40s → warm ~7s of build |
| npm packages | ✅ setup-node | `npm ci` ~4s warm |
| Playwright Chromium | ✅ actions/cache | keyed on the Playwright version; ~break-even on time, but immune to slow CDN days |
| Tauri apt packages (~28s) | ❌ | stock runners can't cache apt; options below |
| Playwright system deps (~16s) | ❌ | same |

The ~44s of apt installs is the current floor. If it ever matters:
a third-party apt-cache action (`awalsh128/cache-apt-pkgs-action`,
supply-chain tradeoff) or a prebuilt container image with the system deps
baked in. At two ~49s jobs, neither is worth the complexity yet.

## Watching runs

```sh
gh run list --limit 5
gh run watch <run-id> --exit-status
gh run view <run-id> --json jobs   # per-step timings
```

The README badge tracks the latest run on `main`.

## Releases

Not set up. Before adding one:

- `tauri build` needs real icons (`npx tauri icon` from real art) and
  `bundle.active: true` in `gui/src-tauri/tauri.conf.json` — the committed
  icon is a generated placeholder.
- CLI/TUI binaries would be `cargo build --release` matrix builds
  (macOS/Linux/Windows) attached to a tagged release.
- The workspace release profile already strips symbols and enables LTO.
