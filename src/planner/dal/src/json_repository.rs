use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn get_store() -> Result<File, String> {
    let file_path = env::var("STORAGE_PATH").unwrap_or("works.json".to_string());

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .map_err(|e| e.to_string())?;

    Ok(f)
}

pub fn get_all<T>() -> Result<HashMap<String, T>, String>
where
    T: DeserializeOwned,
{
    let mut data_file = get_store()?;
    let mut contents = String::new();
    data_file
        .read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    let work_items: HashMap<String, T> =
        serde_json::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(work_items)
}

pub fn get_by_id<T>(key: &str) -> Result<T, String>
where
    T: DeserializeOwned + Clone,
{
    let work_items = get_all::<T>()?;
    match work_items.get(key) {
        Some(wi) => Ok(wi.clone()),
        None => Err(format!("Work item with key '{}' not found", key)),
    }
}

pub fn save_all<T>(work_items: &HashMap<String, T>) -> Result<(), String>
where
    T: Serialize,
{
    let mut data_file = get_store()?;
    let json_data = serde_json::to_string_pretty(work_items).map_err(|e| e.to_string())?;
    data_file
        .write_all(json_data.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn save_single<T>(key: &str, work_item: &T) -> Result<(), String>
where
    T: Serialize + DeserializeOwned + Clone + Debug,
{
    let mut work_items = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    work_items.insert(key.to_string(), work_item.clone());
    save_all(&work_items)
}

pub fn delete<T>(key: &str) -> Result<(), String>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut work_items = get_all::<T>().unwrap_or_default();
    work_items.remove(key);
    save_all(&work_items)
}
