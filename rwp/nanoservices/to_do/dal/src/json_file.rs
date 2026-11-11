use std::{
    collections::HashMap,
    env, fs,
    fs::{File, OpenOptions},
    io::Read,
    path::PathBuf,
};

use serde::{Serialize, de::DeserializeOwned};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

type Result<T> = std::result::Result<T, NanoServiceError>;

fn store_path(path: Option<&str>) -> PathBuf {
    match path {
        Some(p) => PathBuf::from(p),
        None => env::var_os("JSON_STORE_PATH")
            .map_or_else(|| PathBuf::from("./tasks.json"), PathBuf::from),
    }
}

fn get_handle(path: Option<&str>) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(store_path(path))
        .eject_ctx(NanoServiceErrorStatus::Unknown, "Error opening JSON file")
}

pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>> {
    let mut contents = String::new();
    get_handle(None)?.read_to_string(&mut contents).eject_ctx(
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file to get all tasks",
    )?;
    serde_json::from_str(&contents)
        .eject_ctx(NanoServiceErrorStatus::Unknown, "Error parsing JSON file")
}

pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks).eject_ctx(
        NanoServiceErrorStatus::Unknown,
        "Error serializing JSON before saving tasks",
    )?;
    fs::write(store_path(None), json).eject_ctx(
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON file",
    )
}

pub fn get_one<T: DeserializeOwned>(id: &str) -> Result<T> {
    get_all::<T>()?.remove(id).ok_or_else(|| {
        NanoServiceError::new(
            format!("Task with id {id} not found"),
            NanoServiceErrorStatus::Unknown,
        )
    })
}

pub fn save_one<T>(id: &str, task: &T) -> Result<()>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_default();
    tasks.insert(id.to_owned(), task.clone());
    save_all(&tasks)
}

pub fn delete_one<T>(id: &str) -> Result<T>
where
    T: Serialize + DeserializeOwned,
{
    let mut tasks = get_all::<T>().unwrap_or_default();
    let deleted = tasks.remove(id).ok_or_else(|| {
        NanoServiceError::new(
            format!("Task with title {id} not found"),
            NanoServiceErrorStatus::NotFound,
        )
    })?;
    save_all(&tasks)?;
    Ok(deleted)
}

#[cfg(feature = "json-file")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_handle_creates_missing_file() {
        let path = "./non_existent_file.json";
        assert!(get_handle(Some(path)).is_ok());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn save_then_get_one() {
        save_one("1", &"Task 1".to_string()).unwrap();
        assert_eq!(get_one::<String>("1").unwrap(), "Task 1");
    }
}
