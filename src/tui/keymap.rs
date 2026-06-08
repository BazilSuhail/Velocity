//! Single source of truth for keys: `resolve` maps key presses to actions, and the
//! `*_HINTS` tables drive the footer. Keep the two in step when adding a binding.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{AddStage, Mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Open,
    New,
    Edit,
    Delete,
    Filter,
    Help,
    Yes,
    No,
    Submit,
    Cancel,
    Backspace,
    ClearLine,
    Char(char),
}

pub const BROWSE_HINTS: &[(&str, &str)] = &[
    ("Up/Down", "move"),
    ("Enter", "actions"),
    ("n", "new"),
    ("e", "edit"),
    ("d", "delete"),
    ("/", "filter"),
    ("?", "help"),
    ("q", "quit"),
];
pub const ACTIONS_HINTS: &[(&str, &str)] =
    &[("Up/Down", "choose"), ("Enter", "select"), ("Esc", "back")];
pub const FILTER_HINTS: &[(&str, &str)] =
    &[("type", "search"), ("Enter", "keep"), ("Esc", "clear")];
pub const ADD_KEY_HINTS: &[(&str, &str)] =
    &[("Enter", "next"), ("Esc", "cancel"), ("Ctrl+U", "clear")];
pub const ADD_COMMAND_HINTS: &[(&str, &str)] =
    &[("Enter", "save"), ("Esc", "cancel"), ("Ctrl+U", "clear")];
pub const EDIT_HINTS: &[(&str, &str)] =
    &[("Enter", "save"), ("Esc", "cancel"), ("Ctrl+U", "clear")];
pub const CONFIRM_HINTS: &[(&str, &str)] = &[("y/Enter", "delete"), ("n/Esc", "cancel")];
pub const HELP_HINTS: &[(&str, &str)] = &[("?/Esc", "close")];

pub fn hints(mode: &Mode) -> &'static [(&'static str, &'static str)] {
    match mode {
        Mode::Browse => BROWSE_HINTS,
        Mode::Actions => ACTIONS_HINTS,
        Mode::Filter => FILTER_HINTS,
        Mode::Add(AddStage::Key) => ADD_KEY_HINTS,
        Mode::Add(AddStage::Command) => ADD_COMMAND_HINTS,
        Mode::Edit => EDIT_HINTS,
        Mode::ConfirmDelete => CONFIRM_HINTS,
        Mode::Help => HELP_HINTS,
    }
}

/// Map a key press to an action for the current mode. Ctrl+C quits from anywhere.
pub fn resolve(mode: &Mode, key: KeyEvent) -> Option<Action> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    if ctrl && key.code == KeyCode::Char('c') {
        return Some(Action::Quit);
    }
    match mode {
        Mode::Browse => browse(key.code, ctrl),
        Mode::Actions => actions(key.code),
        Mode::Help => match key.code {
            KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => Some(Action::Help),
            _ => None,
        },
        Mode::ConfirmDelete => match key.code {
            KeyCode::Char('y' | 'Y') | KeyCode::Enter => Some(Action::Yes),
            KeyCode::Char('n' | 'N') | KeyCode::Esc => Some(Action::No),
            _ => None,
        },
        Mode::Filter | Mode::Add(_) | Mode::Edit => text(key.code, ctrl),
    }
}

fn browse(code: KeyCode, ctrl: bool) -> Option<Action> {
    if ctrl {
        return None;
    }
    Some(match code {
        KeyCode::Up | KeyCode::Char('k') => Action::Up,
        KeyCode::Down | KeyCode::Char('j') => Action::Down,
        KeyCode::Home | KeyCode::Char('g') => Action::Home,
        KeyCode::End | KeyCode::Char('G') => Action::End,
        KeyCode::PageUp => Action::PageUp,
        KeyCode::PageDown => Action::PageDown,
        KeyCode::Enter => Action::Open,
        KeyCode::Esc => Action::Cancel,
        KeyCode::Char('n') => Action::New,
        KeyCode::Char('e') => Action::Edit,
        KeyCode::Char('d') => Action::Delete,
        KeyCode::Char('/') => Action::Filter,
        KeyCode::Char('?') => Action::Help,
        KeyCode::Char('q') => Action::Quit,
        _ => return None,
    })
}

/// The actions menu for the selected macro: arrows move, Enter picks, Esc or q backs out.
fn actions(code: KeyCode) -> Option<Action> {
    Some(match code {
        KeyCode::Up | KeyCode::Char('k') => Action::Up,
        KeyCode::Down | KeyCode::Char('j') => Action::Down,
        KeyCode::Enter => Action::Submit,
        KeyCode::Esc | KeyCode::Char('q') => Action::Cancel,
        _ => return None,
    })
}

/// Text-entry keys. The buffer is append-only (no cursor), so Ctrl+arrows are ignored.
fn text(code: KeyCode, ctrl: bool) -> Option<Action> {
    Some(match code {
        KeyCode::Enter => Action::Submit,
        KeyCode::Esc => Action::Cancel,
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Char('u') if ctrl => Action::ClearLine,
        KeyCode::Char(c) if !ctrl => Action::Char(c),
        _ => return None,
    })
}
