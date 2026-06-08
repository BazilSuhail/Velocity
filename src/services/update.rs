use crate::platform;
use crate::services::{arg_or_prompt, load_or_report, save_or_report};
use crate::utils::display;
use crate::utils::storage;

const USAGE: &str = "velo update <KEY> <COMMAND>";

/// Exit codes: 0 ok, 1 user/storage error, 2 usage error.
pub fn cmd_update(key: Option<&str>, command: Option<&str>) -> i32 {
    let Some(mut store) = load_or_report() else {
        return 1;
    };

    if platform::is_interactive() && (key.is_none() || command.is_none()) {
        display::info("Current macros:");
        display::list_table(&store);
    }

    let Some(key) = arg_or_prompt(key, "Enter macro name to update:", USAGE) else {
        return 2;
    };
    let Some(current) = store.macros.get(&key).map(|e| e.command.clone()) else {
        display::err(format!("Macro '{key}' not found."));
        return 1;
    };
    if command.is_none() && platform::is_interactive() {
        display::info(format!("Current command: {current}"));
    }

    let Some(command) = arg_or_prompt(command, "New command:", USAGE) else {
        return 2;
    };
    if command.trim().is_empty() {
        display::err("Command cannot be empty.");
        return 1;
    }

    let Some(entry) = store.macros.get_mut(&key) else {
        display::err(format!("Macro '{key}' not found."));
        return 1;
    };
    entry.command = command.clone();
    entry.updated_at = storage::timestamp_now();
    if !save_or_report(&store) {
        return 1;
    }
    display::ok(format!("Updated macro '{key}'  {command}"));
    0
}
