use crate::platform;
use crate::utils::display;
use crate::utils::storage::{self, MacroStore};

pub mod add;
pub mod delete;
pub mod list;
pub mod run;
pub mod update;

/// Loads the macro store, reporting the error to stderr on failure.
pub fn load_or_report() -> Option<MacroStore> {
    match storage::load() {
        Ok(store) => Some(store),
        Err(e) => {
            display::err(e.to_string());
            None
        }
    }
}

/// Saves the macro store, reporting the error to stderr on failure.
pub fn save_or_report(store: &MacroStore) -> bool {
    match storage::save(store) {
        Ok(()) => true,
        Err(e) => {
            display::err(e.to_string());
            false
        }
    }
}

/// Uses the argument if given, prompts for it when interactive, and otherwise
/// reports `usage` and returns None (the caller exits with code 2).
pub fn arg_or_prompt(value: Option<&str>, question: &str, usage: &str) -> Option<String> {
    match value {
        Some(v) => Some(v.to_string()),
        None if platform::is_interactive() => Some(display::prompt(question)),
        None => {
            display::err(format!("Missing argument. Usage: {usage}"));
            None
        }
    }
}
