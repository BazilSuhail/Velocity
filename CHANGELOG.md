# Changelog

## [0.2.0] - Unreleased

### Changed

- Rebuilt the interactive TUI on ratatui. The layout, footer hints, and help overlay come from one keymap.
- Responsive layout: the full VELO banner shows at 60+ columns and 20+ rows, the UPDATED column at 60+ columns, and the details pane at 100+ columns. The TUI needs at least 30x8.
- Enter opens an actions menu (Run, Update, Delete, Back); e and d still work directly.
- Faster redraw: the screen redraws only when state changes (dirty flag).
- New TUI keys: Home/End (and g/G), PageUp/PageDown, `/` filter, `?` help, `e` edit, and `d` delete with confirmation.
- Storage writes are atomic (temporary file, then rename).
- Corrupt `macros.json` is protected. velo reports the path, refuses to write, and leaves the file unchanged.
- Legacy flat-format `macros.json` files are migrated automatically on load.
- `velo list` prints a table with KEY, COMMAND, and UPDATED columns, sorted by key.
- Subcommand names take precedence over keys with the same name. Use `velo run <KEY>` to run such a key.

### Added

- `velo run <KEY>` to run a macro explicitly.
- `velo delete --yes` (`-y`) to skip the confirmation prompt. Non-interactive delete without `--yes` refuses and exits with code 2.
- `VELO_HOME` to choose the directory that holds `macros.json`.
- `NO_COLOR` support, `TERM=dumb` handling, and `CLICOLOR_FORCE=1` to keep color when piping.
- `COLORTERM=truecolor` enables the 24-bit palette.
- `VELO_ASCII` (any value) forces ASCII glyphs.
- Non-interactive mode: `velo` prints the list when output is not a terminal.
- Documented exit codes: 0 success, the macro's code when running, 1 for errors, and 2 for usage errors.
- `aarch64-unknown-linux-gnu` release build.
- Guides in `docs/`: commands, keybindings, configuration, platforms, troubleshooting, and development.

### Fixed

- Install scripts use private temporary directories, fail on download errors, and do not add duplicate PATH entries.
- The PowerShell installer stops on the first error and verifies the archive before extracting.

## [0.1.0] - 2026-06-07

- Initial release
- Interactive TUI with cursor navigation
- Add, update, delete, and run command macros
- Cross-platform support (Windows, Linux, macOS)
- Hot gradient ASCII branding
