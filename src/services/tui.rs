use colored::*;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdin, stdout, Stdout, Write};

use crate::utils::display::{self, fmt_dt, truncate};
use crate::utils::storage::{load_macros, save_macros, timestamp_now};

/// Erase the screen + scrollback and move the cursor home in ONE sequence.
/// Using a single escape string (instead of separate `\x1b[3J`/`\x1b[2J`
/// writes) prevents the visible blank flash between commands.
const FULL_CLEAR: &str = "\x1b[2J\x1b[3J\x1b[H";

enum MenuIdx {
    Macro(usize),
    Exit,
}

/// A frame being built in memory. `lines` tracks how many vertical rows the
/// frame occupies so the next `render` can back up and wipe just those rows.
struct Canvas {
    s: String,
    lines: usize,
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            s: String::new(),
            lines: 0,
        }
    }

    fn push(&mut self, line: &str) {
        self.s.push_str(line);
        self.s.push('\n');
        self.lines += 1;
    }

    fn blank(&mut self) {
        self.s.push('\n');
        self.lines += 1;
    }
}

/// Repaint a frame as a single batched write to avoid inter-command flicker:
///  - first frame / after an action: emit one `FULL_CLEAR`, then the frame.
///  - scrolling in place: back up `prev_lines`, wipe from there down, then
///    write the new frame — everything in one output buffer + one flush.
fn render(stdout: &mut Stdout, prev_lines: usize, canvas: &Canvas) -> usize {
    let mut out = String::new();
    if prev_lines > 0 {
        out.push_str(&format!("\x1b[{}A\x1b[J", prev_lines));
    } else {
        out.push_str(FULL_CLEAR);
    }
    out.push_str(&canvas.s);
    let _ = stdout.write_all(out.as_bytes());
    let _ = stdout.flush();
    canvas.lines
}

/// Read one raw key press. `crossterm` normalises every terminal's encoding to
/// `KeyCode` for us — legacy cmd codes (`\xe0H`/`\xe0P`), ANSI/VT sequences
/// (`ESC [ A`) and Unix alike — so arrows always work. W/S are a text fallback.
fn read_key() -> Option<KeyCode> {
    if event::poll(std::time::Duration::from_millis(200)).ok()?
        && let Event::Key(k) = event::read().ok()?
        && k.kind == KeyEventKind::Press
    {
        return Some(normalize(k.code));
    }
    None
}

fn normalize(code: KeyCode) -> KeyCode {
    match code {
        KeyCode::Char(c) if c == 'w' || c == 'W' => KeyCode::Up,
        KeyCode::Char(c) if c == 's' || c == 'S' => KeyCode::Down,
        other => other,
    }
}

fn pause_before_continue() {
    print!(
        "  {}  {}",
        "◆".bright_cyan(),
        "Press Enter to continue...".bright_white()
    );
    stdout().flush().ok();
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
}

fn run_command(command: &str) {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    println!("{}", command.bright_yellow());
    println!();

    let status = std::process::Command::new(shell)
        .arg(flag)
        .arg(command)
        .spawn()
        .expect("Failed to spawn command")
        .wait()
        .expect("Command failed");

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        eprintln!();
        eprintln!(
            "  {}  {}",
            "✖".bright_red(),
            format!("Exited with code {}", code).bright_white()
        );
    }
}

