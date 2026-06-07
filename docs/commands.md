# Commands Reference

## Overview

Every stored macro has a **key** (a short name) and a **command** (the shell command to run). Keys are case-sensitive and must be unique.

---

## `velo add <key> <command>`

Store a new macro.

```bash
velo add dev "npm run dev"
velo add build "cargo build --release"
velo add deploy "./deploy.sh --env prod"
```

> [!TIP]
> If you don't provide arguments, `velo add` runs in interactive mode and prompts you for the key and command.

---

## `velo delete <key>`

Remove a stored macro.

```bash
velo delete dev
velo delete build
```

> [!WARNING]
> Deletion is permanent. There is no undo. The macro is removed from the JSON file immediately.

You can also delete from the interactive TUI: select a macro, press Enter, choose **Delete**, and confirm with `y`.

---

## `velo update <key> <command>`

Change the command bound to an existing key.

```bash
velo update dev "bun run dev"
velo update build "dotnet build"
```

> [!NOTE]
> If you omit the command, `velo update` enters interactive mode and shows the current command before asking for the new one.

---

## `velo list`

Display all stored macros as a table with timestamps.

```bash
velo list
```

Example output:

```
  KEY                  COMMAND                                DATE     TIME
  ════════════════════════════════════════════════════════════════════════
  dev                  npm run dev                            6/8/26   01:23
  deploy               ./deploy.sh --env prod                 6/7/26   22:30
```

---

## `velo run <key>` / `velo <key>`

Execute a stored macro immediately.

```bash
velo dev       # runs "npm run dev"
velo run dev   # same thing
```

> [!TIP]
> Omitting `run` is purely syntactic sugar. `velo dev` and `velo run dev` behave identically.

---

## Interactive TUI

Running `velo` with no arguments opens the full-screen interactive browser.

| Key | Action |
|-----|--------|
| `Up` / `Down` | Navigate the list |
| `Enter` | Select macro (shows Run / Update / Delete / Back) |
| `q` / `Esc` | Go back or quit |

When a macro is selected, the action submenu lets you:

- **Run** -- Execute the command immediately and see its output
- **Update** -- Change the command string
- **Delete** -- Remove the macro (with confirmation prompt)
- **Back** -- Return to the main macro list

---

> [!IMPORTANT]
> All data is stored in `macros.json` located at `~/.config/velo/` (Linux/macOS) or `%APPDATA%\velo\` (Windows). You can back up or edit this file directly.
