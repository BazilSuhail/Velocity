# Setup Guide

This guide takes you from nothing installed to a working `velo`. It covers installing a release binary or building from source, putting `velo` on your `PATH`, a first run, where macros are stored, and how to update or remove velo.

For a short overview, see the [README](../README.md). For command details, see [Commands](commands.md).

## 1. What you need

### Operating systems

| OS | Versions | Macro shell |
|----|----------|-------------|
| Windows | 10 and 11 (x86_64) | `cmd /C` |
| Linux | x86_64 and aarch64 (glibc) | `sh -c` |
| macOS | Apple Silicon (aarch64) and Intel (x86_64) | `sh -c` |
| WSL | Any distribution with Linux x86_64 or aarch64 | `sh -c` (inside the distribution) |

Prebuilt binaries exist for these five targets only:

- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

On any other platform, [build from source](#4-build-from-source).

### Terminal

velo's TUI needs a real terminal. It does not start in a pipe, a redirect, cron, or CI.

| Terminal | Notes |
|----------|-------|
| Windows Terminal | Recommended on Windows. Full color, Unicode glyphs, and truecolor. |
| iTerm2 | Recommended on macOS. |
| GNOME Terminal, Konsole, Alacritty, kitty, WezTerm | Work well on Linux. Use a UTF-8 locale. |
| Terminal.app | Works. Enable **Use Unicode** in the profile if glyphs look wrong. |
| Classic Windows console (ConHost) | Works, with fewer colors. Prefer Windows Terminal. |

Use a font with box-drawing glyphs, such as Cascadia Mono on Windows, Menlo or a Nerd Font on macOS, or any monospace font with box-drawing support on Linux. If glyphs still break, see [Troubleshooting: glyphs are broken](troubleshooting.md#glyphs-are-broken).

### Rust (only for building from source)

velo uses Rust edition 2024, so you need **Rust 1.85 or newer**. Install Rust with [rustup](https://rustup.rs). Check the version:

```bash
rustc --version
cargo --version
```

On Windows, Rust's default MSVC toolchain needs the Visual Studio C++ build tools. rustup tells you what to install if they are missing. On Linux, you need a C linker such as `gcc` or `clang`.

## 2. Install a prebuilt binary

### Option A: installer script (recommended)

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.ps1 | iex
```

The script:

- downloads `velo-x86_64-pc-windows-msvc.zip` from the latest release,
- extracts `velo.exe` to `%USERPROFILE%\.velo\bin`,
- adds that folder to your user `PATH` (skipped if it is already there).

To pass options, run the script as a block instead of piping it to `iex`:

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.ps1)))
```

| Parameter | Effect |
|-----------|--------|
| `-Version <x.y.z>` | Install a specific release instead of the latest. Omit the leading `v`. |
| `-InstallDir <path>` | Install to a different folder. Default: `%USERPROFILE%\.velo\bin`. |
| `-NoPath` | Do not change the user `PATH`. |

**Linux, macOS, and WSL (bash or zsh):**

```bash
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash
```

The script detects your OS and CPU, downloads the matching archive, and installs `velo` to `~/.velo/bin`. It needs `curl`, `tar`, `grep`, `sed`, and `mktemp`.

Options:

```bash
# Pin a version (the "v" prefix is added for you)
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash -s

# Install somewhere else (for example, a folder that needs sudo)
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | INSTALL_DIR=/usr/local/bin bash
```

| Variable | Effect |
|----------|--------|
| `INSTALL_DIR` | Install location. Default: `$HOME/.velo/bin`. |

Writing to `/usr/local/bin` needs `sudo`, and the script does not add `sudo` for you. Use the manual method below if you need to elevate.

### Option B: manual download

Download the archive for your platform from the [releases page](https://github.com/BazilSuhail/Velocity/releases), or use the `latest` URLs below.

**Windows (PowerShell):**

```powershell
curl.exe -fsSL -o velo.zip https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-x86_64-pc-windows-msvc.zip
Expand-Archive -LiteralPath velo.zip -DestinationPath "$HOME\.velo\bin" -Force
Remove-Item velo.zip
```

If Windows blocks the file, right-click `velo.zip` before extracting, choose **Properties**, tick **Unblock**, then extract again.

**Linux and macOS:**

```bash
# Pick one archive:
#   velo-x86_64-unknown-linux-gnu.tar.gz    (Linux x86_64)
#   velo-aarch64-unknown-linux-gnu.tar.gz   (Linux aarch64)
#   velo-x86_64-apple-darwin.tar.gz         (macOS Intel)
#   velo-aarch64-apple-darwin.tar.gz        (macOS Apple Silicon)
curl -fsSL -o velo.tar.gz https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-aarch64-apple-darwin.tar.gz
tar -xzf velo.tar.gz
mkdir -p ~/.velo/bin && mv velo ~/.velo/bin/
rm velo.tar.gz
chmod +x ~/.velo/bin/velo
```

On macOS, a file downloaded through a browser can be blocked by Gatekeeper. Clear the quarantine flag if that happens:

```bash
xattr -d com.apple.quarantine ~/.velo/bin/velo
```

## 3. Put velo on your PATH

The installer does this for you. Use these steps if you installed manually, used `-NoPath` or `INSTALL_DIR`, or built from source.

### Windows

The installer adds `%USERPROFILE%\.velo\bin` to the user `PATH`. To add it yourself:

1. Open **Edit environment variables for your account**.
2. Select **Path** under **User variables**, then **Edit**.
3. Add `%USERPROFILE%\.velo\bin`, then click OK.
4. Open a new terminal. Existing terminals do not see the change.

Or, from PowerShell, for the current user:

```powershell
$dir = "$HOME\.velo\bin"
$user = [Environment]::GetEnvironmentVariable("PATH", "User")
if (($user -split ';') -notcontains $dir) {
    [Environment]::SetEnvironmentVariable("PATH", "$dir;$user", "User")
}
```

### Linux, macOS, and WSL

The installer appends this line to `~/.bashrc` and `~/.zshrc` (whichever exists; it creates `~/.bashrc` if neither does), once per file:

```bash
export PATH="$PATH:$HOME/.velo/bin"
```

If you installed by hand, add that line to the rc file for your shell:

| Shell | File |
|-------|------|
| bash | `~/.bashrc` (Linux), `~/.bash_profile` (macOS login shells) |
| zsh | `~/.zshrc` |
| fish | Run once: `fish_add_path ~/.velo/bin` (fish keeps its own PATH list) |

Then open a new terminal, or reload the file:

```bash
source ~/.zshrc      # or ~/.bashrc
```

### Check the PATH

```bash
velo --help
```

```powershell
velo --help
```

If you see `command not found` or `velo` is not recognized, see [Troubleshooting: command not found after install](troubleshooting.md#command-not-found-after-install).

## 4. Build from source

Use this for unsupported platforms, for testing unreleased changes, or if you do not trust prebuilt binaries.

```bash
git clone https://github.com/BazilSuhail/Velocity.git
cd Velocity
cargo build --release
```

The binary is written to:

- `target/release/velo` on Linux and macOS
- `target\release\velo.exe` on Windows

You can run it in place, or install it with Cargo. `cargo install --path .` builds in release mode and copies the binary to Cargo's bin folder (`~/.cargo/bin` by default), which rustup already puts on your `PATH`:

```bash
cargo install --path .
```

Or copy it yourself:

```bash
mkdir -p ~/.velo/bin && cp target/release/velo ~/.velo/bin/
```

```powershell
New-Item -ItemType Directory -Force "$HOME\.velo\bin" | Out-Null
Copy-Item target\release\velo.exe "$HOME\.velo\bin\"
```

If you copy the binary, make sure the folder is on your `PATH` (see section 3).

## 5. Verify the install

```bash
velo --version     # prints the version, for example: velo 0.2.0
velo --help        # prints usage, the commands, and the storage locations
```

`velo --help` also shows where your macros file is stored on each platform.

## 6. First run

This walkthrough adds a macro, lists it, runs it, opens the TUI, and quits. It works the same in every shell, because the commands are plain `velo` calls.

1. **Add a macro.** Pass the key and the command. Quote the command if it contains spaces.

   ```bash
   velo add hello "echo hello from velo"
   ```

   ```powershell
   velo add hello "echo hello from velo"
   ```

2. **List macros.** The table shows KEY, COMMAND, and UPDATED.

   ```bash
   velo list
   ```

3. **Run it.** velo prints the command, runs it, and exits with the macro's exit code.

   ```bash
   velo hello
   echo $?          # 0 on success
   ```

   ```powershell
   velo hello
   $LASTEXITCODE    # 0 on success
   ```

4. **Open the TUI.** Run `velo` with no arguments in a terminal.

   ```bash
   velo
   ```

   Use the arrow keys or `j`/`k` to select `hello`. Press `Enter` to open the actions menu, then choose **Run**. velo runs the command, prints `Press Enter to return to velo...`, and returns you to the list.

5. **Quit.** Press `q`, or `Ctrl+C`, from Browse mode.

Next steps: read [Keybindings](keybindings.md) for every TUI key, and [Commands](commands.md) for the full command reference.

## 7. Where macros are stored

All macros live in one file, `macros.json`. velo has no other config file.

| Platform | Default file |
|----------|--------------|
| Linux | `$XDG_CONFIG_HOME/velo/macros.json` if `XDG_CONFIG_HOME` is set, otherwise `~/.config/velo/macros.json` |
| macOS | `~/Library/Application Support/velo/macros.json` |
| Windows | `%APPDATA%\velo\macros.json` |
| WSL | Same as Linux, inside the distribution |

velo creates the directory on the first write. Reads of a missing file return an empty store, so `velo list` works before you add anything.

### Change the location with VELO_HOME

`VELO_HOME` is a directory, not a file. velo stores `macros.json` inside it, and it replaces the default directory on every platform.

| Shell | Set for the current session | Make it persistent |
|-------|-----------------------------|--------------------|
| bash | `export VELO_HOME="$HOME/dotfiles/velo"` | Add the `export` line to `~/.bashrc` |
| zsh | `export VELO_HOME="$HOME/dotfiles/velo"` | Add the `export` line to `~/.zshrc` |
| fish | `set -gx VELO_HOME ~/dotfiles/velo` | `set -Ux VELO_HOME ~/dotfiles/velo` |
| PowerShell | `$env:VELO_HOME = "D:\sync\velo"` | `[Environment]::SetEnvironmentVariable("VELO_HOME", "D:\sync\velo", "User")` |
| cmd | `set VELO_HOME=D:\sync\velo` | `setx VELO_HOME "D:\sync\velo"` (new terminals only) |

Check which file velo uses:

```bash
echo "$VELO_HOME"
velo list
```

```powershell
$env:VELO_HOME
velo list
```

Notes:

- Setting `VELO_HOME` does not move an existing `macros.json`. Copy the file yourself if you want the same macros in the new place.
- An empty `VELO_HOME` is ignored, and the default location is used.
- Use an isolated folder to test without touching your real macros:

  ```bash
  VELO_HOME="$(mktemp -d)" velo list
  ```

  ```powershell
  $env:VELO_HOME = Join-Path $env:TEMP ("velo-" + [guid]::NewGuid()); velo list
  ```

## 8. Environment variables

| Variable | Read by | Effect |
|----------|---------|--------|
| `VELO_HOME` | velo | Directory that holds `macros.json`. Overrides the default location. |
| `VELO_ASCII` | velo | Any value forces ASCII glyphs in the TUI and the output. |
| `NO_COLOR` | velo | Any value disables color output. |
| `CLICOLOR_FORCE` | velo | Set to `1` to keep color when output is piped. |
| `TERM` | velo | `dumb` disables color. |
| `COLORTERM` | velo | `truecolor` (or `24bit`) selects the 24-bit palette. |
| `WT_SESSION` | velo | Set by Windows Terminal. velo uses it to enable truecolor, and to allow Unicode glyphs on Windows. You do not set it yourself. |
| `XDG_CONFIG_HOME` | velo (Linux) | Changes the default base directory to `$XDG_CONFIG_HOME/velo`. |
| `INSTALL_DIR` | `scripts/install.sh` | Install location for the Linux and macOS installer. |

Examples:

```bash
NO_COLOR=1 velo list                    # plain text
CLICOLOR_FORCE=1 velo list | less -R    # keep color through a pipe
VELO_ASCII=1 velo                       # ASCII-only TUI
COLORTERM=truecolor velo                # force the 24-bit palette
```

```powershell
$env:VELO_ASCII = "1"; velo
```

Color is off when output is not a terminal, unless `CLICOLOR_FORCE=1` is set. See [Configuration](configuration.md#environment-variables) for the full rules.

## 9. Back up and move your macros

The whole store is one file. Back it up by copying it.

```bash
cp ~/.config/velo/macros.json ~/backups/macros-$(date +%F).json          # Linux
cp "~/Library/Application Support/velo/macros.json" ~/backups/macros-$(date +%F).json   # macOS
```

```powershell
Copy-Item "$env:APPDATA\velo\macros.json" "$HOME\Backups\macros-$(Get-Date -Format yyyy-MM-dd).json"
```

To move macros to a new machine or folder:

1. Stop using velo (close the TUI).
2. Copy `macros.json` to the new location.
3. Set `VELO_HOME` to that folder, or copy the file into the default location for that machine.
4. Run `velo list` to confirm the macros are there.

Avoid editing the file on two machines at the same time. For details, see [Configuration: backup](configuration.md#backup).

## 10. Update velo

Updating does not change your macros. Back up `macros.json` first if you want an easy rollback (section 9).

**Installer:** run the install command again. It replaces the binary and does not add a second PATH entry.

```bash
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash
```

```powershell
irm https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.ps1 | iex
```

**Manual download:** download the new archive and replace the binary in your install folder.

**From source:** pull the latest code and rebuild.

```bash
cd Velocity
git pull
cargo install --path .
```

Store format: current versions use the schema described in [Configuration](configuration.md#macro-file-format). Files in the older flat format (versions before 0.2.0) are converted automatically on load, and rewritten in the current format on the next save. Keep a copy of the old file before the first run of a new version if you want to keep the original bytes. See [Configuration: legacy format migration](configuration.md#legacy-format-migration).

## 11. Uninstall velo

Removing velo does not remove your macros unless you delete the config folder. Back up `macros.json` first (section 9).

### Linux, macOS, and WSL

1. Remove the binary:

   ```bash
   rm ~/.velo/bin/velo
   rmdir ~/.velo/bin ~/.velo 2>/dev/null
   ```

2. Remove the PATH lines the installer added. Open `~/.bashrc` and `~/.zshrc`, and delete the `# Added by velo installer` comment and the `export PATH="$PATH:$HOME/.velo/bin"` line below it.

3. Optional: delete your macros. This removes the store permanently.

   ```bash
   rm -r ~/.config/velo          # Linux and WSL
   rm -r ~/Library/Application\ Support/velo    # macOS
   ```

If you installed with `cargo install`, run `cargo uninstall velo` instead of step 1.

### Windows

1. Remove the binary:

   ```powershell
   Remove-Item "$HOME\.velo\bin\velo.exe"
   Remove-Item "$HOME\.velo\bin" -ErrorAction SilentlyContinue
   ```

2. Remove the folder from the user `PATH`:

   ```powershell
   $dir = "$HOME\.velo\bin"
   $user = [Environment]::GetEnvironmentVariable("PATH", "User")
   $kept = ($user -split ';' | Where-Object { $_ -and $_ -ne $dir }) -join ';'
   [Environment]::SetEnvironmentVariable("PATH", $kept, "User")
   ```

3. Optional: delete your macros. This removes the store permanently.

   ```powershell
   Remove-Item -Recurse "$env:APPDATA\velo"
   ```

4. If you set `VELO_HOME`, remove it too. For a persistent setting:

   ```powershell
   [Environment]::SetEnvironmentVariable("VELO_HOME", $null, "User")
   ```

Open a new terminal after any change to `PATH` or environment variables.

## 12. Troubleshooting

Start with [Troubleshooting](troubleshooting.md). The most common setup problems have their own sections:

| Problem | Section |
|---------|---------|
| `velo: command not found` or `velo` is not recognized | [Command not found after install](troubleshooting.md#command-not-found-after-install) |
| `Permission denied` when running or saving | [Permission denied](troubleshooting.md#permission-denied) |
| `macros.json` cannot be parsed | [Corrupt macros.json](troubleshooting.md#corrupt-macrosjson) |
| Boxes or symbols show as `?` or squares | [Glyphs are broken](troubleshooting.md#glyphs-are-broken) |
| Colors are missing | [Colors do not show](troubleshooting.md#colors-do-not-show) |
| Terminal looks broken after a macro | [Terminal left in a bad state](troubleshooting.md#terminal-left-in-a-bad-state) |
| TUI says the terminal is too small | [Terminal too small](troubleshooting.md#the-tui-says-the-terminal-is-too-small) |
| A macro fails or does nothing | [A macro does nothing or fails](troubleshooting.md#a-macro-does-nothing-or-fails) |

If none of these match, open an issue with the information listed in [Troubleshooting: still stuck](troubleshooting.md#still-stuck).

## Related

- [README](../README.md): overview and quick start
- [Commands](commands.md): every command, flag, and exit code
- [Keybindings](keybindings.md): the TUI keys
- [Configuration](configuration.md): storage, `VELO_HOME`, and the file format
- [Platforms](platforms.md): per-OS notes, including quoting for macros
- [Troubleshooting](troubleshooting.md): symptoms and fixes
- [Contributing](../CONTRIBUTING.md): building, checks, and pull requests
