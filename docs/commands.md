# Commands Reference

This page documents every `velo` command and flag. For an overview, see the [README](../README.md). For the TUI, see [Keybindings](keybindings.md).

## Synopsis

```text
velo                                  Open the TUI (in a terminal) or list macros (piped)
velo <KEY>                            Run a macro (shorthand for velo run <KEY>)
velo run <KEY>                        Run a macro
velo add [KEY] [COMMAND]              Store a new macro
velo update [KEY] [COMMAND]           Change the command of an existing macro
velo delete [KEY] [-y|--yes]          Remove a macro
velo list                             Print all macros as a table
velo --help | -h                      Show help
velo --version                        Show the version
```

## Concepts

- A **macro** is a named shell command. Its **key** is the name you type. Its **command** is the text that runs.
- Keys must be unique. Adding a key that already exists fails. Use `velo update` to change it.
- Keys cannot be empty, cannot contain whitespace, and cannot contain any of these characters: `/ \ " '` or control characters.
- Subcommand names (`add`, `delete`, `list`, `run`, `update`) take precedence over keys with the same name. To run a macro named `list`, use `velo run list`.

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| The macro's own code | When running a macro, velo exits with the code the macro returned |
| `1` | Error: macro not found, storage error, or invalid input |
| `2` | Usage error, such as a non-interactive `delete` without `--yes` |
| `127` | The macro's shell could not start. The command never ran. |

---

## `velo` (no arguments)

In a terminal, opens the interactive TUI. See [Keybindings](keybindings.md).

When the output is not a terminal (for example, `velo | cat`), velo prints the macro list instead of starting the TUI.

```bash
velo
velo | cat        # prints the list
```

---

## `velo <KEY>` and `velo run <KEY>`

Run a stored macro. velo starts the command through the platform shell (`cmd /C` on Windows, `sh -c` on Linux and macOS), waits for it to finish, and exits with the macro's exit code.

```bash
velo dev
velo run dev          # same as above; use this when the key matches a subcommand name
```

Example: a macro named `test` runs the test suite. When the suite fails, `velo test` exits with the suite's non-zero code, so scripts can check it:

```bash
velo test || echo "tests failed"
```

If the key does not exist, velo prints an error to stderr and exits with `1`.

```bash
velo nope      # exits 1: no macro named 'nope'
```

> [!NOTE]
> Use `velo run <KEY>` for keys that share a name with a subcommand, such as `velo run list` or `velo run delete`.

---

## `velo add [KEY] [COMMAND]`

Store a new macro.

```bash
velo add dev "npm run dev"
velo add build "cargo build --release"
velo add deploy "./deploy.sh --env prod"
```

With both arguments, velo saves the macro immediately. With no arguments, velo prompts for the key, then for the command. Run `velo add` with no arguments in a terminal. Scripts must pass both arguments.

Errors (exit `1`):

| Case | Example |
|------|---------|
| Empty key | `velo add "" "echo hi"` |
| Key with whitespace | `velo add "my key" "echo hi"` |
| Key with `/ \ " '` or a control character | `velo add "a/b" "echo hi"` |
| Key already exists | `velo add dev "make"` when `dev` exists. Use `velo update dev "make"` instead. |

> [!TIP]
> Quote commands that contain spaces, pipes, or `$` variables. Otherwise your shell expands them before velo sees them.

```bash
velo add logs "tail -f app.log | grep ERROR"    # quoted, velo stores the whole string
```

---

## `velo update [KEY] [COMMAND]`

Change the command of an existing macro.

```bash
velo update dev "bun run dev"
velo update build "dotnet build"
```

With both arguments, velo replaces the command. With no arguments, velo prompts for the key and then for the new command, showing the current command first.

Updating a key that does not exist fails with exit `1`. Use `velo add` to create it.

```bash
velo update missing "echo hi"
# error: no macro named 'missing' (exit 1)
```

---

## `velo delete [KEY] [-y|--yes]`

Remove a macro.

```bash
velo delete dev          # asks for confirmation in a terminal
velo delete dev --yes    # no prompt; required in scripts
velo delete dev -y       # short form
```

| Context | Without `--yes` | With `--yes` |
|---------|-----------------|--------------|
| Terminal | Asks you to confirm and waits for `y` | Deletes without asking |
| Non-interactive (piped, CI, cron) | Refuses with a message and exits `2` | Deletes without asking |

If you omit `KEY` in a terminal, velo prompts for it.

> [!WARNING]
> Deletion is permanent. velo removes the macro from `macros.json` immediately. Keep a backup of the file if you need an undo. See [Configuration](configuration.md#backup).

Deleting a missing key exits with `1`.

---

## `velo list`

Print every macro as a table, sorted by key.

```bash
velo list
```

The table has three columns:

| Column | Content |
|--------|---------|
| `KEY` | The macro key |
| `COMMAND` | The command text. Long commands are truncated with an ellipsis. |
| `UPDATED` | When the macro was last changed |

The exact layout depends on your terminal width.

Use `velo list | cat` for plain output without the TUI.

---

## `velo --help` and `velo --version`

```bash
velo --help        # or velo -h
velo --version     # prints the version, for example "velo 0.2.0"
```

`velo <subcommand> --help` shows the options of one subcommand.

---

## Scripting examples

Store a macro from a script, then run it and check the result:

```bash
#!/usr/bin/env bash
set -euo pipefail

velo update build "cargo build --release" 2>/dev/null || velo add build "cargo build --release"
if velo build; then
  echo "build succeeded"
else
  echo "build failed with exit code $?"
fi
```

PowerShell:

```powershell
velo add lint "cargo clippy --all-targets -- -D warnings"
velo lint
if ($LASTEXITCODE -ne 0) { Write-Error "lint failed" }
```

Delete without a prompt:

```bash
velo delete old-key --yes
```

Related: [Configuration](configuration.md), [Troubleshooting](troubleshooting.md).
