# Velocity

[![CI](https://img.shields.io/github/actions/workflow/status/BazilSuhail/Velocity/release.yml?style=flat-square)](https://github.com/BazilSuhail/Velocity/actions)
[![Version](https://img.shields.io/github/v/release/BazilSuhail/Velocity?style=flat-square)](https://github.com/BazilSuhail/Velocity/releases)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange?style=flat-square)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-windows%20%7C%20linux%20%7C%20macos-lightgrey?style=flat-square)](https://github.com/BazilSuhail/Velocity/releases)

**Velocity** is a cross-platform command-macro engine that lets you store, browse, and execute frequently used terminal commands under short, memorable names. Think of it as a personal hotkey registry for your shell.

---

## Features

- **Interactive TUI** -- Cursor-navigable interface (arrow keys) to browse, run, update, or delete macros without leaving the terminal
- **Direct execution** -- `velo dev` instantly runs the macro named `dev`
- **Persistent storage** -- Macros are saved as JSON and survive reboots
- **Timestamps** -- Each macro tracks creation and last-updated time
- **Cross-platform** -- Works on Windows, Linux, and macOS
- **Hot gradient ASCII** -- Branded neon-red-to-orange header on every screen
- **Single binary** -- No runtime dependencies; just download and run

---

## Installation

### Windows

**Option 1 -- PowerShell one-liner (recommended)**
```powershell
iex "& { $(iwr -useb https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.ps1) }"
```
Adds `velo.exe` to your PATH automatically.

**Option 2 -- curl (Windows 10/11)**
```cmd
curl -sL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-x86_64-pc-windows-msvc.zip -o %TEMP%\velo.zip
tar -xf %TEMP%\velo.zip -C %USERPROFILE%\.velo\bin
setx PATH "%PATH%;%USERPROFILE%\.velo\bin"
velo --help
```

### Linux / macOS
```bash
curl -sL https://raw.githubusercontent.com/BazilSuhail/Velocity/main/scripts/install.sh | bash
```

Or manually:
```bash
# Linux (x86_64)
curl -sL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-x86_64-unknown-linux-gnu.tar.gz | tar xz
./velo --help

# macOS (Apple Silicon)
curl -sL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-aarch64-apple-darwin.tar.gz | tar xz
./velo --help

# macOS (Intel)
curl -sL https://github.com/BazilSuhail/Velocity/releases/latest/download/velo-x86_64-apple-darwin.tar.gz | tar xz
./velo --help
```

---

## Usage

### Commands

| Command | Description |
|---------|-------------|
| `velo` | Open the interactive TUI (cursor navigation) |
| `velo list` | List all stored macros |
| `velo add <key> <command>` | Store a new macro |
| `velo delete <key>` | Remove a macro |
| `velo update <key> <command>` | Change a macro's command |
| `velo <key>` | Run a macro directly (shorthand) |
| `velo --help` | Show colored help with all options |

### Examples

```
velo add dev "npm run dev"
velo add build "cargo build --release"
velo dev
velo list
velo delete dev
velo update build "bun run build"
```

### Interactive TUI

Running `velo` with no arguments opens the interactive menu:

- `Up/Down` -- Navigate through macros and Exit
- `Enter` -- Select a macro to view actions (Run / Update / Delete / Back)
- `Esc` -- Go back to main menu
- `q` -- Quit

The first time you enter, **Exit** is selected by default.

---

## Why Rust?

| Concern | Rust's answer |
|---------|---------------|
| **Startup time** | Near-zero; TUI renders instantly |
| **Binary size** | Stripped binary is ~3 MB with no runtime |
| **Cross-compilation** | `cargo build --target` for all three OSes from a single CI pipeline |
| **Memory safety** | No segfaults, no use-after-free, no data races |
| **Dependency management** | Cargo's semantic versioning and lockfile guarantee reproducible builds |
| **Terminal control** | `crossterm` provides a zero-cost abstraction over raw TTY on every platform |
| **Single binary** | Statically linked; no Python, Node, or JVM required |

Rust was chosen over alternatives (Go, Python, Node.js) because Velocity demands **sub-millisecond startup**, a **single distributable binary**, and **safe systems-level terminal I/O** -- all of which are Rust's sweet spot.

---

## Building from Source

```bash
git clone https://github.com/BazilSuhail/Velocity.git
cd Velocity
cargo build --release
./target/release/velo --help
```

Requires Rust 1.85 or later.

---

## Author

**Bazil Suhail** -- [bazilsuhail.netlify.app](https://bazilsuhail.netlify.app)

---

## License

MIT
