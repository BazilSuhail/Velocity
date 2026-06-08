use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroEntry {
    pub command: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MacroStore {
    pub macros: BTreeMap<String, MacroEntry>,
}

impl MacroStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub enum StoreError {
    NoConfigDir,
    Io(std::io::Error),
    Corrupt { path: PathBuf },
    Serialize(serde_json::Error),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::NoConfigDir => write!(
                f,
                "could not find a config directory; set VELO_HOME to choose where macros are stored"
            ),
            StoreError::Io(e) => write!(f, "could not access macros file: {e}"),
            StoreError::Corrupt { path } => write!(
                f,
                "macros file {} is corrupt and was NOT modified; fix or remove it, then retry",
                path.display()
            ),
            StoreError::Serialize(e) => write!(f, "could not serialize macros: {e}"),
        }
    }
}

impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StoreError::Io(e) => Some(e),
            StoreError::Serialize(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        StoreError::Io(e)
    }
}

pub fn storage_path() -> Result<PathBuf, StoreError> {
    let dir = match std::env::var_os("VELO_HOME").filter(|v| !v.is_empty()) {
        Some(home) => PathBuf::from(home),
        None => dirs::config_dir()
            .ok_or(StoreError::NoConfigDir)?
            .join("velo"),
    };
    Ok(dir.join("macros.json"))
}

pub fn timestamp_now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn load() -> Result<MacroStore, StoreError> {
    let path = storage_path()?;
    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(MacroStore::new()),
        Err(e) => return Err(StoreError::Io(e)),
    };

    if let Ok(store) = serde_json::from_str::<MacroStore>(&content) {
        return Ok(store);
    }

    // Legacy format: plain name -> command map. Converted in memory; the file
    // is rewritten in the new format on the next save.
    if let Ok(old) = serde_json::from_str::<HashMap<String, String>>(&content) {
        let now = timestamp_now();
        let macros = old
            .into_iter()
            .map(|(name, command)| {
                let entry = MacroEntry {
                    command,
                    created_at: now.clone(),
                    updated_at: now.clone(),
                };
                (name, entry)
            })
            .collect();
        return Ok(MacroStore { macros });
    }

    // Never fall back to an empty store here: the caller would then overwrite the user's data.
    Err(StoreError::Corrupt { path })
}

pub fn save(store: &MacroStore) -> Result<(), StoreError> {
    let path = storage_path()?;
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;
    let content = serde_json::to_string_pretty(store).map_err(StoreError::Serialize)?;

    // Write a temp file and rename it over the target so a crash never leaves
    // a half-written macros.json. rename replaces the target on Windows too.
    // Per-process name so two velo processes saving at once cannot share one temp file.
    let tmp = path.with_file_name(format!("macros.json.{}.tmp", std::process::id()));
    let mut file = std::fs::File::create(&tmp)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    drop(file);
    std::fs::rename(&tmp, &path)?;
    Ok(())
}

pub fn validate_key(key: &str) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err("macro name cannot be empty".to_string());
    }
    if key.chars().any(char::is_whitespace) {
        return Err(format!("macro name '{key}' cannot contain spaces"));
    }
    if let Some(c) = key
        .chars()
        .find(|c| "/\\\"'".contains(*c) || c.is_control())
    {
        return Err(format!(
            "macro name '{key}' contains invalid character {c:?}"
        ));
    }
    Ok(())
}
