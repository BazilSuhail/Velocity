use colored::*;

use crate::services::load_or_report;
use crate::utils::display;

/// Prints the compact header and macro table. Exit codes: 0 ok, 1 storage error.
pub fn cmd_list() -> i32 {
    let Some(store) = load_or_report() else {
        return 1;
    };

    println!();
    println!(
        "  {}  {}",
        "velo".truecolor(255, 60, 60).bold(),
        format!("· {} macro(s)", store.macros.len()).truecolor(255, 150, 0)
    );
    display::list_table(&store);
    0
}
