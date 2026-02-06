use std::fs;
use std::io;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::app::SaveData;

/// Metadata tracking which save was last used
#[derive(Serialize, Deserialize, Default)]
pub struct SaveMeta {
    pub last_used: Option<String>,
}

/// Information about a save file
#[derive(Debug, Clone)]
pub struct SaveInfo {
    pub label: String,
}

/// Get the base data directory for solaris
fn get_data_dir() -> io::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "solaris")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find data directory"))?;

    let data_dir = proj_dirs.data_local_dir().to_path_buf();
    fs::create_dir_all(&data_dir)?;

    Ok(data_dir)
}

/// Get the saves directory
fn get_saves_dir() -> io::Result<PathBuf> {
    let saves_dir = get_data_dir()?.join("saves");
    fs::create_dir_all(&saves_dir)?;
    Ok(saves_dir)
}

/// Get the path to the meta file
fn get_meta_path() -> io::Result<PathBuf> {
    Ok(get_data_dir()?.join("meta.json"))
}

/// Get the legacy save path (for migration)
pub fn get_legacy_save_path() -> io::Result<PathBuf> {
    Ok(get_data_dir()?.join("save.json"))
}

/// Sanitize a label for use as a filename
/// Allows alphanumeric, hyphens, and underscores
pub fn sanitize_label(label: &str) -> String {
    label
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else if c.is_whitespace() {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches(|c| c == '-' || c == '_')
        .to_string()
}

/// Validate a label
pub fn validate_label(label: &str) -> Result<String, String> {
    let sanitized = sanitize_label(label);
    if sanitized.is_empty() {
        return Err("Label cannot be empty".to_string());
    }
    if sanitized.len() > 50 {
        return Err("Label is too long (max 50 characters)".to_string());
    }
    Ok(sanitized)
}

/// Get the path to a save file by label
pub fn get_save_path(label: &str) -> io::Result<PathBuf> {
    let sanitized = sanitize_label(label);
    Ok(get_saves_dir()?.join(format!("{}.json", sanitized)))
}

/// Load the save metadata
pub fn load_meta() -> io::Result<SaveMeta> {
    let path = get_meta_path()?;
    if !path.exists() {
        return Ok(SaveMeta::default());
    }

    let json = fs::read_to_string(&path)?;
    serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Save the metadata
pub fn save_meta(meta: &SaveMeta) -> io::Result<()> {
    let path = get_meta_path()?;
    let json = serde_json::to_string_pretty(meta)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, json)
}

/// Update the last used save label
pub fn set_last_used(label: &str) -> io::Result<()> {
    let mut meta = load_meta()?;
    meta.last_used = Some(label.to_string());
    save_meta(&meta)
}

/// Get the last used save label
pub fn get_last_used() -> io::Result<Option<String>> {
    let meta = load_meta()?;
    Ok(meta.last_used)
}

/// List all available saves
pub fn list_saves() -> io::Result<Vec<SaveInfo>> {
    let saves_dir = get_saves_dir()?;
    let mut saves = Vec::new();

    if saves_dir.exists() {
        for entry in fs::read_dir(&saves_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(stem) = path.file_stem() {
                    if let Some(label) = stem.to_str() {
                        saves.push(SaveInfo {
                            label: label.to_string(),
                        });
                    }
                }
            }
        }
    }

    // Sort by label
    saves.sort_by(|a, b| a.label.cmp(&b.label));

    Ok(saves)
}

/// Check if a save exists
pub fn save_exists(label: &str) -> io::Result<bool> {
    let path = get_save_path(label)?;
    Ok(path.exists())
}

/// Save the game to a specific save slot
pub fn save_game(label: &str, save_data: &SaveData) -> io::Result<()> {
    let path = get_save_path(label)?;
    let json = serde_json::to_string_pretty(save_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, json)?;

    // Update last used
    set_last_used(label)?;

    Ok(())
}

/// Load the game from a specific save slot
pub fn load_game(label: &str) -> io::Result<Option<SaveData>> {
    let path = get_save_path(label)?;

    if !path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(path)?;
    let save_data: SaveData =
        serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Update last used
    set_last_used(label)?;

    Ok(Some(save_data))
}

/// Delete a save
pub fn delete_save(label: &str) -> io::Result<bool> {
    let path = get_save_path(label)?;

    if !path.exists() {
        return Ok(false);
    }

    fs::remove_file(path)?;

    // If this was the last used save, clear it from meta
    let meta = load_meta()?;
    if meta.last_used.as_deref() == Some(label) {
        let mut new_meta = meta;
        new_meta.last_used = None;
        save_meta(&new_meta)?;
    }

    Ok(true)
}

/// Migrate legacy save.json to the new saves system
/// Returns the label of the migrated save if migration occurred
pub fn migrate_legacy_save() -> io::Result<Option<String>> {
    let legacy_path = get_legacy_save_path()?;

    if !legacy_path.exists() {
        return Ok(None);
    }

    // Read the legacy save
    let json = fs::read_to_string(&legacy_path)?;
    let save_data: SaveData =
        serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Ensure saves directory exists
    get_saves_dir()?;

    // Write to the new location with label "main"
    let label = "main";
    save_game(label, &save_data)?;

    // Remove the legacy save
    fs::remove_file(&legacy_path)?;

    Ok(Some(label.to_string()))
}

/// Determine the save label to use
/// Priority: explicit label > last used > "main" (if exists) > None
pub fn resolve_save_label(explicit: Option<&str>) -> io::Result<Option<String>> {
    // If explicit label provided, use it
    if let Some(label) = explicit {
        return Ok(Some(label.to_string()));
    }

    // Check for last used
    if let Some(last) = get_last_used()? {
        if save_exists(&last)? {
            return Ok(Some(last));
        }
    }

    // Check if "main" exists
    if save_exists("main")? {
        return Ok(Some("main".to_string()));
    }

    // No saves exist
    Ok(None)
}
