# Troubleshooting

Find your symptom below. If nothing matches, open an issue at <https://github.com/BazilSuhail/Velocity/issues> with your OS, terminal, `velo --version`, and the exact error.

## Corrupt `macros.json`

**Symptom:** velo reports that `macros.json` cannot be parsed. Adding, updating, or deleting a macro fails.

velo refuses to write to a corrupt file. It reports the path and leaves the file unchanged. Your data is still there. Recover it in this order:

1. **Find the file.** The path is in the error message. The defaults are listed in [Configuration](configuration.md#storage-location).
2. **Make a copy first.** Do not edit the original until you have a copy.

   ```bash
   cp ~/.config/velo/macros.json ~/macros-corrupt.json
   ```

   ```powershell
   Copy-Item "$env:APPDATA\velo\macros.json" "$HOME\macros-corrupt.json"
   ```

3. **Check for a backup.** Restore the newest good copy if you have one.
4. **Repair by hand.** Open the copy in an editor. Common causes are a trailing comma, a missing closing brace, or a half-written file after a crash. Validate it:

   ```bash
   python3 -m json.tool ~/macros-corrupt.json > /dev/null && echo "valid"
   ```

5. **Start over if needed.** Replace the broken file with a valid empty store, then re-add your macros:

   ```json
   {
     "macros": {}
   }
   ```

Keep the copy until you have checked that the repaired store works with `velo list`.

## Terminal left in a bad state

**Symptom:** after a macro or after velo exits, the cursor is hidden, the text does not echo, or the colors are wrong.

velo normally restores the terminal on exit. A crash, a killed process, or a macro that changes terminal modes can leave it in a bad state.

| Platform | Fix |
|----------|-----|
| Linux, macOS, WSL | Run `reset`. Type it blind if the prompt is invisible, then press Enter. |
| Windows Terminal, ConHost | Run `cls`. If that does not help, close the window and open a new one. |

## Non-interactive use

**Symptom:** `velo` prints a plain list instead of the TUI, or a command hangs waiting for input.

velo uses the TUI only when it runs in a terminal. In pipes, redirects, cron, and CI, it prints output and never prompts.

- `velo` with output piped (`velo | cat`) prints the list.
- `velo delete <KEY>` without `--yes` refuses and exits with `2`. Add `--yes`.
- `velo add` and `velo update` with missing arguments prompt for input. Pass both arguments in scripts.

```bash
velo delete old-key --yes
velo add lint "cargo clippy --all-targets -- -D warnings"
```

Check the exit code in scripts. See the exit code table in [Commands](commands.md#exit-codes).

## Colors do not show

**Symptom:** the output is plain, or the TUI is not colored.

Color is off in these cases:

- `NO_COLOR` is set. Check with `echo $NO_COLOR` (Linux, macOS) or `$env:NO_COLOR` (Windows).
- `TERM=dumb`. Check with `echo $TERM`.
- The output is not a terminal, such as a pipe or a redirect.

To keep color when piping:

```bash
CLICOLOR_FORCE=1 velo list | less -R
```

To unset `NO_COLOR` for one run:

```bash
env -u NO_COLOR velo
```

```powershell
Remove-Item Env:NO_COLOR -ErrorAction SilentlyContinue; velo
```

If colors look washed out or wrong in a terminal that supports 24-bit color, set `COLORTERM=truecolor`. ConHost has limited truecolor support.

## Glyphs are broken

**Symptom:** the banner, borders, or arrows show as `?`, squares, or random characters.

The terminal font does not have the glyphs, or the locale is not UTF-8. Two fixes:

1. Force ASCII for one session:

   ```bash
   VELO_ASCII=1 velo
   ```

   ```powershell
   $env:VELO_ASCII = "1"; velo
   ```

2. Switch to a font with box-drawing glyphs, such as a Nerd Font, Cascadia Mono, or Menlo. On Linux, check that the locale ends in `UTF-8`:

   ```bash
   locale | grep -i utf
   ```

## Command not found after install

**Symptom:** `velo: command not found` (or `velo` is not recognized in PowerShell or cmd) right after install.

The install folder is not on your `PATH` in this shell.

1. Open a new terminal. The installer changed the rc file or the user `PATH`, but existing shells do not see the change.
2. Check where the binary is:

   ```bash
   ls ~/.velo/bin/velo          # Linux, macOS, WSL
   ```

   ```powershell
   Test-Path "$HOME\.velo\bin\velo.exe"
   ```

3. If the file exists, add the folder for this session:

   ```bash
   export PATH="$PATH:$HOME/.velo/bin"
   ```

   ```powershell
   $env:PATH = "$HOME\.velo\bin;$env:PATH"
   ```

4. Make it permanent. On Linux and macOS, add the `export` line to `~/.bashrc` or `~/.zshrc`, depending on your shell. On Windows, add the folder under **Edit environment variables for your account**, then open a new terminal. Use `-NoPath` with the installer if you prefer to manage `PATH` yourself.

5. Check which shell you use. `echo $SHELL` shows your login shell. Your rc file must match it.

## Permission denied

**Symptom:** `Permission denied` when you run the binary, or when velo saves or reads the store.

- **Binary will not run.** Make it executable:

  ```bash
  chmod +x ~/.velo/bin/velo
  ```

- **Install folder not writable.** Install to a folder you own, such as `~/.velo/bin`. Avoid `/usr/local/bin` unless you use `sudo`.
- **Store not writable.** velo writes a temporary file in the store's directory, then renames it. Check that you can write to the directory, not only the file:

  ```bash
  ls -ld ~/.config/velo
  ```

- **macOS blocks the binary.** See the Gatekeeper note in [Platforms](platforms.md#gatekeeper).
- **Windows blocks the file.** Right-click the zip, choose **Properties**, then **Unblock**, and extract again.

## A macro does nothing or fails

**Symptom:** `velo <KEY>` exits with a non-zero code.

velo exits with the macro's own code. The problem is usually in the command.

- Run the command directly in your shell first. Copy it from `velo list`.
- Check quoting. On Windows, macros run through `cmd /C`, so PowerShell-only syntax fails. See [Platforms](platforms.md#quoting).
- Check that the program is on your `PATH` in the environment where velo runs.

If you see `no macro named '...'`, the key is misspelled or belongs to another `VELO_HOME`. Run `velo list` and check `echo $VELO_HOME`.

## The TUI says the terminal is too small

**Symptom:** the TUI shows a resize message.

The TUI needs at least 30 columns and 8 rows. Make the window larger. See [Keybindings](keybindings.md#responsive-layout).

## Still stuck

Collect this information and open an issue:

```bash
velo --version
echo "$SHELL $TERM $COLORTERM $NO_COLOR $VELO_HOME"
uname -a                     # Linux and macOS
```

Include the exact command, the output, and the exit code (`echo $?`).
