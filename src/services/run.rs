use colored::Colorize;

use crate::platform;

/// Runs the macro `key` and returns the exit code the process should use.
pub fn cmd_run(key: &str) -> i32 {
    if !platform::color_enabled() {
        colored::control::set_override(false);
    }

    let store = match crate::utils::storage::load() {
        Ok(store) => store,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };

    let Some(entry) = store.macros.get(key) else {
        eprintln!("Macro '{key}' not found. Run `velo list` to see your macros.");
        return 1;
    };

    println!("{}", entry.command.dimmed());
    println!();

    match platform::run_macro(&entry.command) {
        Ok(Some(0)) => 0,
        Ok(Some(code)) => {
            eprintln!("Macro '{key}' exited with code {code}.");
            code
        }
        Ok(None) => {
            eprintln!("Macro '{key}' was terminated by a signal.");
            1
        }
        Err(e) => {
            eprintln!("Could not start macro '{key}': {e}");
            127
        }
    }
}
