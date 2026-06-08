//! Platform and terminal capability detection, plus shell execution.

use std::env;
use std::io::{self, IsTerminal};
use std::process::Command;

/// Builds a command that runs `command` through the platform shell.
pub fn shell_command(command: &str) -> Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let mut cmd = Command::new("cmd");
        // raw_arg: cmd.exe does not understand the \" escaping that `arg` applies to quotes.
        cmd.arg("/C").raw_arg(command);
        cmd
    }
    #[cfg(not(windows))]
    {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(command);
        cmd
    }
}

/// Runs `command` with inherited stdio. Returns the exit code, or `None` if killed by a signal.
pub fn run_macro(command: &str) -> io::Result<Option<i32>> {
    shell_command(command).status().map(|status| status.code())
}

/// Colour is off for NO_COLOR, TERM=dumb, or a non-terminal stdout; CLICOLOR_FORCE=1 overrides the last two.
pub fn color_enabled() -> bool {
    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if env::var("CLICOLOR_FORCE").is_ok_and(|v| v == "1") {
        return true;
    }
    if env::var("TERM").is_ok_and(|t| t == "dumb") {
        return false;
    }
    io::stdout().is_terminal()
}

/// True when COLORTERM advertises 24-bit colour, or when running in Windows Terminal.
pub fn truecolor_supported() -> bool {
    let colorterm = env::var("COLORTERM").unwrap_or_default().to_lowercase();
    colorterm.contains("truecolor")
        || colorterm.contains("24bit")
        || env::var_os("WT_SESSION").is_some()
}

/// Rule: VELO_ASCII forces ASCII. On Windows, Unicode needs Windows Terminal or a
/// TERM_PROGRAM host, since legacy ConHost renders it poorly. On other platforms,
/// the first non-empty of LC_ALL, LC_CTYPE, LANG must name UTF-8.
pub fn unicode_supported() -> bool {
    if env::var_os("VELO_ASCII").is_some() {
        return false;
    }
    if cfg!(windows) {
        return env::var_os("WT_SESSION").is_some() || env::var_os("TERM_PROGRAM").is_some();
    }
    ["LC_ALL", "LC_CTYPE", "LANG"]
        .iter()
        .find_map(|k| env::var(k).ok().filter(|v| !v.is_empty()))
        .is_some_and(|v| {
            let v = v.to_lowercase();
            v.contains("utf-8") || v.contains("utf8")
        })
}

/// True when both stdin and stdout are terminals.
pub fn is_interactive() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}
