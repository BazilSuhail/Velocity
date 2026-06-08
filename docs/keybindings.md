# Keybindings and TUI Reference

Run `velo` with no arguments in a terminal to open the TUI. The TUI has several modes. The footer always shows the keys that work in the current mode.

For the command-line equivalents, see [Commands](commands.md).

## Browse mode

This is the default mode.

| Key | Action |
|-----|--------|
| `Up` or `k` | Move the selection up one row |
| `Down` or `j` | Move the selection down one row |
| `PageUp` | Move up 10 rows |
| `PageDown` | Move down 10 rows |
| `Home` or `g` | Jump to the first macro |
| `End` or `G` | Jump to the last macro |
| `Enter` | Open the actions menu for the selected macro |
| `n` | New macro: enter a key, then a command |
| `e` | Edit the command of the selected macro |
| `d` | Delete the selected macro (asks for confirmation) |
| `/` | Start a filter |
| `?` | Show the help overlay |
| `q` or `Ctrl+C` | Quit |

`e` and `d` skip the menu. They do the same thing as the **Update** and **Delete** entries.

## Actions menu

Press `Enter` in Browse mode to open the menu for the selected macro. It lists four entries:

| Entry | What it does |
|-------|--------------|
| **Run** | Runs the macro (see below) |
| **Update** | Opens the edit form for the command |
| **Delete** | Asks for confirmation before removing the macro |
| **Back** | Returns to the list |

| Key | Action |
|-----|--------|
| `Up` or `Down` (or `k` / `j`) | Move the highlight |
| `Enter` | Choose the highlighted entry |
| `Esc` or `q` | Back to the list |

### Running a macro

Choose **Run** in the actions menu:

1. The TUI suspends and returns the terminal to normal mode.
2. The command runs, and its output prints to the terminal.
3. velo prints `Press Enter to return to velo...`.
4. Press `Enter`. The TUI restores and redraws, and the status line shows the exit code.

Use this to check a result before returning to the list.

## Input mode

Input mode opens for `n` (new macro: key, then command) and `e` (edit command). A prompt shows the field you are filling in.

| Key | Action |
|-----|--------|
| Type | Add text to the field |
| `Backspace` | Delete the last character |
| `Ctrl+U` | Clear the field |
| `Enter` | Submit the field. For `n`, the flow moves from key to command, and the last step saves. |
| `Esc` | Cancel. Nothing is saved. |

The field has no cursor, so text is always added and removed at the end. Keys are validated when you submit them. See the key rules in [Commands](commands.md#concepts).

## Delete confirmation

Pressing `d` in Browse mode, or choosing **Delete** in the actions menu, asks for confirmation.

| Key | Action |
|-----|--------|
| `y` or `Enter` | Delete the macro |
| `n` or `Esc` | Cancel. Nothing is deleted. |

## Filter mode

Pressing `/` opens a filter. The list narrows as you type. The filter matches both keys and commands.

| Key | Action |
|-----|--------|
| Type | Narrow the list by the text you enter |
| `Backspace` | Delete the last character of the filter |
| `Ctrl+U` | Clear the filter text |
| `Enter` | Keep the filter applied and return to Browse mode |
| `Esc` | Clear the filter and return to the full list |

## Help overlay

Press `?` to open the help overlay. It lists the keys for Browse mode and the command-line forms. Press `?` or `Esc` to close it.

## Saving and safety

Every change is re-read from disk before it is applied and saved. A change made in another terminal while velo is open is kept, not overwritten. If the file cannot be loaded, velo shows the error and refuses writes until you fix or remove the file and restart it.

## Tips

- **Find a macro fast.** Press `/`, type two or three letters of the key, then press `Enter`. Use the arrow keys to pick a row, then `Enter` to open its menu.
- **Jump around a long list.** Use `PageUp` and `PageDown` for large jumps. Use `g` and `G` to reach either end.
- **Keep the filter.** `Enter` in filter mode keeps the narrowed list, so you can act on several macros from it. Press `/` and then `Esc` to clear the filter.
- **Fix a typo in a command.** Select the macro and press `e`. Press `Ctrl+U` to clear the field and retype the command.
- **Use the keyboard only.** velo does not respond to the mouse.

## Responsive layout

The TUI needs at least 30 columns by 8 rows. Smaller windows show a resize message instead of the list.

| Terminal size | What appears |
|---------------|--------------|
| 60 columns or more, and 20 rows or more | The full VELO banner and subtitle |
| Smaller than that | A one-line title |
| 60 columns or more | The UPDATED column in the table |
| 100 columns or more | The details pane beside the list |

Resize the window to see the layout change. Glyphs can be forced to ASCII with `VELO_ASCII`. See [Configuration](configuration.md#environment-variables).
