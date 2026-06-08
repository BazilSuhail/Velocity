use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

/// Everything the reducer reacts to.
#[derive(Debug, Clone, Copy)]
pub enum AppEvent {
    Key(KeyEvent),
    Resize,
}

/// Wait up to `timeout` for one terminal event. `Ok(None)` means nothing relevant arrived.
/// Windows reports key releases too, so only Press and Repeat are forwarded.
pub fn next(timeout: Duration) -> io::Result<Option<AppEvent>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }
    Ok(match event::read()? {
        Event::Key(k) if matches!(k.kind, KeyEventKind::Press | KeyEventKind::Repeat) => {
            Some(AppEvent::Key(k))
        }
        Event::Resize(..) => Some(AppEvent::Resize),
        _ => None,
    })
}
