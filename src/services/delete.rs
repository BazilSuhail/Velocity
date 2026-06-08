use crate::platform;
use crate::services::{arg_or_prompt, load_or_report, save_or_report};
use crate::utils::display;

const USAGE: &str = "velo delete <KEY> [--yes]";

/// Exit codes: 0 ok or cancelled, 1 user/storage error, 2 usage error.
pub fn cmd_delete(key: Option<&str>, yes: bool) -> i32 {
    let Some(mut store) = load_or_report() else {
        return 1;
    };
    let interactive = platform::is_interactive();

    if key.is_none() && interactive {
        if store.macros.is_empty() {
            display::warn("No macros stored.");
            return 0;
        }
        display::info("Current macros:");
        display::list_table(&store);
    }
    let Some(key) = arg_or_prompt(key, "Enter macro name to delete:", USAGE) else {
        return 2;
    };

    if !store.macros.contains_key(&key) {
        display::err(format!("Macro '{key}' not found."));
        return 1;
    }

    if !yes {
        if !interactive {
            display::err("Use --yes to delete without a prompt.");
            return 2;
        }
        let answer = display::prompt(&format!("Delete '{key}'? [y/N]"));
        if !matches!(answer.to_lowercase().as_str(), "y" | "yes") {
            display::warn("Cancelled.");
            return 0;
        }
    }

    store.macros.remove(&key);
    if !save_or_report(&store) {
        return 1;
    }
    display::ok(format!("Removed macro '{key}'"));
    0
}
