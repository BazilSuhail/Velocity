# Development Guide

This guide is for contributors and maintainers. It covers the architecture, the TUI event loop, build and test commands, how to add a command or key, and the release process.

## Requirements

- Rust 1.85 or later. The crate uses edition 2024, which needs 1.85.
- `cargo`, `rustfmt`, and `clippy` (install with `rustup component add rustfmt clippy`).
- A terminal. The TUI does not run in CI or in a pipe.

## Build, test, and lint

```bash
cargo build                                       # debug build
cargo build --release                             # optimized build, target/release/velo
# No automated test suite yet. Check changes by running the affected flow by hand.
cargo clippy --all-targets -- -D warnings         # lint; warnings are errors
cargo fmt --check                                 # formatting check
cargo fmt                                         # apply formatting
```

Run the same three checks before you push:

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings
```

The `check` job in `.github/workflows/release.yml` runs clippy and `cargo fmt --check` on every push and pull request.

### Testing against a scratch store

Use `VELO_HOME` so manual tests never touch your real macros:

```bash
export VELO_HOME="$(mktemp -d)"
cargo run -- add demo "echo hello"
cargo run -- demo
cargo run -- list
```

```powershell
$env:VELO_HOME = Join-Path $env:TEMP ("velo-" + [guid]::NewGuid())
cargo run -- add demo "echo hello"
cargo run -- demo
```

## Architecture

### Module tree

```text
src/
  main.rs              Entry point; parses arguments and dispatches to a service or the TUI
  platform.rs          Platform detection and the shell used to run macros (sh -c, cmd /C)
  services/
    mod.rs             Service module exports
    add.rs             velo add
    update.rs          velo update
    delete.rs          velo delete (confirmation and --yes)
    list.rs            velo list
    run.rs             velo <KEY> and velo run <KEY>; returns the macro's exit code
  utils/
    mod.rs
    storage.rs         Load and save macros.json; schema, legacy migration, atomic write, corruption guard
    display.rs         Table rendering and plain or colored output helpers
  tui/
    mod.rs             TUI entry point and terminal setup and teardown
    app.rs             Application state and the reducer
    event.rs           Input events and the event loop
    keymap.rs          Maps key events to actions, per mode
    ui.rs              Rendering with ratatui (layout, list, details, footer, overlays)
    theme.rs           Color palettes (basic, truecolor) and the ASCII fallback
```

Keep layers separate. Parsing and dispatch live in `main.rs`. Business logic lives in `services/`. Persistence lives in `utils/storage.rs`. The TUI reads state and sends actions. It does not write files directly.

> [!NOTE]
> The module names above follow the 0.2.0 layout. Check `src/` before you rely on a path, because the TUI moved from `services/tui.rs` into the `tui/` module during the revamp.

### Storage

`utils/storage.rs` is the only module that touches `macros.json`.

- It resolves the directory from `VELO_HOME`, then the platform config directory.
- It loads the file, detects the legacy flat format, and migrates it to the current schema.
- It writes through a temporary file in the same directory and renames it over the original. The rename is the commit point.
- If the file fails to parse, it returns an error that names the path. Writes stop, so the file is never overwritten.

Do not add a second writer. All saves go through the storage functions so that the corruption guard applies everywhere.

### Exit codes

| Code | Source |
|------|--------|
| `0` | Success |
| macro's code | `services/run.rs` returns the child process status |
| `1` | Errors: not found, storage error, invalid input |
| `2` | Usage errors, such as non-interactive `delete` without `--yes` |

Return these from the service layer. Do not call `std::process::exit` deep in the code, so tests can check the result.

## TUI: event loop and reducer

The TUI follows a unidirectional loop: events become actions, a reducer turns the current state and an action into a new state, and the renderer draws the state.

```text
terminal event -> event.rs -> keymap.rs (mode + key -> Action)
                                     |
                                     v
                          app.rs reducer: (State, Action) -> State, sets dirty flag
                                     |
                                     v
                     if dirty: ui.rs draws State with ratatui, clears dirty flag
