use colored::*;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{stdin, stdout, Write};

use crate::utils::display::{self, fmt_dt};
use crate::utils::storage::{load_macros, save_macros, timestamp_now};

use crossterm::cursor::MoveTo;

enum MenuIdx {
    Macro(usize),
    Exit,
}

fn read_key() -> Option<KeyCode> {
    if event::poll(std::time::Duration::from_millis(200)).ok()? {
        if let Event::Key(k) = event::read().ok()? {
            if k.kind == KeyEventKind::Press {
                return Some(k.code);
            }
        }
    }
    None
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

fn draw_banner(count: usize) {
    println!("{}", "      ██╗   ██╗███████╗██╗     ██████╗ ".truecolor(255, 50, 50).bold());
    println!("{}", "      ██║   ██║██╔════╝██║    ██╔═══██╗".truecolor(255, 90, 30).bold());
    println!("{}", "      ██║   ██║█████╗  ██║    ██║   ██║".truecolor(255, 130, 10).bold());
    println!("{}", "      ╚██╗ ██╔╝██╔══╝  ██║    ██║   ██║".truecolor(255, 160, 0).bold());
    println!("{}", "       ╚████╔╝ ███████╗███████╗╚██████╔╝".truecolor(255, 190, 0).bold());
    println!("{}", "        ╚═══╝  ╚══════╝╚══════╝ ╚═════╝ ".truecolor(255, 210, 0).bold());
    println!();
    println!(
        "  {} {} {}",
        "by".truecolor(120, 120, 120),
        "Bazil Suhail".truecolor(255, 180, 80).bold(),
        "»  bazilsuhail.netlify.app".truecolor(255, 140, 40)
    );
    println!(
        "  {} {} {} {}",
        "Universal Command Macro Engine".truecolor(255, 60, 60).bold(),
        "•".truecolor(100, 100, 100),
        format!("{} macros loaded", count).truecolor(255, 150, 0),
        "•".truecolor(100, 100, 100),
    );
    println!(
        "  {} {} {} {}",
        "Version".truecolor(120, 120, 120),
        format!("v{}", env!("CARGO_PKG_VERSION")).truecolor(255, 150, 0),
        "»".truecolor(100, 100, 100),
        "⌨  ↑↓ navigate  ↵ select  q quit".truecolor(100, 100, 100)
    );
    println!("{}", "  ════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70));
    println!();
}

fn draw_main_menu(
    stdout: &mut std::io::Stdout,
    store: &crate::utils::storage::MacroStore,
    keys: &[String],
    selected: &MenuIdx,
) {
    execute!(stdout, MoveTo(0, 0)).ok();

    let count = keys.len();
    draw_banner(count);

    if keys.is_empty() {
        println!(
            "  {}  {}",
            "▲".bright_yellow(),
            "No macros stored. Use `velo add` to create one.".bright_white()
        );
        println!();
    } else {
        println!(
            "  {:<20} {:<38} {:<8} {}",
            "KEY".truecolor(255, 130, 0).bold(),
            "COMMAND".truecolor(255, 130, 0).bold(),
            "DATE".truecolor(255, 130, 0).bold(),
            "TIME".truecolor(255, 130, 0).bold()
        );
        println!("  {}", "════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70));

        for (i, key) in keys.iter().enumerate() {
            let entry = &store.macros[key];
            let (date, time) = fmt_dt(entry);
            let is_sel = matches!(selected, MenuIdx::Macro(j) if *j == i);
            let prefix = if is_sel { "▸".truecolor(255, 110, 0) } else { " ".bright_white() };
            let key_display = if is_sel {
                key.as_str().truecolor(255, 130, 0).bold()
            } else {
                key.as_str().truecolor(255, 150, 0)
            };
            println!(
                "  {} {:<20} {:<38} {:<8} {}",
                prefix,
                key_display,
                entry.command.as_str().bright_white(),
                date.truecolor(255, 190, 0),
                time.truecolor(255, 190, 0),
            );
        }
    }

    println!("  {}", "════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70));

    let exit_sel = matches!(selected, MenuIdx::Exit);
    let exit_prefix = if exit_sel { "▸".truecolor(255, 110, 0) } else { " ".bright_white() };
    let exit_text = if exit_sel {
        "Exit".truecolor(255, 130, 0).bold()
    } else {
        "Exit".bright_cyan()
    };
    println!("  {} {}", exit_prefix, exit_text);
    println!();

    execute!(stdout, Clear(ClearType::FromCursorDown)).ok();
    stdout.flush().ok();
}

fn draw_actions_menu(
    stdout: &mut std::io::Stdout,
    store: &crate::utils::storage::MacroStore,
    key: &str,
    selected: usize,
) {
    let entry = &store.macros[key];
    let actions = ["Run", "Update", "Delete", "Back"];

    execute!(stdout, MoveTo(0, 0)).ok();

    draw_banner(store.macros.len());

    let (date, time) = fmt_dt(entry);
    println!(
        "  {}  {}  {}  {}  {}  {}",
        "▸".truecolor(255, 110, 0),
        key.truecolor(255, 130, 0).bold(),
        "→".truecolor(100, 100, 100),
        entry.command.truecolor(255, 190, 0),
        date.truecolor(255, 190, 0),
        time.truecolor(255, 190, 0),
    );
    println!();
    println!("  {}", "Actions:".truecolor(255, 210, 0).bold());
    println!("  {}", "══════════════════════════════════".truecolor(70, 70, 70));

    for (i, action) in actions.iter().enumerate() {
        let prefix = if i == selected { "▸".truecolor(255, 110, 0) } else { " ".bright_white() };
        let text = if i == selected {
            action.truecolor(255, 130, 0).bold()
        } else {
            action.bright_white()
        };
        println!("  {}  {}", prefix, text);
    }
    println!();
    println!(
        "  {}  {}",
        "↑↓".truecolor(255, 110, 0),
        "Navigate  |  Enter Select  |  Esc Back".truecolor(100, 100, 100)
    );

    execute!(stdout, Clear(ClearType::FromCursorDown)).ok();
    stdout.flush().ok();
}

pub fn interactive_menu() {
    let _ = terminal::enable_raw_mode();
    let mut stdout = stdout();
    execute!(stdout, Hide, Clear(ClearType::All)).ok();
    let mut store = load_macros();

    'main: loop {
        let keys: Vec<String> = {
            let mut k: Vec<String> = store.macros.keys().cloned().collect();
            k.sort();
            k
        };
        let total = keys.len() + 1;
        let mut selected: usize = total - 1;

        loop {
            let idx = if selected >= keys.len() {
                MenuIdx::Exit
            } else {
                MenuIdx::Macro(selected)
            };
            draw_main_menu(&mut stdout, &store, &keys, &idx);

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

        loop {
            draw_actions_menu(&mut stdout, &store, &key, action_sel);

            match read_key() {
                Some(KeyCode::Up) => action_sel = action_sel.saturating_sub(1),
                Some(KeyCode::Down) => action_sel = (action_sel + 1).min(3),
                Some(KeyCode::Enter) => match action_sel {
                    0 => {
                        let cmd = store.macros[&key].command.clone();
                        execute!(stdout, Show).ok();
                        let _ = terminal::disable_raw_mode();
                        println!();
                        run_command(&cmd);
                        pause_before_continue();
                        store = load_macros();
                        let _ = terminal::enable_raw_mode();
                        execute!(stdout, Hide, Clear(ClearType::All)).ok();
                    }
                    1 => {
                        execute!(stdout, Show).ok();
                        let _ = terminal::disable_raw_mode();
                        println!();
                        let current = &store.macros[&key].command;
                        display::info(format!("Current: {}", current));
                        let new_cmd = input_line(&format!(
                            "  {}  {} ",
                            "▸".truecolor(255, 110, 0),
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
                        let _ = terminal::enable_raw_mode();
                        execute!(stdout, Hide, Clear(ClearType::All)).ok();
                    }
                    2 => {
                        execute!(stdout, Show).ok();
                        let _ = terminal::disable_raw_mode();
                        println!();
                        display::warn(format!("Delete macro '{}'?", key));
                        let confirm = input_line(&format!(
                            "  {}  {} ",
                            "▸".truecolor(255, 110, 0),
                            "Are you sure? (y/N):".bright_white()
                        ));
                        if confirm.to_lowercase() == "y" {
                            store.macros.remove(&key);
                            let _ = save_macros(&store);
                            println!();
                            display::ok(format!("Removed macro '{}'", key));
                            pause_before_continue();
                            store = load_macros();
                            let _ = terminal::enable_raw_mode();
                            execute!(stdout, Hide, Clear(ClearType::All)).ok();
                            continue 'main;
                        } else {
                            println!();
                            display::info("Delete cancelled");
                            pause_before_continue();
                            let _ = terminal::enable_raw_mode();
                            execute!(stdout, Hide, Clear(ClearType::All)).ok();
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

    execute!(stdout, Show).ok();
    let _ = terminal::disable_raw_mode();
}
