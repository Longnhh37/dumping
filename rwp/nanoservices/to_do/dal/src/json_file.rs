use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::HashMap,
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

type Result<T> = std::result::Result<T, NanoServiceError>;

fn store_path() -> PathBuf {
    env::var("JSON_STORE_PATH")
        .unwrap_or_else(|_| "./tasks.json".to_string())
        .into()
}

fn read_handle(path: &PathBuf) -> Result<Option<File>> {
    if !path.exists() {
        return Ok(None);
    }
    safe_eject!(
        File::open(path).map(Some),
        NanoServiceErrorStatus::Unknown,
        "Error opening file for reading"
    )
}

fn write_handle(path: &PathBuf) -> Result<File> {
    safe_eject!(
        OpenOptions::new().write(true).create(true).truncate(true).open(path),
        NanoServiceErrorStatus::Unknown,
        "Error opening JSON file for writing"
    )
}

pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>> {
    let Some(mut file) = read_handle(&store_path())? else {
        return Ok(HashMap::new());
    };
    let mut contents = String::new();
    safe_eject!(
        file.read_to_string(&mut contents),
        NanoServiceErrorStatus::Unknown,
        "Error reading file to get all tasks"
    )?;
    if contents.trim().is_empty() {
        return Ok(HashMap::new());
    }
    safe_eject!(
        serde_json::from_str(&contents),
        NanoServiceErrorStatus::Unknown,
        "Error parsing JSON file"
    )
}

pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<()> {
    let mut file = write_handle(&store_path())?;
    let json = safe_eject!(
        serde_json::to_string_pretty(tasks),
        NanoServiceErrorStatus::Unknown,
        "Error serializing JSON to save all tasks"
    )?;
    safe_eject!(
        file.write_all(json.as_bytes()),
        NanoServiceErrorStatus::Unknown,
        "Error writing file"
    )
}

pub fn get_one<T: DeserializeOwned>(id: &str) -> Result<T> {
    get_all::<T>()?
        .remove(id)
        .ok_or_else(|| NanoServiceError::new(
            format!("task '{}' not found", id),
            NanoServiceErrorStatus::NotFound,
        ))
}

pub fn save_one<T>(id: &str, task: &T) -> Result<()>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>()?;
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}

pub fn delete_one<T>(id: &str) -> Result<T>
where
    T: Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    let mut tasks = get_all::<T>().unwrap_or_default();
    let deleted_item = tasks.remove(id).ok_or_else(|| {
        NanoServiceError::new(format!("task with title '{}' not found", id), NanoServiceErrorStatus::NotFound)
    })?;
    save_all(&tasks)?;
    Ok(deleted_item)
}
