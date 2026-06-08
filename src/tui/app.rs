//! App state and the reducer. Every write goes through `mutate`, which re-reads the file,
//! applies one change, and saves. The in-memory store only changes after a successful save.

use std::collections::BTreeMap;
use std::io;

use crate::tui::event::AppEvent;
use crate::tui::keymap::{self, Action};
use crate::utils::storage::{
    self, MacroEntry, MacroStore, StoreError, timestamp_now, validate_key,
};

/// Entries of the actions menu, in display order. Indexes match the constants below.
pub const ACTIONS: [&str; 4] = ["Run", "Update", "Delete", "Back"];
const RUN: usize = 0;
const UPDATE: usize = 1;
const DELETE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Browse,
    Actions,
    Filter,
    Add(AddStage),
    Edit,
    ConfirmDelete,
    Help,
}

/// Add flows Key -> Command; `App::input` holds the text of the current stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddStage {
    Key,
    Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    Ok,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub kind: StatusKind,
    pub text: String,
}

pub struct App {
    pub store: MacroStore,
    pub mode: Mode,
    pub query: String,
    pub selected: usize,
    /// Highlighted row of the actions menu.
    pub action: usize,
    pub input: String,
    pub draft_key: String,
    pub status: Option<Status>,
    pub dirty: bool,
    pub should_quit: bool,
    /// Command the loop must run outside the alternate screen.
    pub pending_run: Option<String>,
    /// Set when the store failed to load. Writes are refused while it is set.
    pub load_error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            store: MacroStore {
                macros: BTreeMap::new(),
            },
            mode: Mode::Browse,
            query: String::new(),
            selected: 0,
            action: 0,
            input: String::new(),
            draft_key: String::new(),
            status: None,
            dirty: true,
            should_quit: false,
            pending_run: None,
            load_error: None,
        }
    }

    /// Keys matching the query (case-insensitive substring on key or command), in sorted order.
    pub fn visible_keys(&self) -> Vec<String> {
        let q = self.query.to_lowercase();
        self.store
            .macros
            .iter()
            .filter(|(k, e)| {
                q.is_empty()
                    || k.to_lowercase().contains(&q)
                    || e.command.to_lowercase().contains(&q)
            })
            .map(|(k, _)| k.clone())
            .collect()
    }

    pub fn selected_key(&self) -> Option<String> {
        let keys = self.visible_keys();
        keys.get(self.selected.min(keys.len().saturating_sub(1)))
            .cloned()
    }

    /// Install a freshly loaded store, or record why loading failed.
    pub fn replace_store(&mut self, res: Result<MacroStore, StoreError>) {
        match res {
            Ok(store) => {
                self.store = store;
                self.load_error = None;
            }
            Err(e) => self.load_error = Some(e.to_string()),
        }
        self.dirty = true;
    }

    /// Record the outcome of a macro run executed by the loop.
    pub fn record_run(&mut self, result: io::Result<Option<i32>>) {
        let status = match result {
            Ok(Some(0)) => Status {
                kind: StatusKind::Ok,
                text: "ran successfully (exit 0)".to_string(),
            },
            Ok(Some(code)) => Status {
                kind: StatusKind::Error,
                text: format!("exited with code {code}"),
            },
            Ok(None) => Status {
                kind: StatusKind::Error,
                text: "terminated without an exit code".to_string(),
            },
            Err(e) => Status {
                kind: StatusKind::Error,
                text: format!("failed to run: {e}"),
            },
        };
        self.status = Some(status);
        self.dirty = true;
    }

    /// The reducer: apply one event to the state.
    pub fn on_event(app: &mut App, ev: AppEvent) {
        match ev {
            AppEvent::Resize => app.dirty = true,
            AppEvent::Key(key) => {
                app.dirty = true;
                if let Some(action) = keymap::resolve(&app.mode, key) {
                    app.apply(action);
                }
            }
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Up => self.step(-1),
            Action::Down => self.step(1),
            Action::PageUp => self.step(-10),
            Action::PageDown => self.step(10),
            Action::Home => self.selected = 0,
            Action::End => self.selected = self.visible_keys().len().saturating_sub(1),
            Action::Open => self.open_menu(),
            Action::New => {
                if self.write_allowed() {
                    self.input.clear();
                    self.draft_key.clear();
                    self.mode = Mode::Add(AddStage::Key);
                }
            }
            Action::Edit => self.begin_edit(),
            Action::Delete => self.request_delete(),
            Action::Filter => self.mode = Mode::Filter,
            Action::Help => {
                self.mode = if self.mode == Mode::Help {
                    Mode::Browse
                } else {
                    Mode::Help
                };
            }
            Action::Yes => self.confirm_delete(),
            Action::No => self.mode = Mode::Browse,
            Action::Submit => self.submit(),
            Action::Cancel => self.cancel(),
            Action::Backspace => {
                if let Some(buf) = self.text_target() {
                    buf.pop();
                }
                self.after_text_edit();
            }
            Action::ClearLine => {
                if let Some(buf) = self.text_target() {
                    buf.clear();
                }
                self.after_text_edit();
            }
            Action::Char(c) => {
                if let Some(buf) = self.text_target() {
                    buf.push(c);
                }
                self.after_text_edit();
            }
        }
    }

    /// Arrow keys move the menu highlight in the actions menu and the selection elsewhere.
    fn step(&mut self, delta: isize) {
        if self.mode == Mode::Actions {
            let last = ACTIONS.len() as isize - 1;
            self.action = (self.action as isize + delta).clamp(0, last) as usize;
        } else {
            self.move_by(delta);
        }
    }

    fn move_by(&mut self, delta: isize) {
        let len = self.visible_keys().len();
        if len == 0 {
            self.selected = 0;
            return;
        }
        let cur = self.selected.min(len - 1) as isize;
        self.selected = (cur + delta).clamp(0, len as isize - 1) as usize;
    }

    fn open_menu(&mut self) {
        if self.selected_key().is_some() {
            self.action = 0;
            self.mode = Mode::Actions;
        }
    }

    /// Run the highlighted menu entry. Every entry leaves the menu first.
    fn run_action(&mut self) {
        match self.action {
            RUN => {
                self.mode = Mode::Browse;
                self.pending_run = self
                    .selected_key()
                    .and_then(|k| self.store.macros.get(&k))
                    .map(|e| e.command.clone());
            }
            UPDATE => self.begin_edit(),
            DELETE => self.request_delete(),
            _ => self.mode = Mode::Browse,
        }
    }

    /// The buffer that text keys edit in the current mode.
    fn text_target(&mut self) -> Option<&mut String> {
        match self.mode {
            Mode::Filter => Some(&mut self.query),
            Mode::Add(_) | Mode::Edit => Some(&mut self.input),
            _ => None,
        }
    }

    fn after_text_edit(&mut self) {
        if self.mode == Mode::Filter {
            self.selected = 0;
        }
    }

    fn begin_edit(&mut self) {
        if !self.write_allowed() {
            return;
        }
        let Some(key) = self.selected_key() else {
            return;
        };
        let Some(entry) = self.store.macros.get(&key) else {
            return;
        };
        self.input = entry.command.clone();
        self.draft_key = key;
        self.mode = Mode::Edit;
    }

    fn request_delete(&mut self) {
        if self.write_allowed() && self.selected_key().is_some() {
            self.mode = Mode::ConfirmDelete;
        }
    }

    fn submit(&mut self) {
        match self.mode {
            Mode::Actions => self.run_action(),
            Mode::Add(AddStage::Key) => {
                let key = self.input.trim().to_string();
                if let Err(msg) = validate_key(&key) {
                    self.set_status(StatusKind::Error, msg);
                } else if self.store.macros.contains_key(&key) {
                    self.set_status(StatusKind::Error, format!("'{key}' already exists"));
                } else {
                    self.draft_key = key;
                    self.input.clear();
                    self.mode = Mode::Add(AddStage::Command);
                }
            }
            Mode::Add(AddStage::Command) => {
                let Some(command) = self.trimmed_input() else {
                    return;
                };
                let key = self.draft_key.clone();
                let saved = self.mutate(move |store| {
                    if store.macros.contains_key(&key) {
                        return Err(format!("'{key}' already exists"));
                    }
                    let now = timestamp_now();
                    store.macros.insert(
                        key.clone(),
                        MacroEntry {
                            command,
                            created_at: now.clone(),
                            updated_at: now,
                        },
                    );
                    Ok(format!("added '{key}'"))
                });
                if saved {
                    self.selected = self
                        .visible_keys()
                        .iter()
                        .position(|k| *k == self.draft_key)
                        .unwrap_or(0);
                    self.leave_input();
                }
            }
            Mode::Edit => {
                let Some(command) = self.trimmed_input() else {
                    return;
                };
                let key = self.draft_key.clone();
                let saved = self.mutate(move |store| {
                    let Some(entry) = store.macros.get_mut(&key) else {
                        return Err(format!("'{key}' no longer exists"));
                    };
                    entry.command = command;
                    entry.updated_at = timestamp_now();
                    Ok(format!("updated '{key}'"))
                });
                if saved {
                    self.leave_input();
                }
            }
            Mode::Filter => self.mode = Mode::Browse,
            Mode::Browse | Mode::ConfirmDelete | Mode::Help => {}
        }
    }

    fn cancel(&mut self) {
        match self.mode {
            Mode::Filter => {
                self.query.clear();
                self.selected = 0;
                self.mode = Mode::Browse;
            }
            Mode::Add(_) | Mode::Edit => self.leave_input(),
            Mode::ConfirmDelete | Mode::Help | Mode::Actions => self.mode = Mode::Browse,
            Mode::Browse => {
                if !self.query.is_empty() {
                    self.query.clear();
                    self.selected = 0;
                }
            }
        }
    }

    fn confirm_delete(&mut self) {
        self.mode = Mode::Browse;
        let Some(key) = self.selected_key() else {
            return;
        };
        self.mutate(move |store| {
            if store.macros.remove(&key).is_none() {
                return Err(format!("'{key}' no longer exists"));
            }
            Ok(format!("deleted '{key}'"))
        });
    }

    fn leave_input(&mut self) {
        self.input.clear();
        self.draft_key.clear();
        self.mode = Mode::Browse;
    }

    fn trimmed_input(&mut self) -> Option<String> {
        let text = self.input.trim().to_string();
        if text.is_empty() {
            self.set_status(StatusKind::Error, "command cannot be empty");
            None
        } else {
            Some(text)
        }
    }

    /// Refuse writes while the store is unavailable, explaining why.
    fn write_allowed(&mut self) -> bool {
        if self.load_error.is_some() {
            self.set_status(
                StatusKind::Error,
                "store failed to load; writes are disabled",
            );
            false
        } else {
            true
        }
    }

    /// Re-read the file, apply `change`, and save. Re-reading means a change made in another
    /// shell since startup is kept rather than overwritten. Returns true when the save succeeded.
    fn mutate(&mut self, change: impl FnOnce(&mut MacroStore) -> Result<String, String>) -> bool {
        if !self.write_allowed() {
            return false;
        }
        let mut next = match storage::load() {
            Ok(store) => store,
            Err(e) => {
                self.set_status(StatusKind::Error, e.to_string());
                return false;
            }
        };
        let ok_text = match change(&mut next) {
            Ok(text) => text,
            Err(msg) => {
                self.set_status(StatusKind::Error, msg);
                return false;
            }
        };
        match storage::save(&next) {
            Ok(()) => {
                self.store = next;
                self.set_status(StatusKind::Ok, ok_text);
                true
            }
            Err(e) => {
                self.set_status(StatusKind::Error, format!("save failed: {e}"));
                false
            }
        }
    }

    fn set_status(&mut self, kind: StatusKind, text: impl Into<String>) {
        self.status = Some(Status {
            kind,
            text: text.into(),
        });
        self.dirty = true;
    }
}
