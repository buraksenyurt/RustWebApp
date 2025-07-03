use serde::Serialize;
use serde::de::DeserializeOwned;
use shared::errors::{ServiceError, ServiceErrorStatus};
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn get_reader() -> Result<File, ServiceError> {
    let file_path = env::var("STORAGE_PATH").unwrap_or("works.json".to_string());

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .map_err(|e| {
            ServiceError::new(
                ServiceErrorStatus::InternalServerError,
                format!("Error opening store file: {}", e),
            )
        })?;

    Ok(f)
}

fn get_writer() -> Result<File, ServiceError> {
    let file_path = env::var("STORAGE_PATH").unwrap_or("works.json".to_string());

    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .map_err(|e| {
            ServiceError::new(
                ServiceErrorStatus::InternalServerError,
                format!("Error opening store file: {}", e),
            )
        })?;

    Ok(f)
}

pub fn select_all<T>() -> Result<HashMap<String, T>, ServiceError>
where
    T: DeserializeOwned,
{
    let mut data_file = get_reader()?;
    let mut contents = String::new();
    data_file.read_to_string(&mut contents).map_err(|e| {
        ServiceError::new(
            ServiceErrorStatus::InternalServerError,
            format!("Error reading file: {}", e),
        )
    })?;
    let work_items: HashMap<String, T> = serde_json::from_str(&contents).map_err(|e| {
        ServiceError::new(
            ServiceErrorStatus::Unknown,
            format!("Serialization error: {}", e),
        )
    })?;
    Ok(work_items)
}

pub fn select_by_id<T>(key: &str) -> Result<T, ServiceError>
where
    T: DeserializeOwned + Clone,
{
    let work_items = select_all::<T>()?;
    match work_items.get(key) {
        Some(wi) => Ok(wi.clone()),
        None => Err(ServiceError::new(
            ServiceErrorStatus::NotFound,
            format!("Work item with key '{}' not found", key),
        )),
    }
}

pub async fn save_all<T>(work_items: &HashMap<String, T>) -> Result<(), ServiceError>
where
    T: Serialize,
{
    let mut data_file = get_writer()?;
    let json_data = serde_json::to_string_pretty(work_items).map_err(|e| {
        ServiceError::new(
            ServiceErrorStatus::InternalServerError,
            format!("Error on json serialization, {}", e),
        )
    })?;
    data_file.write_all(json_data.as_bytes()).map_err(|e| {
        ServiceError::new(
            ServiceErrorStatus::Unknown,
            format!("Error on data save operations, {}", e),
        )
    })?;
    Ok(())
}

pub async fn save_single<T>(key: &str, work_item: &T) -> Result<(), ServiceError>
where
    T: Serialize + DeserializeOwned + Clone + Debug,
{
    let mut work_items = select_all::<T>().unwrap_or_else(|_| HashMap::new());
    work_items.insert(key.to_string(), work_item.clone());
    Ok(save_all(&work_items).await?)
}

pub async fn delete_single<T>(key: &str) -> Result<(), ServiceError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut work_items = select_all::<T>().unwrap_or_default();
    match work_items.remove(key) {
        Some(_) => {
            save_all(&work_items).await?;
            Ok(())
        }
        None => Err(ServiceError::new(
            ServiceErrorStatus::NotFound,
            format!("Work item with key '{}' not found", key),
        )),
    }
}
