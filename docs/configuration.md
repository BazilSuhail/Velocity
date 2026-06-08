# Configuration

velo stores all macros in one JSON file. It has no config file of its own. Behavior is controlled by a few environment variables.

## Storage location

The file is named `macros.json`. velo chooses the directory as follows.

| Platform | Default directory | Default file |
|----------|-------------------|--------------|
| Linux | `$XDG_CONFIG_HOME/velo` if `XDG_CONFIG_HOME` is set, otherwise `~/.config/velo` | `~/.config/velo/macros.json` |
| macOS | `~/Library/Application Support/velo` | `~/Library/Application Support/velo/macros.json` |
| Windows | `%APPDATA%\velo` | `%APPDATA%\velo\macros.json` |
| WSL | Same as Linux (the Linux home inside the distro) | `~/.config/velo/macros.json` |

To find the file on your system:

```bash
ls -l ~/.config/velo/macros.json           # Linux (macOS: ~/Library/Application Support/velo)
```

```powershell
Get-Item "$env:APPDATA\velo\macros.json"   # Windows
```

## VELO_HOME

Set `VELO_HOME` to store macros in a different directory. velo uses `$VELO_HOME/macros.json`. The variable replaces the default directory on every platform.

```bash
export VELO_HOME="$HOME/dotfiles/velo"     # Linux, macOS, WSL
velo list                                  # reads ~/dotfiles/velo/macros.json
```

```powershell
$env:VELO_HOME = "D:\sync\velo"            # Windows, current session
[Environment]::SetEnvironmentVariable("VELO_HOME", "D:\sync\velo", "User")   # persistent
```

Use `VELO_HOME` to keep macros in a synced folder, or to run an isolated test store:

```bash
VELO_HOME="$(mktemp -d)" velo list         # empty store, nothing touched
```

The directory is created when velo first writes. Make sure you have write permission to it.

## Environment variables

| Variable | Effect |
|----------|--------|
| `VELO_HOME` | Directory that holds `macros.json`. Overrides the default location. |
| `VELO_ASCII` | Set to any value (for example `1`) to force ASCII glyphs in the TUI. Use this when boxes or symbols render as `?` or garbage. |
| `NO_COLOR` | Set to any value to disable color output. Follows the [no-color.org](https://no-color.org) convention. |
| `CLICOLOR_FORCE` | Set to `1` to keep color when output is piped or redirected. |
| `COLORTERM` | Set to `truecolor` to enable the 24-bit palette. Terminals that support it usually set this for you. |
| `TERM` | When set to `dumb`, velo disables color. |

Color is off when output is not a terminal, unless `CLICOLOR_FORCE=1` is set.

```bash
NO_COLOR=1 velo list                       # plain text
CLICOLOR_FORCE=1 velo list | less -R       # keep color through a pipe
COLORTERM=truecolor velo                   # force the truecolor palette
```

## Macro file format

`macros.json` is a JSON object with one `macros` map. Each key is a macro name, and each value holds its command and timestamps.

```json
{
  "macros": {
    "build": {
      "command": "cargo build --release",
      "created_at": "2026-06-07 10:15:00",
      "updated_at": "2026-06-07 10:15:00"
    },
    "dev": {
      "command": "npm run dev",
      "created_at": "2026-06-08 09:02:11",
      "updated_at": "2026-06-20 16:40:03"
    }
  }
}
```

### Schema

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `macros` | object | yes | Map of key to macro. velo sorts the keys when it lists them. |
| `macros.<key>.command` | string | yes | The shell command to run. |
| `macros.<key>.created_at` | string | yes | Local time, `YYYY-MM-DD HH:MM:SS`. |
| `macros.<key>.updated_at` | string | yes | Local time, `YYYY-MM-DD HH:MM:SS`. |

Keys are unique by construction, because they are the object's keys. A key must be non-empty, contain no whitespace, and contain none of `/ \ " '` or control characters.

Editing the file by hand is supported. Keep the format valid JSON. velo validates the file when it loads it.

An empty store is valid:

```json
{
  "macros": {}
}
```

## Writes are atomic

velo never edits `macros.json` in place. It writes the new content to a temporary file in the same directory, then renames it over the original. An interrupted write leaves the previous file intact.

## Corrupt files

If `macros.json` cannot be parsed, velo:

- refuses to write to it,
- reports the path of the file,
- leaves the file unchanged.

Read operations report the error. Write operations (`add`, `update`, `delete`, and saves from the TUI) fail with exit code `1`. Nothing is overwritten. To recover, see [Troubleshooting](troubleshooting.md#corrupt-macrosjson).

## Legacy format migration

Versions before 0.2.0 stored macros as a flat object that maps each key to its command:

```json
{
  "dev": "npm run dev",
  "build": "cargo build --release"
}
```

velo detects this format on load and converts it to the current schema automatically. You do not need to run a migration step.

Keep a copy of the old file before you first run 0.2.0:

```bash
cp ~/.config/velo/macros.json ~/velo-macros-legacy.json
```

## Backup

The whole store is one file. Back it up by copying it.

```bash
cp ~/.config/velo/macros.json ~/backups/macros-$(date +%F).json       # Linux (on macOS, use ~/Library/Application Support/velo)
```

```powershell
Copy-Item "$env:APPDATA\velo\macros.json" "$HOME\Backups\macros-$(Get-Date -Format yyyy-MM-dd).json"
```

Restore by copying the backup back over `macros.json` while velo is not running.

Validate a backup before you rely on it:

```bash
python3 -m json.tool ~/backups/macros-2026-06-07.json > /dev/null && echo "valid"
```

Sync the directory with your own tool (Git, Dropbox, or a dotfiles repo). Point `VELO_HOME` at the synced folder to share one store across machines. Avoid editing the file on two machines at the same time.