fn input_line(prompt_msg: &str) -> String {
    print!("{}", prompt_msg);
    stdout().flush().ok();
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn draw_banner(canvas: &mut Canvas, count: usize) {
    canvas.push(&"      ██╗   ██╗███████╗██╗     ██████╗ ".truecolor(255, 50, 50).bold().to_string());
    canvas.push(&"      ██║   ██║██╔════╝██║    ██╔═══██╗".truecolor(255, 90, 30).bold().to_string());
    canvas.push(&"      ██║   ██║█████╗  ██║    ██║   ██║".truecolor(255, 130, 10).bold().to_string());
    canvas.push(&"      ╚██╗ ██╔╝██╔══╝  ██║    ██║   ██║".truecolor(255, 160, 0).bold().to_string());
    canvas.push(&"       ╚████╔╝ ███████╗███████╗╚██████╔╝".truecolor(255, 190, 0).bold().to_string());
    canvas.push(&"        ╚═══╝  ╚══════╝╚══════╝ ╚═════╝ ".truecolor(255, 210, 0).bold().to_string());
    canvas.blank();
    canvas.push(&format!(
        "  {} {} {}",
        "by".truecolor(120, 120, 120),
        "Bazil Suhail".truecolor(255, 180, 80).bold(),
        "»  bazilsuhail.netlify.app".truecolor(255, 140, 40)
    ));
    canvas.push(&format!(
        "  {} {} {} {}",
        "Universal Command Macro Engine".truecolor(255, 60, 60).bold(),
        "•".truecolor(100, 100, 100),
        format!("{} macros loaded", count).truecolor(255, 150, 0),
        "•".truecolor(100, 100, 100),
    ));
    canvas.push(&format!(
        "  {} {} {} {}",
        "Version".truecolor(120, 120, 120),
        format!("v{}", env!("CARGO_PKG_VERSION")).truecolor(255, 150, 0),
        "»".truecolor(100, 100, 100),
        "⌨  ↑/w ↓/s navigate  ↵ select  q quit".truecolor(100, 100, 100)
    ));
    canvas.push(&"  ════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70).to_string());
    canvas.blank();
}

/// Active row: bold white on blue with a `>` marker. Inactive rows are dimmed.
fn main_row(selected: bool, prefix: usize, key: &str, cmd: &str, date: &str, time: &str) -> String {
    let pad = " ".repeat(prefix);
    if selected {
        let row = format!(
            "  {}>  {:<18} {:<38} {:<8} {}",
            pad,
            key,
            cmd,
            date,
            time
        );
        row.bright_white().bold().on_blue().to_string()
    } else {
        let row = format!(
            "  {}   {:<18} {:<38} {:<8} {}",
            pad,
            key,
            cmd,
            date,
            time
        );
        row.dimmed().to_string()
    }
}

fn draw_main_menu(canvas: &mut Canvas, store: &crate::utils::storage::MacroStore, keys: &[String], selected: &MenuIdx) {
    let count = keys.len();
    draw_banner(canvas, count);

    if keys.is_empty() {
        canvas.push(&format!(
            "  {}  {}",
            "▲".bright_yellow(),
            "No macros stored. Use `velo add` to create one.".bright_white()
        ));
        canvas.blank();
    } else {
        canvas.push(&format!(
            "  {:<20} {:<38} {:<8} {}",
            "KEY".truecolor(255, 130, 0).bold(),
            "COMMAND".truecolor(255, 130, 0).bold(),
            "DATE".truecolor(255, 130, 0).bold(),
            "TIME".truecolor(255, 130, 0).bold()
        ));
        canvas.push(&"  ════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70).to_string());

        for (i, key) in keys.iter().enumerate() {
            let entry = &store.macros[key];
            let (date, time) = fmt_dt(entry);
            let is_sel = matches!(selected, MenuIdx::Macro(j) if *j == i);
            let indent = if is_sel { 0 } else { 1 };
            canvas.push(&main_row(is_sel, indent, key, &truncate(&entry.command, 38), &date, &time));
        }
    }

    canvas.push(&"  ════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70).to_string());

    let exit_sel = matches!(selected, MenuIdx::Exit);
    canvas.push(&if exit_sel {
        let row = format!("  >  {}", "Exit");
        row.bright_white().bold().on_blue().to_string()
    } else {
        let row = format!("  {}   {}", " ", "Exit");
        row.dimmed().to_string()
    });
    canvas.blank();
}

fn draw_actions_menu(canvas: &mut Canvas, store: &crate::utils::storage::MacroStore, key: &str, selected: usize) {
    let entry = &store.macros[key];
    let actions = ["Run", "Update", "Delete", "Back"];

    draw_banner(canvas, store.macros.len());

    let (date, time) = fmt_dt(entry);
    canvas.push(&format!(
        "  {}  {}  {}  {}  {}  {}",
        ">".truecolor(255, 110, 0).bold(),
        key.truecolor(255, 130, 0).bold(),
        "→".truecolor(100, 100, 100),
        truncate(&entry.command, 34).truecolor(255, 190, 0),
        date.truecolor(255, 190, 0),
        time.truecolor(255, 190, 0),
    ));
    canvas.blank();
    canvas.push(&format!("  {}", "Actions:".truecolor(255, 210, 0).bold()));
    canvas.push(&"  ══════════════════════════════════════".truecolor(70, 70, 70).to_string());

    for (i, action) in actions.iter().enumerate() {
        let is_sel = i == selected;
        if is_sel {
            let row = format!("  >  {}", action);
            canvas.push(&row.bright_white().bold().on_blue().to_string());
        } else {
            let row = format!("  {}  {}", " ", action);
            canvas.push(&row.dimmed().to_string());
        }
    }
    canvas.blank();
    canvas.push(&format!(
        "  {}  {}",
        "↑/w ↓/s".truecolor(255, 110, 0),
        "Navigate  |  Enter Select  |  Esc Back  |  q Quit".truecolor(100, 100, 100)
    ));
}

/// Re-enter raw mode for the TUI after a sub-command has run. Clears nothing by
/// itself; the caller resets `prev_lines = 0` so the next render issues a fresh
/// `FULL_CLEAR`.
fn reset_tui(stdout: &mut Stdout) {
    let _ = enable_raw_mode();
    execute!(stdout, Hide).ok();
}

pub fn interactive_menu() {
    let _ = enable_raw_mode();
    let mut stdout = stdout();
    execute!(stdout, Hide).ok();
    let _ = stdout.write_all(FULL_CLEAR.as_bytes());
    let _ = stdout.flush();
    let mut store = load_macros();

    'main: loop {
        let keys: Vec<String> = {
            let mut k: Vec<String> = store.macros.keys().cloned().collect();
            k.sort();
            k
        };
        let total = keys.len() + 1;
        let mut selected: usize = total - 1;

        let mut prev_lines: usize = 0;
        loop {
            let idx = if selected >= keys.len() {
                MenuIdx::Exit
            } else {
                MenuIdx::Macro(selected)
            };
            let mut canvas = Canvas::new();
            draw_main_menu(&mut canvas, &store, &keys, &idx);
            prev_lines = render(&mut stdout, prev_lines, &canvas);

            match read_key() {
                Some(KeyCode::Up) => selected = selected.saturating_sub(1),
                Some(KeyCode::Down) => selected = (selected + 1).min(total - 1),
                Some(KeyCode::Enter) => {
                    if selected >= keys.len() {
                        break 'main;
                    }
                    break;
                }
                Some(KeyCode::Char('q')) | Some(KeyCode::Esc) => break 'main,
                _ => {}
            }
        }

        let key = keys[selected].clone();
        let mut action_sel: usize = 0;
        let mut prev_lines: usize = 0;

        loop {
            let mut canvas = Canvas::new();
            draw_actions_menu(&mut canvas, &store, &key, action_sel);
            prev_lines = render(&mut stdout, prev_lines, &canvas);

            match read_key() {
                Some(KeyCode::Up) => action_sel = action_sel.saturating_sub(1),
                Some(KeyCode::Down) => action_sel = (action_sel + 1).min(3),
                Some(KeyCode::Enter) => match action_sel {
                    0 => {
                        let cmd = store.macros[&key].command.clone();
                        execute!(stdout, Show).ok();
                        let _ = disable_raw_mode();
                        println!();
                        run_command(&cmd);
                        pause_before_continue();
                        store = load_macros();
                        reset_tui(&mut stdout);
                        prev_lines = 0;
                    }
                    1 => {
                        let current = &store.macros[&key].command;
                        execute!(stdout, Show).ok();
                        let _ = disable_raw_mode();
                        println!();
                        display::info(format!("Current: {}", current));
                        let new_cmd = input_line(&format!(
                            "  {}  {} ",
                            ">".truecolor(255, 110, 0),
                            "New command:".bright_white()
                        ));
                        if !new_cmd.is_empty() {
                            if let Some(entry) = store.macros.get_mut(&key) {
                                entry.command = new_cmd.clone();
                                entry.updated_at = timestamp_now();
                            }
                            let _ = save_macros(&store);
                            println!();
                            display::ok("Macro updated");
                        } else {
                            println!();
                            display::warn("Update cancelled");
                        }
                        pause_before_continue();
                        store = load_macros();
                        reset_tui(&mut stdout);
                        prev_lines = 0;
                    }
                    2 => {
                        execute!(stdout, Show).ok();
                        let _ = disable_raw_mode();
                        println!();
                        display::warn(format!("Delete macro '{}'?", key));
                        let confirm = input_line(&format!(
                            "  {}  {} ",
                            ">".truecolor(255, 110, 0),
                            "Are you sure? (y/N):".bright_white()
                        ));
                        if confirm.to_lowercase() == "y" {
                            store.macros.remove(&key);
                            let _ = save_macros(&store);
                            println!();
                            display::ok(format!("Removed macro '{}'", key));
                            pause_before_continue();
                            store = load_macros();
                            reset_tui(&mut stdout);
                            continue 'main;
                        } else {
                            println!();
                            display::info("Delete cancelled");
                            pause_before_continue();
                            reset_tui(&mut stdout);
                            prev_lines = 0;
                        }
                    }
                    3 => break,
                    _ => {}
                },
                Some(KeyCode::Esc) => break,
                Some(KeyCode::Char('q')) => break 'main,
                _ => {}
            }
        }
    }

    let _ = stdout.write_all(FULL_CLEAR.as_bytes());
    execute!(stdout, Show).ok();
    let _ = stdout.flush();
    let _ = disable_raw_mode();
}