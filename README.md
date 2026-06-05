# lorem-ipsum

Markov-chain lorem ipsum generator with three front ends sharing one Rust
core. Five themes (classic Latin, tech startup, pirate, corporate buzzword,
cosmic), three generation modes (paragraphs, sentences, words), seedable
and reproducible everywhere — plus an infinite streaming mode for piping
endless prose wherever you need it.

| Front end | What it is |
| --- | --- |
| `lorem` (CLI) | JSON output, saved defaults, `--infinite` streaming (txt or JSON Lines) |
| `lorem-tui` | ratatui form + scrollable output table |
| Lorem Ipsum Studio (GUI) | Tauri 2 + TypeScript desktop app, royal purple editorial look |

## Quick start

```sh
# CLI
cargo run -p lorem-cli -- --theme pirate -n 2 --seed 42
cargo run -p lorem-cli -- --infinite --mode sentences > lorem.txt   # Ctrl-C when done

# TUI (interactive)
cargo run -p lorem-tui

# GUI
cd gui && npm install && npm run tauri dev

# Tests
cargo test --workspace          # Rust
cd gui && npm test              # frontend unit + component
cd gui && npm run test:e2e      # frontend e2e (npx playwright install chromium once)
```

Every output reports the seed it used — feed it back in to reproduce the
output exactly.

## Documentation

| Doc | Contents |
| --- | --- |
| [docs/core/architecture.md](docs/core/architecture.md) | the generation pipeline, modes, invariants, adding a theme |
| [docs/cli/architecture.md](docs/cli/architecture.md) | CLI modules, flags, streaming formats |
| [docs/tui/architecture.md](docs/tui/architecture.md) | TUI modules, layout, key bindings |
| [docs/gui/architecture.md](docs/gui/architecture.md) | Tauri backend, frontend modules, the mock-backend seam, styling gotchas |
| [docs/testing/design.md](docs/testing/design.md) | test strategy, layers, seams, known limits |
| [docs/testing/how-to.md](docs/testing/how-to.md) | commands quick reference ([unit](docs/testing/unit.md) · [component](docs/testing/component.md) · [e2e](docs/testing/e2e.md)) |

## License

MIT © Alleato LLC — see [LICENSE](LICENSE).
