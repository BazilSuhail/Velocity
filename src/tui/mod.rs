pub mod app;
pub mod event;
pub mod keymap;
pub mod theme;
pub mod ui;

use std::io::{self, Write};
use std::time::Duration;

use ratatui::DefaultTerminal;

use crate::platform;
use crate::utils::storage;
use app::App;

/// How long the loop waits for input before checking again. Key presses wake it at once,
/// so this only bounds idle wake-ups, and nothing redraws unless the state changed.
const POLL: Duration = Duration::from_millis(500);

/// Owns the terminal. Dropping it restores the screen, so every exit path (including `?`) cleans up.
struct Session {
    term: DefaultTerminal,
}

impl Drop for Session {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

/// Run the interactive TUI until the user quits.
pub fn run() -> io::Result<()> {
    let term = ratatui::try_init().inspect_err(|_| ratatui::restore())?;
    let mut session = Session { term };

    let mut app = App::new();
    app.replace_store(storage::load());

    loop {
        if app.dirty {
            session.term.draw(|f| ui::draw(f, &app))?;
            app.dirty = false;
        }
        if app.should_quit {
            return Ok(());
        }
        if let Some(cmd) = app.pending_run.take() {
            run_outside(&mut session, &mut app, &cmd)?;
            continue;
        }
        if let Some(ev) = event::next(POLL)? {
            App::on_event(&mut app, ev);
        }
    }
}

/// Leave the TUI, run the macro in the normal console, then re-enter the TUI.
fn run_outside(session: &mut Session, app: &mut App, cmd: &str) -> io::Result<()> {
    ratatui::restore();
    println!();
    println!("> {cmd}");
    let result = platform::run_macro(cmd);
    println!();
    print!("Press Enter to return to velo...");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    session.term = ratatui::try_init()?;
    app.record_run(result);
    app.replace_store(storage::load());
    Ok(())
}
