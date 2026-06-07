use colored::*;
use std::io::Write;

use crate::utils::display;
use crate::utils::storage::{load_macros, save_macros, timestamp_now};

pub fn cmd_update(key_opt: Option<&str>, cmd_opt: Option<&str>) {
    let store = load_macros();

    if let (Some(key), Some(command)) = (key_opt, cmd_opt) {
        if !store.macros.contains_key(key) {
            println!();
            display::err(format!("Macro '{}' not found.", key));
            println!();
            return;
        }
        let mut store = store;
        if let Some(entry) = store.macros.get_mut(key) {
            entry.command = command.to_string();
            entry.updated_at = timestamp_now();
        }
        let _ = save_macros(&store);
        println!();
        display::ok(format!("Updated macro '{}'", key));
        println!(
            "  {} {}  {} {}",
            "↻".bright_cyan(),
            key.bright_cyan(),
            "→".bright_black(),
            command.bright_yellow()
        );
        println!();
        return;
    }

    if store.macros.is_empty() {
        println!();
        display::warn("No macros stored.");
        println!();
        return;
    }

    println!();
    display::info("Current macros:");
    display::list_table(&store);

    let key = {
        print!(
            "  {}  {} ",
            "▸".bright_cyan(),
            "Enter macro name to update:".bright_white()
        );
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    };

    if key.is_empty() {
        println!();
        display::err("Name cannot be empty.");
        println!();
        return;
    }

    if !store.macros.contains_key(&key) {
        println!();
        display::err(format!("Macro '{}' not found.", key));
        println!();
        return;
    }

    let current = &store.macros[&key].command;
    println!();
    display::info(format!("Current command: {}", current));

    let command = {
        print!(
            "  {}  {} ",
            "▸".bright_cyan(),
            "New command:".bright_white()
        );
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    };

    if command.is_empty() {
        println!();
        display::warn("Update cancelled.");
        println!();
        return;
    }

    let mut store = store;
    if let Some(entry) = store.macros.get_mut(&key) {
        entry.command = command.clone();
        entry.updated_at = timestamp_now();
    }
    let _ = save_macros(&store);

    println!();
    display::ok(format!("Updated macro '{}'", key));
    println!(
        "  {} {}  {} {}",
        "↻".bright_cyan(),
        key.bright_cyan(),
        "→".bright_black(),
        command.bright_yellow()
    );
    println!();
}
