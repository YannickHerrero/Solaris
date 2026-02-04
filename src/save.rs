use std::fs;
use std::io;
use std::path::PathBuf;

use directories::ProjectDirs;

use crate::app::SaveData;

pub fn get_save_path() -> io::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "solaris")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find data directory"))?;

    let data_dir = proj_dirs.data_local_dir();
    fs::create_dir_all(data_dir)?;

    Ok(data_dir.join("save.json"))
}

pub fn save_game(save_data: &SaveData) -> io::Result<()> {
    let path = get_save_path()?;
    let json = serde_json::to_string_pretty(save_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, json)
}

pub fn load_game() -> io::Result<Option<SaveData>> {
    let path = get_save_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(path)?;
    let save_data: SaveData =
        serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(Some(save_data))
}

pub fn delete_save() -> io::Result<bool> {
    let path = get_save_path()?;

    if !path.exists() {
        return Ok(false);
    }

    fs::remove_file(path)?;
    Ok(true)
}
