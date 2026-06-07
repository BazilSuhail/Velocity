use colored::*;
use std::io::Write;

use crate::utils::display;
use crate::utils::storage::{load_macros, save_macros};

fn prompt(msg: &str) -> String {
    print!("  {}  {} ", "▸".bright_cyan(), msg.bright_white());
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

pub fn cmd_delete(key_opt: Option<&str>) {
    let store = load_macros();

    if let Some(key) = key_opt {
        let mut store = store;
        if store.macros.remove(key).is_some() {
            let _ = save_macros(&store);
            println!();
            display::ok(format!("Removed macro '{}'", key));
            println!();
        } else {
            println!();
            display::err(format!("Macro '{}' not found", key));
            println!();
        }
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

    let key = prompt("Enter macro name to delete:");
    if key.is_empty() {
        println!();
        display::err("Name cannot be empty.");
        println!();
        return;
    }

    let mut store = store;
    if store.macros.remove(&key).is_some() {
        let _ = save_macros(&store);
        println!();
        display::ok(format!("Removed macro '{}'", key));
        println!();
    } else {
        println!();
        display::err(format!("Macro '{}' not found", key));
        println!();
    }
}
