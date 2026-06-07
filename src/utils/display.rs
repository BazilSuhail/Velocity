use chrono::{Datelike, Timelike};
use colored::*;

use crate::utils::storage::MacroEntry;

pub fn fmt_dt(entry: &MacroEntry) -> (String, String) {
    if let Ok(dt) =
        chrono::NaiveDateTime::parse_from_str(&entry.updated_at, "%Y-%m-%d %H:%M:%S")
    {
        let date = format!("{}/{}/{}", dt.month(), dt.day(), dt.year() % 100);
        let time = format!("{}:{:02}", dt.hour(), dt.minute());
        (date, time)
    } else {
        (entry.updated_at.clone(), String::new())
    }
}

pub fn ok(msg: impl AsRef<str>) {
    println!("  {}  {}", "●".bright_green(), msg.as_ref().bright_white());
}

pub fn info(msg: impl AsRef<str>) {
    println!("  {}  {}", "◆".bright_cyan(), msg.as_ref().bright_white());
}

pub fn err(msg: impl AsRef<str>) {
    eprintln!("  {}  {}", "✖".bright_red(), msg.as_ref().bright_white());
}

pub fn warn(msg: impl AsRef<str>) {
    println!("  {}  {}", "▲".bright_yellow(), msg.as_ref().bright_white());
}

pub fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else if max > 3 {
        format!("{}…", &s[..max - 1])
    } else {
        s[..max].to_string()
    }
}

pub fn list_table(store: &crate::utils::storage::MacroStore) {
    if store.macros.is_empty() {
        println!();
        warn("No macros stored.");
        return;
    }
    println!();
    println!(
        "  {:<20} {:<38} {:<8} {}",
        "KEY".bright_yellow().bold(),
        "COMMAND".bright_yellow().bold(),
        "DATE".bright_yellow().bold(),
        "TIME".bright_yellow().bold()
    );
    println!("  {}", "════════════════════════════════════════════════════════════════════════".truecolor(70, 70, 70));
    let mut keys: Vec<&String> = store.macros.keys().collect();
    keys.sort();
    for key in keys {
        let entry = &store.macros[key];
        let (date, time) = fmt_dt(entry);
        println!(
            "  {:<20} {:<38} {:<8} {}",
            key.as_str().bright_cyan(),
            truncate(&entry.command, 38).bright_white(),
            date.bright_yellow(),
            time.bright_yellow(),
        );
    }
    println!();
}
