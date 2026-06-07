use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroEntry {
    pub command: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroStore {
    pub macros: HashMap<String, MacroEntry>,
}

impl MacroStore {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }
}

pub fn storage_path() -> PathBuf {
    dirs::config_dir()
        .expect("Could not determine config directory")
        .join("velo")
        .join("macros.json")
}

pub fn timestamp_now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn load_macros() -> MacroStore {
    let path = storage_path();
    if !path.exists() {
        return MacroStore::new();
    }
    let content = std::fs::read_to_string(&path).unwrap_or_default();

    if let Ok(store) = serde_json::from_str::<MacroStore>(&content) {
        return store;
    }

    if let Ok(old) = serde_json::from_str::<HashMap<String, String>>(&content) {
        let now = timestamp_now();
        let macros = old
            .into_iter()
            .map(|(k, cmd)| {
                (
                    k,
                    MacroEntry {
                        command: cmd,
                        created_at: now.clone(),
                        updated_at: now.clone(),
                    },
                )
            })
            .collect();
        let store = MacroStore { macros };
        let _ = save_macros(&store);
        return store;
    }

    MacroStore::new()
}

pub fn save_macros(store: &MacroStore) -> Result<(), String> {
    let path = storage_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}", e))?;
    }
    let content =
        serde_json::to_string_pretty(store).map_err(|e| format!("{}", e))?;
    std::fs::write(&path, content).map_err(|e| format!("{}", e))?;
    Ok(())
}
