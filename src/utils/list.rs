use colored::*;

use crate::utils::display;
use crate::utils::storage::load_macros;

pub fn cmd_list() {
    let store = load_macros();
    let count = store.macros.len();

    println!();
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
        "Universal Command Macro Engine".truecolor(255, 60, 60).bold(),
        "•".truecolor(100, 100, 100),
        format!("{} macros loaded", count).truecolor(255, 150, 0),
        "•".truecolor(100, 100, 100),
    );
    println!(
        "  {} {} {} {}",
        "Version".truecolor(120, 120, 120),
        format!("v{}", env!("CARGO_PKG_VERSION")).truecolor(255, 150, 0),
        "»".truecolor(100, 100, 100),
        "⌨  ↑↓ navigate  ↵ select  q quit".truecolor(100, 100, 100)
    );
    println!("{}", "  ════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70));
    display::list_table(&store);
}
