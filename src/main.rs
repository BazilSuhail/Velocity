mod utils;

use clap::Parser;
use colored::*;

use utils::{add, delete, list, run, tui, update};

fn print_colored_help() {
    println!();
    // Modern un-boxed ASCII art with a smooth Red -> Crimson -> Orange gradient
    println!("{}", "      ██╗   ██╗███████╗██╗     ██████╗ ".truecolor(255, 50, 50).bold());
    println!("{}", "      ██║   ██║██╔════╝██║    ██╔═══██╗".truecolor(255, 90, 30).bold());
    println!("{}", "      ██║   ██║█████╗  ██║    ██║   ██║".truecolor(255, 130, 10).bold());
    println!("{}", "      ╚██╗ ██╔╝██╔══╝  ██║    ██║   ██║".truecolor(255, 160, 0).bold());
    println!("{}", "       ╚████╔╝ ███████╗███████╗╚██████╔╝".truecolor(255, 190, 0).bold());
    println!("{}", "        ╚═══╝  ╚══════╝╚══════╝ ╚═════╝ ".truecolor(255, 210, 0).bold());

    println!();
    println!(
        "  {} {} {}",
        "by".truecolor(120, 120, 120),
        "Bazil Suhail".truecolor(255, 180, 80).bold(),
        "»  bazilsuhail.netlify.app".truecolor(255, 140, 40)
    );
    println!(
        "  {} {} {} {}",
        "⚡".truecolor(255, 110, 0),
        "UNIVERSAL COMMAND MACRO ENGINE".truecolor(255, 60, 60).bold(),
        "»".truecolor(100, 100, 100),
        format!("v{}", env!("CARGO_PKG_VERSION")).truecolor(255, 150, 0).bold()
    );
    println!("{}", "  ──────────────────────────────────────────────────────────────────────".truecolor(70, 70, 70));
    println!();

    println!(
        "  {}  velo [{}] [{}]",
        "RUN:".truecolor(255, 60, 60).bold(),
        "KEY".truecolor(255, 150, 0),
        "COMMAND".truecolor(255, 150, 0)
    );
    println!();

    println!("  {}", "CORE COMMANDS:".truecolor(255, 60, 60).bold());
    println!(
        "    {:<18} {}",
        "add".truecolor(255, 130, 0),
        "Instantly map a new macro string".bright_white()
    );
    println!(
        "    {:<18} {}",
        "delete".truecolor(255, 130, 0),
        "Purge a macro from your registry".bright_white()
    );
    println!(
        "    {:<18} {}",
        "list".truecolor(255, 130, 0),
        "Stream all saved macros to stdout".bright_white()
    );
    println!(
        "    {:<18} {}",
        "update".truecolor(255, 130, 0),
        "Modify definitions on the fly".bright_white()
    );
    println!(
        "    {:<18} {}",
        "help".truecolor(255, 130, 0),
        "Display this terminal interface".bright_white()
    );
    println!();

    println!("  {}", "DIRECT ARGUMENTS:".truecolor(255, 60, 60).bold());
    println!(
        "    {:<18} {}",
        "[KEY]".truecolor(255, 150, 0),
        "Executes a stored macro mapping instantly (e.g., velo dev)".bright_white()
    );
    println!();

    println!("  {}", "QUICK EXAMPLES:".truecolor(255, 60, 60).bold());
    let gray_pipe = "│".truecolor(90, 90, 90);
    println!("    velo                        {}  Launch interactive HUD wrapper", gray_pipe);
    println!("    velo add                    {}  Interactive step-by-step setup", gray_pipe);
    println!("    velo add dev \"npm run dev\"  {}  Bind key directly without prompts", gray_pipe);
    println!("    velo dev                    {}  Fires the 'dev' execution sequence", gray_pipe);
    println!("    velo list                   {}  Dump all records cleanly", gray_pipe);
    println!();

    println!("{}", "  ──────────────────────────────────────────────────────────────────────".truecolor(70, 70, 70));
    println!(
        "  {} ~/.config/velo/macros.json",
        "Registry [Unix]:".truecolor(120, 120, 120)
    );
    println!(
        "  {} %USERPROFILE%\\AppData\\Roaming\\velo\\macros.json",
        "Registry [Win] :".truecolor(120, 120, 120)
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

    /// Macro key to run (e.g. `velo dev`)
    key: Option<String>,
}

#[derive(Parser)]
enum Commands {
    /// Add a new macro
    Add {
        key: Option<String>,
        command: Option<String>,
    },
    /// Delete a macro
    Delete { key: Option<String> },
    /// List all macros
    List,
    /// Update an existing macro
    Update {
        key: Option<String>,
        command: Option<String>,
    },
}

fn main() {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        print_colored_help();
        return;
    }

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { key, command }) => add::cmd_add(key.as_deref(), command.as_deref()),
        Some(Commands::Delete { key }) => delete::cmd_delete(key.as_deref()),
        Some(Commands::List) => list::cmd_list(),
        Some(Commands::Update { key, command }) => {
            update::cmd_update(key.as_deref(), command.as_deref())
        }
        None if cli.key.is_some() => run::cmd_run(cli.key.as_ref().unwrap()),
        None => tui::interactive_menu(),
    }
}