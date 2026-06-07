use colored::*;

use crate::utils::storage::load_macros;

pub fn cmd_run(key: &str) {
    let store = load_macros();

    match store.macros.get(key) {
        Some(entry) => {
            let (shell, flag) = if cfg!(target_os = "windows") {
                ("cmd", "/C")
            } else {
                ("sh", "-c")
            };

            println!("{}", entry.command.bright_yellow());
            println!();

            let status = std::process::Command::new(shell)
                .arg(flag)
                .arg(&entry.command)
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
                    format!("Macro '{}' exited with code {}", key, code).bright_white()
                );
            }
        }
        None => {
            eprintln!();
            eprintln!(
                "  {}  {}",
                "✖".bright_red(),
                format!("Macro '{}' not found", key).bright_white()
            );
            eprintln!(
                "  {}  {}",
                "◆".bright_cyan(),
                "Use: velo".bright_white()
            );
            eprintln!();
            std::process::exit(1);
        }
    }
}
