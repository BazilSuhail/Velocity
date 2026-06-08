use crate::platform;
use crate::services::{arg_or_prompt, load_or_report, save_or_report};
use crate::utils::display;
use crate::utils::storage::{self, MacroEntry};

const USAGE: &str = "velo add <KEY> <COMMAND>";

/// Exit codes: 0 ok, 1 user/storage error, 2 usage error.
pub fn cmd_add(key: Option<&str>, command: Option<&str>) -> i32 {
    let Some(mut store) = load_or_report() else {
        return 1;
    };

    if platform::is_interactive() && (key.is_none() || command.is_none()) {
        display::info("Current macros:");
        display::list_table(&store);
    }

    let Some(key) = arg_or_prompt(key, "Enter macro name:", USAGE) else {
        return 2;
    };
    let Some(command) = arg_or_prompt(command, "Enter command:", USAGE) else {
        return 2;
    };

    if let Err(msg) = storage::validate_key(&key) {
        display::err(msg);
        return 1;
    }
    if command.trim().is_empty() {
        display::err("Command cannot be empty.");
        return 1;
    }
    if store.macros.contains_key(&key) {
        display::err(format!("Macro '{key}' already exists."));
        display::hint(format!(
            "Use `velo update {key} \"<COMMAND>\"` to change it."
        ));
        return 1;
    }

    let now = storage::timestamp_now();
    store.macros.insert(
        key.clone(),
        MacroEntry {
            command: command.clone(),
            created_at: now.clone(),
            updated_at: now,
        },
    );
    if !save_or_report(&store) {
        return 1;
    }
    display::ok(format!("Saved macro '{key}'  {command}"));
    0
}
