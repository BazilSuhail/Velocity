mod platform;
mod services;
mod tui;
mod utils;

use clap::{Parser, Subcommand};
use colored::*;

use services::{add, delete, list, run, update};
use utils::display;

fn print_colored_help() {
    println!();
    println!(
        "{}",
        "      ██╗   ██╗███████╗██╗     ██████╗ "
            .truecolor(255, 50, 50)
            .bold()
    );
    println!(
        "{}",
        "      ██║   ██║██╔════╝██║    ██╔═══██╗"
            .truecolor(255, 90, 30)
            .bold()
    );
    println!(
        "{}",
        "      ██║   ██║█████╗  ██║    ██║   ██║"
            .truecolor(255, 130, 10)
            .bold()
    );
    println!(
        "{}",
        "      ╚██╗ ██╔╝██╔══╝  ██║    ██║   ██║"
            .truecolor(255, 160, 0)
            .bold()
    );
    println!(
        "{}",
        "       ╚████╔╝ ███████╗███████╗╚██████╔╝"
            .truecolor(255, 190, 0)
            .bold()
    );
    println!(
        "{}",
        "        ╚═══╝  ╚══════╝╚══════╝ ╚═════╝ "
            .truecolor(255, 210, 0)
            .bold()
    );
    println!();
    println!(
        "  {} {} {}  {}",
        "by".truecolor(120, 120, 120),
        "Bazil Suhail".truecolor(255, 180, 80).bold(),
        "»".truecolor(100, 100, 100),
        "bazilsuhail.netlify.app".truecolor(255, 140, 40)
    );
    println!(
        "  {} {} {} {}",
        "⚡".truecolor(255, 110, 0),
        "UNIVERSAL COMMAND MACRO ENGINE"
            .truecolor(255, 60, 60)
            .bold(),
        "»".truecolor(100, 100, 100),
        format!("v{}", env!("CARGO_PKG_VERSION"))
            .truecolor(255, 150, 0)
            .bold()
    );
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────────────────────"
            .truecolor(70, 70, 70)
    );
    println!();

    println!(
        "  {}  velo [{}] | velo {}",
        "USAGE:".truecolor(255, 60, 60).bold(),
        "KEY".truecolor(255, 150, 0),
        "<COMMAND>".truecolor(255, 150, 0)
    );
    println!();

    println!("  {}", "COMMANDS:".truecolor(255, 60, 60).bold());
    // Pad before coloring: ANSI codes would otherwise defeat the width spec.
    let cmd = |name: &str, desc: &str| {
        println!(
            "    {} {}",
            format!("{name:<28}").truecolor(255, 130, 0),
            desc.bright_white()
        )
    };
    cmd("add [KEY] [COMMAND]", "Save a new macro");
    cmd("update [KEY] [COMMAND]", "Change a macro's command");
    cmd("delete [KEY] [-y, --yes]", "Remove a macro (asks first)");
    cmd("list", "Show all macros");
    cmd("run <KEY>", "Run a macro (same as velo <KEY>)");
    println!();

    println!("  {}", "ARGUMENTS:".truecolor(255, 60, 60).bold());
    println!(
        "    {} {}",
        format!("{:<28}", "<KEY>").truecolor(255, 150, 0),
        "Run a stored macro (e.g. velo dev)".bright_white()
    );
    println!();

    println!(
        "  {}",
        "TUI KEYS (plain velo in a terminal):"
            .truecolor(255, 60, 60)
            .bold()
    );
    println!(
        "    {}",
        "Enter actions  n new  e edit  d delete  / filter  ? help  q quit".bright_white()
    );
    println!();

    println!("  {}", "ENVIRONMENT:".truecolor(255, 60, 60).bold());
    println!(
        "    {} {}",
        format!("{:<28}", "VELO_HOME").truecolor(255, 150, 0),
        "Directory for macros.json".bright_white()
    );
    println!(
        "    {} {}",
        format!("{:<28}", "NO_COLOR").truecolor(255, 150, 0),
        "Disable colored output".bright_white()
    );
    println!();

    println!(
        "{}",
        "  ──────────────────────────────────────────────────────────────────────"
            .truecolor(70, 70, 70)
    );
    println!(
        "  {} ~/.config/velo/macros.json",
        "Registry [Linux]:".truecolor(120, 120, 120)
    );
    println!(
        "  {} ~/Library/Application Support/velo/macros.json",
        "Registry [macOS]:".truecolor(120, 120, 120)
    );
    println!(
        "  {} %APPDATA%\\velo\\macros.json",
        "Registry [Win]  :".truecolor(120, 120, 120)
    );
    println!();
}

#[derive(Parser)]
#[command(
    name = "velo",
    version,
    about = "⚡ Universal Command Macro Engine",
    long_about = "velo is a cross-platform CLI tool for storing and running \
                  frequently used commands as named macros."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// KEY to run directly
    key: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Save a new macro
    Add {
        key: Option<String>,
        command: Option<String>,
    },
    /// Remove a macro
    Delete {
        key: Option<String>,
        /// Delete without asking for confirmation
        #[arg(short, long)]
        yes: bool,
    },
    /// Show all macros
    List,
    /// Run a macro
    Run { key: String },
    /// Change a macro's command
    Update {
        key: Option<String>,
        command: Option<String>,
    },
}

fn main() {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    if !platform::color_enabled() {
        colored::control::set_override(false);
    }

    if std::env::args()
        .nth(1)
        .is_some_and(|a| a == "-h" || a == "--help")
    {
        print_colored_help();
        return;
    }

    let cli = Cli::parse();
    let code = match cli.command {
        Some(Commands::Add { key, command }) => add::cmd_add(key.as_deref(), command.as_deref()),
        Some(Commands::Update { key, command }) => {
            update::cmd_update(key.as_deref(), command.as_deref())
        }
        Some(Commands::Delete { key, yes }) => delete::cmd_delete(key.as_deref(), yes),
        Some(Commands::List) => list::cmd_list(),
        Some(Commands::Run { key }) => run::cmd_run(&key),
        None => match cli.key {
            Some(key) => run::cmd_run(&key),
            None if platform::is_interactive() => match tui::run() {
                Ok(()) => 0,
                Err(e) => {
                    display::err(format!("terminal UI failed: {e}"));
                    1
                }
            },
            None => list::cmd_list(),
        },
    };

    std::process::exit(code);
}
