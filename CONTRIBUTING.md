# Contributing to velo

Thank you for helping with velo. This guide explains how to report a bug, propose a change, set up a development build, and get a pull request merged.

velo is maintained by [Bazil Suhail](https://bazilsuhail.netlify.app). It is licensed under the [MIT License](LICENSE).

## Ways to help

- **Report bugs.** Open an issue with the details listed below.
- **Improve the docs.** Fix an unclear step, a wrong command, or a missing platform note in `README.md` or `docs/`.
- **Test on your platform.** Run velo on Windows, Linux, macOS, or WSL, and report what you find. Manual testing is the main check for now (see [Checks](#checks-every-pr-must-pass)).
- **Fix a bug or add a feature.** Open an issue first for anything large (see [Proposing a change](#proposing-a-change)).

## Reporting a bug

Open an issue at <https://github.com/BazilSuhail/Velocity/issues>. Include:

1. **Operating system and version.** For example, Windows 11, Ubuntu 24.04, or macOS 15 on Apple Silicon.
2. **Terminal and shell.** For example, Windows Terminal with PowerShell, or iTerm2 with zsh.
3. **velo version.** Run `velo --version`.
4. **The exact command** you ran, including any `VELO_HOME`, `VELO_ASCII`, or `NO_COLOR` settings.
5. **The output and exit code.** Run `echo $?` (bash, zsh) or `$LASTEXITCODE` (PowerShell) right after the command.
6. **What you expected** and what happened instead.

For a TUI problem, describe the keys you pressed and the screen you saw. Screenshots help. If the problem involves your store, say whether `macros.json` was valid JSON before the problem. Do not paste private commands or secrets. Replace them with placeholders.

For a security issue, do not open a public issue. Contact the maintainer through the link on the [author site](https://bazilsuhail.netlify.app).

## Proposing a change

- **Small fixes** (a typo, a clear bug, a doc correction): open a pull request directly.
- **Large changes** (a new command, a new storage format, a new TUI mode, a new dependency, or a change to the release process): open an issue first and describe the problem and the proposed approach. This avoids spending time on work that does not fit the project.

Keep each pull request to one purpose. Separate refactors from behavior changes.

## Development setup

You need Rust 1.85 or newer, because the crate uses edition 2024.

1. Install Rust with [rustup](https://rustup.rs), then add the tools used by the checks:

   ```bash
   rustup component add rustfmt clippy
   ```

2. Clone and build:

   ```bash
   git clone https://github.com/BazilSuhail/Velocity.git
   cd Velocity
   cargo build
   ```

3. Run the debug binary against a scratch store, so your real macros are not touched:

   ```bash
   export VELO_HOME="$(mktemp -d)"            # bash, zsh
   cargo run -- add demo "echo hello"
   cargo run -- demo
   cargo run -- list
   ```

   ```powershell
   $env:VELO_HOME = Join-Path $env:TEMP ("velo-" + [guid]::NewGuid())
   cargo run -- add demo "echo hello"
   cargo run -- demo
   cargo run -- list
   ```

   Run `cargo run` with no arguments in a real terminal to open the TUI. The TUI does not start when output is piped.

For the full setup guide, see [docs/setup.md](docs/setup.md).

## Code layout

The crate has three layers. Parsing and dispatch live in `src/main.rs`. Commands live in `src/services/`. Persistence lives in `src/utils/storage.rs`, and the TUI lives in `src/tui/`.

```text
src/main.rs            clap definitions, help text, dispatch
src/platform.rs        shell used for macros, color and terminal detection
src/services/          one file per command (add, update, delete, list, run)
src/utils/storage.rs   the only code that reads or writes macros.json
src/utils/display.rs   output helpers
src/tui/               keymap.rs, app.rs (state and reducer), ui.rs, event.rs, theme.rs
```

The full module tree and the TUI event loop are in [docs/development.md](docs/development.md).

Rules that reviewers will check:

- Only `src/utils/storage.rs` touches `macros.json`. Do not add a second writer.
- The TUI reads state and sends actions. It does not write files directly.
- Services return exit codes. Do not call `std::process::exit` deep in the code.
- Map TUI keys in `src/tui/keymap.rs` only. Do not map keys in `ui.rs` or `event.rs`.
- Every new glyph needs an ASCII fallback, and every new color needs to respect `NO_COLOR` and `TERM=dumb`.

## Checks every PR must pass

Run these from the repository root before you open a pull request:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release
```

- `cargo fmt --check` fails if formatting differs. Run `cargo fmt` to fix it.
- `cargo clippy --all-targets -- -D warnings` treats every warning as an error. Do not add `#[allow(...)]` without a comment that explains why.
- `cargo build --release` confirms the release build still compiles.

CI runs the first two checks on every push and pull request (the `check` job in `.github/workflows/release.yml`).

### Tests

The repository does not have an automated test suite yet. `cargo test` runs but finds no tests. Do not claim that a change is tested by `cargo test`. Instead:

- Run the flow your change touches, by hand, against a scratch `VELO_HOME`. For example, after a storage change, run `add`, `list`, `update`, `delete`, and a corrupt-file case.
- For a TUI change, run `cargo run` in a terminal and walk through every mode you touched. Resize the window to check both layouts (under 60 columns, and 100 or more).
- Test on at least one OS. Say which one in the PR. Changes to `platform.rs` or the shell code should be checked on Windows and on Linux or macOS.
- If you add non-trivial logic, add unit tests next to the code. They are welcome, and they will give the project its first tests.

## Commit messages

- Use a short, imperative subject in the form `type: summary`, for example `fix: refuse writes when macros.json is corrupt`.
- The types used in this repository are `feat`, `fix`, `refactor`, `docs`, and `chore`.
- Keep the subject under about 72 characters. Add a body if the reason is not obvious from the diff.
- Make one logical change per commit. Squash fixup commits before you ask for review.

## Pull request checklist

Before you request review, confirm that:

- [ ] The change has one purpose, and the description says what and why.
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.
- [ ] You ran the affected flow by hand and say on which OS.
- [ ] Documentation is updated where behavior changed: `docs/commands.md`, `docs/keybindings.md`, `docs/configuration.md`, and the `README.md` command table.
- [ ] `CHANGELOG.md` has an entry under the next version.
- [ ] No secrets, personal paths, or real macros are in the diff or the screenshots.

## How to add a command

Use the existing commands in `src/services/` as a model.

1. **Define it in `src/main.rs`.** Add a variant to the `Commands` enum with its flags and a doc comment. The doc comment becomes the help text.
2. **Write the service.** Create `src/services/<name>.rs` with a function that takes the parsed arguments and returns an exit code. Follow the exit code table in [docs/commands.md](docs/commands.md#exit-codes): `0` success, `1` errors, `2` usage.
3. **Export and dispatch.** Add `pub mod <name>;` in `src/services/mod.rs`, then add a match arm in `main()` in `src/main.rs`.
4. **Use the shared helpers.** Read and write through `src/utils/storage.rs` (`load_or_report`, `save_or_report`). Print through `src/utils/display.rs`. Use `arg_or_prompt` if the command should prompt in a terminal and fail with exit `2` in scripts.
5. **Test it by hand** with a scratch `VELO_HOME`, including the error cases.
6. **Document it.** Add the command to `docs/commands.md`, the synopsis block at its top, and the command table in `README.md`. If you add it to the help text in `print_colored_help()` in `main.rs`, keep that list in step too.
7. **Add a CHANGELOG entry** under the next version.

Keys and command names must follow the rules in [docs/commands.md](docs/commands.md#concepts). Subcommand names win over keys with the same name, so pick a name that does not collide with a subcommand.

## How to add a TUI key

The keymap is the single source of truth for keys. The footer and the help overlay read from it.

1. **Add an action** if you need one. The `Action` enum is in `src/tui/keymap.rs`.
2. **Map the key** in `src/tui/keymap.rs`. Add it to the function for the mode (`browse`, `actions`, `text`, or the confirm and help arms in `resolve`).
3. **Handle the action** in the reducer in `src/tui/app.rs`. Set the dirty flag if the screen changes, or the screen will not redraw.
4. **Show the key in the footer.** Add it to the matching `*_HINTS` table in `src/tui/keymap.rs`. The Browse table also feeds the help overlay, so keep it short.
5. **Document it** in `docs/keybindings.md`, in the table for that mode.
6. **Check it by hand** in a real terminal: the key, the footer, the help overlay, and the narrow layout (under 60 columns).

## Code of conduct

Be respectful and constructive. Assume good intent, and keep criticism about the work, not the person. Give credit where it is due, and be patient with people who are new to Rust or to open source.

Harassment, insults, personal attacks, discrimination, and publishing someone's private information are not acceptable in issues, pull requests, or discussions. Maintainers may edit or remove content that breaks these rules, and may block people who repeat them. To report a problem, contact the maintainer through the [author site](https://bazilsuhail.netlify.app).

This summary is not a full code of conduct. If the project adopts a longer policy, that policy will replace this section.

## License

velo is released under the [MIT License](LICENSE). By submitting a contribution, you agree that it is licensed under the same MIT terms, and that you have the right to submit it. Keep the copyright notice in files you copy from elsewhere, and only contribute code you can license this way.
