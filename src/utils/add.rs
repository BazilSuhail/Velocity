use colored::*;
use std::io::Write;

use crate::utils::display;
use crate::utils::storage::{load_macros, save_macros, timestamp_now, MacroEntry};

fn prompt(msg: &str) -> String {
    print!("  {}  {} ", "▸".bright_cyan(), msg.bright_white());
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

pub fn cmd_add(key_opt: Option<&str>, cmd_opt: Option<&str>) {
    let store = load_macros();

    if let (Some(key), Some(command)) = (key_opt, cmd_opt) {
        let key = key.to_string();
        let command = command.to_string();
        if store.macros.contains_key(&key) {
            println!();
            display::warn(format!("Macro '{}' already exists.", key));
            println!();
            return;
        }
        let mut store = store;
        let now = timestamp_now();
        store.macros.insert(
            key.clone(),
            MacroEntry {
                command: command.clone(),
                created_at: now.clone(),
                updated_at: now,
            },
        );
        let _ = save_macros(&store);
        println!();
        display::ok("Macro saved");
        println!(
            "  {} {}  {} {}",
            "▸".bright_cyan(),
            key.bright_cyan(),
            "→".bright_black(),
            command.bright_yellow()
        );
        println!();
        return;
    }

    println!();
    display::info("Current macros:");
    display::list_table(&store);

    let key = prompt("Enter macro name:");
    if key.is_empty() {
        println!();
        display::err("Name cannot be empty.");
        println!();
        return;
    }

    if store.macros.contains_key(&key) {
        println!();
        display::warn(format!("Macro '{}' already exists.", key));
        println!();
        return;
    }

    let command = prompt("Enter command:");
    if command.is_empty() {
        println!();
        display::err("Command cannot be empty.");
        println!();
        return;
    }

    let mut store = store;
    let now = timestamp_now();
    store.macros.insert(
        key.clone(),
        MacroEntry {
            command: command.clone(),
            created_at: now.clone(),
            updated_at: now,
        },
    );
    let _ = save_macros(&store);

    println!();
    display::ok("Macro saved");
    println!(
        "  {} {}  {} {}",
        "▸".bright_cyan(),
        key.bright_cyan(),
        "→".bright_black(),
        command.bright_yellow()
    );
    println!();
}
