use std::io::Write;

use chrono::{Datelike, Timelike};
use colored::*;

use crate::platform;
use crate::utils::storage::{MacroEntry, MacroStore};

/// Picks the Unicode glyph, or its ASCII fallback on terminals that cannot show it.
fn mark(unicode: &'static str, ascii: &'static str) -> &'static str {
    if platform::unicode_supported() {
        unicode
    } else {
        ascii
    }
}

/// Splits `updated_at` into a short date and time; unparseable values are shown raw.
pub fn fmt_dt(entry: &MacroEntry) -> (String, String) {
    match chrono::NaiveDateTime::parse_from_str(&entry.updated_at, "%Y-%m-%d %H:%M:%S") {
        Ok(dt) => (
            format!("{}/{}/{}", dt.month(), dt.day(), dt.year() % 100),
            format!("{}:{:02}", dt.hour(), dt.minute()),
        ),
        Err(_) => (entry.updated_at.clone(), String::new()),
    }
}

pub fn ok(msg: impl AsRef<str>) {
    println!(
        "  {}  {}",
        mark("●", "*").bright_green(),
        msg.as_ref().bright_white()
    );
}

pub fn info(msg: impl AsRef<str>) {
    println!(
        "  {}  {}",
        mark("◆", "-").bright_cyan(),
        msg.as_ref().bright_white()
    );
}

pub fn warn(msg: impl AsRef<str>) {
    println!(
        "  {}  {}",
        mark("▲", "!").bright_yellow(),
        msg.as_ref().bright_white()
    );
}

pub fn err(msg: impl AsRef<str>) {
    eprintln!(
        "  {}  {}",
        mark("✖", "x").bright_red(),
        msg.as_ref().bright_white()
    );
}

pub fn hint(msg: impl AsRef<str>) {
    println!(
        "  {}  {}",
        mark("›", ">").bright_black(),
        msg.as_ref().bright_black()
    );
}

/// Prints `question` and reads one trimmed line. Returns "" on EOF, so callers never loop.
pub fn prompt(question: &str) -> String {
    print!(
        "  {}  {} ",
        mark("▸", ">").bright_cyan(),
        question.bright_white()
    );
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

/// Shortens `s` to at most `max` characters (not bytes), ending with an ellipsis.
pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let dots = mark("…", "...");
    let dots_len = dots.chars().count();
    if max <= dots_len {
        return s.chars().take(max).collect();
    }
    let mut out: String = s.chars().take(max - dots_len).collect();
    out.push_str(dots);
    out
}

pub fn list_table(store: &MacroStore) {
    println!();
    if store.macros.is_empty() {
        warn("No macros stored yet.");
        hint("Add one with: velo add <KEY> <COMMAND>");
        println!();
        return;
    }

    let rule = mark("═", "=").repeat(72);
    println!(
        "  {}  {}  {}",
        format!("{:<20}", "KEY").bright_yellow().bold(),
        format!("{:<38}", "COMMAND").bright_yellow().bold(),
        "UPDATED".bright_yellow().bold(),
    );
    println!("  {}", rule.truecolor(70, 70, 70));
    for (key, entry) in &store.macros {
        let (date, time) = fmt_dt(entry);
        println!(
            "  {}  {}  {} {}",
            format!("{:<20}", truncate(key, 20)).bright_cyan(),
            format!("{:<38}", truncate(&entry.command, 38)).bright_white(),
            date.bright_yellow(),
            time.bright_yellow(),
        );
    }
    println!();
}
