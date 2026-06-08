# Platform Guide

velo runs on Windows, Linux, macOS, and WSL. This page covers install steps, terminal choices, and known limits for each.

## Summary

| Platform | Shell used for macros | Config location | Notes |
|----------|-----------------------|-----------------|-------|
| Windows 10/11 | `cmd /C` | `%APPDATA%\velo\macros.json` | Works in PowerShell, cmd, and Windows Terminal |
| Linux | `sh -c` | `~/.config/velo/macros.json` | x86_64 and aarch64 builds |
| macOS | `sh -c` | `~/Library/Application Support/velo/macros.json` | Apple Silicon and Intel builds |
| WSL | `sh -c` (inside the distro) | `~/.config/velo/macros.json` inside the distro | Install inside WSL, not in Windows |

## Windows

### Install

PowerShell:

```powershell
irm https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.ps1 | iex
```

The script installs `velo.exe` to `%USERPROFILE%\.velo\bin` and adds that folder to your user `PATH`.

Options (run the script as a block to pass parameters):

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.ps1))) -NoPath
```

| Parameter | Effect |
|-----------|--------|
| `-Version <x.y.z>` | Install a specific release instead of the latest |
| `-InstallDir <path>` | Install to a different folder |
| `-NoPath` | Do not change the user `PATH` |

Manual install (Windows 10 and 11 include `tar.exe`, which reads zip files):

```powershell
curl.exe -fsSL -o velo.zip https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-x86_64-pc-windows-msvc.zip
Expand-Archive -LiteralPath velo.zip -DestinationPath "$HOME\.velo\bin" -Force
Remove-Item velo.zip
& "$HOME\.velo\bin\velo.exe" --help
```

Add `$HOME\.velo\bin` to your `PATH` yourself if you use the manual route.

### Which shell to use

velo works in any of these:

| Environment | Notes |
|-------------|-------|
| Windows Terminal | Recommended. Full color, Unicode glyphs, and truecolor. |
| PowerShell (in Windows Terminal) | Same as above. Set the font to one with box-drawing glyphs, such as Cascadia Mono. |
| cmd (in Windows Terminal) | Same as above. |
| PowerShell or cmd in ConHost (the classic console window) | Works. ConHost has limited truecolor support, so velo may show fewer colors. See the note below. |

Open Windows Terminal from the Start menu, or set it as your default terminal under **Settings > System > For developers**.

### Quoting

Each macro runs as `cmd /C "<command>"`. The command follows cmd's rules, not PowerShell's.

| Want | Write in the command |
|------|----------------------|
| Run a PowerShell script | `powershell -NoProfile -File .\build.ps1` |
| Use an environment variable | `%USERPROFILE%` in cmd. Not `$env:USERPROFILE`. |
| Pipe | `dir \| findstr .rs` (quote the whole velo argument so PowerShell does not pipe it) |
| Path with spaces | `"C:\Program Files\tool\tool.exe" --flag` |

Store a PowerShell command by calling PowerShell explicitly:

```powershell
velo add ps-ls "powershell -NoProfile -Command Get-ChildItem"
velo add logs "powershell -NoProfile -Command Get-Content app.log -Tail 50"
```

PowerShell expands `$` inside double quotes. Use single quotes around the whole velo argument when the command contains `$`:

```powershell
velo add env-user 'echo %USERNAME%'
```

### Known limits (Windows)

- **ConHost colors.** The classic console does not reliably render 24-bit colors. velo uses the basic palette there. Use Windows Terminal for the full look.
- **Glyphs.** If the banner or borders show as squares or question marks, set `VELO_ASCII=1` for the session, or change the console font.
- **Resize.** ConHost redraws more slowly on resize. Let the window settle before you press keys.

## macOS

### Install

```bash
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash
```

The script detects Apple Silicon or Intel and installs to `~/.velo/bin`. It adds that folder to `PATH` in `~/.bashrc` and `~/.zshrc`, whichever exists. If neither exists, it creates `~/.bashrc`. Re-running the script does not add the line twice.

Manual install:

```bash
# Apple Silicon
curl -fsSL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-aarch64-apple-darwin.tar.gz | tar xz
# Intel
curl -fsSL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-x86_64-apple-darwin.tar.gz | tar xz

mkdir -p ~/.velo/bin && mv velo ~/.velo/bin/
echo 'export PATH="$PATH:$HOME/.velo/bin"' >> ~/.zshrc
velo --help
```

### Gatekeeper

A binary downloaded with `curl` is not quarantined, so it runs without a prompt. If you downloaded the archive with a browser, macOS may block it. Clear the flag:

```bash
xattr -d com.apple.quarantine ~/.velo/bin/velo
```

### Terminal recommendations

| Terminal | Status |
|----------|--------|
| Terminal.app | Works. Enable **Use Unicode** in the profile if glyphs look wrong. |
| iTerm2 | Recommended. Full color and truecolor. |

Choose a font with box-drawing and braille glyphs, such as a Nerd Font or Menlo.

## Linux

### Install

```bash
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash
```

The script detects `x86_64` and `aarch64`, installs to `~/.velo/bin`, and adds that folder to `PATH` in `~/.bashrc` and `~/.zshrc`, whichever exists. It does not add the line twice.

Manual install:

```bash
# x86_64
curl -fsSL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-x86_64-unknown-linux-gnu.tar.gz | tar xz
# aarch64 (ARM64)
curl -fsSL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-aarch64-unknown-linux-gnu.tar.gz | tar xz

mkdir -p ~/.velo/bin && mv velo ~/.velo/bin/
```

Pin a version with the script:

```bash
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash -s
```

Install to another location with `INSTALL_DIR`:

```bash
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | INSTALL_DIR=/usr/local/bin bash
```

Writing to `/usr/local/bin` needs `sudo`. Run the `mv` step with `sudo` if you use this path.

### Terminal recommendations

Any terminal with UTF-8 and 256 colors works, including GNOME Terminal, Konsole, Alacritty, kitty, and WezTerm. Use a font with box-drawing glyphs.

## WSL

Install velo inside the WSL distribution, not in Windows. macros run inside the distro, and the config lives in the distro's home directory.

```bash
curl -fsSL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash
```

Use Windows Terminal with a WSL profile for the best rendering.

Keep one install per environment. The Windows and WSL builds run different shells, so a macro that works in one may fail in the other.

### Known limits (WSL)

- Macros run in the Linux shell (`sh -c`). Commands written for `cmd` or PowerShell do not work here.
- Files under `/mnt/c` are slower to access. Keep the macro store in the Linux home unless you need it on Windows too.

## Known limits (all platforms)

| Limit | Workaround |
|-------|-----------|
| Glyphs render as squares or `?` | `export VELO_ASCII=1` (or `$env:VELO_ASCII = "1"` on Windows) |
| Colors are wrong or missing | Set `COLORTERM=truecolor` in terminals that support it, or `NO_COLOR=1` to turn color off |
| Terminal looks broken after a macro | See [Troubleshooting](troubleshooting.md#terminal-left-in-a-bad-state) |
| Interactive prompts in scripts | Pass all arguments and use `--yes` for `delete`. See [Commands](commands.md). |

Related: [Configuration](configuration.md), [Troubleshooting](troubleshooting.md).