```

### The loop

1. `event.rs` waits for the next terminal event.
2. The event is translated to an `Action` by `keymap.rs`. The keymap depends on the current mode: Browse, Input, Confirm, Filter, or Help.
3. The reducer in `app.rs` applies the action. It updates the state and marks the state as dirty when something visible changed.
4. If the state is dirty, `ui.rs` draws one frame and the dirty flag is cleared.
5. Events that change nothing (for example, a key that does nothing in the current mode) do not redraw.

### Dirty flag

The dirty flag keeps the TUI from redrawing when nothing changed. The loop waits for input with a 500 ms timeout and redraws only after a key press, a resize, or a state change. This keeps idle CPU use near zero and avoids flicker.

When you add state that affects the screen, set the dirty flag in the reducer. If you forget, the screen will not update until the next unrelated change, so test the new state visually.

### Running a macro from the TUI

`Enter` in Browse mode:

1. Leaves the alternate screen and disables raw mode.
2. Runs the macro through the same path as `velo <KEY>`.
3. Prints `Press Enter to return to velo...` and waits for a line of input.
4. Re-enables raw mode, re-enters the alternate screen, and redraws.

Restore the terminal on every path, including errors. Use a guard or a `finally`-style cleanup.

## Adding a command

1. Add a variant to the clap definition in `main.rs`, with its flags and help text.
2. Create `src/services/<name>.rs` with a function that takes the parsed arguments and the storage handle, and returns an exit code.
3. Export it from `src/services/mod.rs` and dispatch to it in `main.rs`.
4. Use `utils/storage.rs` for all reads and writes. Use `utils/display.rs` for output.
5. Add tests next to the code, using a temporary `VELO_HOME`.
6. Document it in `docs/commands.md` and in the command table in `README.md`.
7. Add a `CHANGELOG.md` entry under the next version.

## Adding a key to the TUI

1. Add an `Action` variant in `src/tui/keymap.rs`, if the key needs a new one, and handle it in the reducer in `src/tui/app.rs`.
2. Map the key to the action in `keymap.rs`, in the right mode. Do not map keys in `ui.rs` or `event.rs`.
3. Handle the action in the reducer in `app.rs`. Set the dirty flag if the screen changes.
4. Show the key in the footer or the help overlay, which `ui.rs` builds from the keymap.
5. Document it in `docs/keybindings.md`.

Keep the keymap as the single source of truth. Help text and footer hints come from it, so they cannot drift from the real bindings.

## Colors and glyphs

- `theme.rs` owns the palettes. Use `COLORTERM=truecolor` to pick the 24-bit palette. Otherwise use the basic palette.
- Honor `NO_COLOR`, `TERM=dumb`, and non-terminal output. Do not hard-code escape codes.
- `VELO_ASCII=1` forces the ASCII fallback. Any new glyph needs an ASCII entry.

## Release process

Releases are built by `.github/workflows/release.yml`.

1. **Check.** Every push and pull request runs `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`.
2. **Build.** Pushes build release binaries for:
   - `x86_64-unknown-linux-gnu`
   - `aarch64-unknown-linux-gnu` (on the `ubuntu-24.04-arm` runner)
   - `x86_64-apple-darwin`
   - `aarch64-apple-darwin`
   - `x86_64-pc-windows-msvc`
3. **Publish.**
   - A push to `main` replaces the `latest` release with the new binaries and a `checksums.txt`.
   - A tag `vX.Y.Z` creates a versioned GitHub release with the same files.

To cut a version:

1. Bump `version` in `Cargo.toml` and update `Cargo.lock` with `cargo build`.
2. Move the `Unreleased` section in `CHANGELOG.md` to the new version with the date.
3. Run the three checks locally.
4. Commit, then tag and push:

   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

5. Confirm that the release has all five archives and `checksums.txt`. Test one archive on each platform you can reach.

The installers download from these release URLs. Do not rename the archives without updating `scripts/install.sh` and `scripts/install.ps1`.

## Code style

- Run `cargo fmt` before you commit.
- Keep clippy clean with `-D warnings`. Do not add `#[allow]` without a comment that says why.
- Keep the TUI free of blocking work. Long operations belong in the service layer, with a status message in the UI.
- Write error messages that name the file, key, or flag involved.
