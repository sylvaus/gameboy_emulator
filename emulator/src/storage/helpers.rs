use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use crate::storage::interface::{LoadError, StoreError};

pub fn store_to_string(value: &impl Serialize) -> Result<String, StoreError> {
    serde_json::to_string(value)
        .map_err(|e| StoreError::new(e.to_string()))
}

pub fn store_to_file(value: &impl Serialize, path: &Path) -> Result<(), StoreError> {
    let file = File::create(path)
        .map_err(|err| StoreError::new(format!("Failed to open file: {}", err)) )?;

    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, value)
        .map_err(|err| StoreError::new(format!("Failed to serialize to file: {}", err)))
}

pub fn load_from_str<'a, T: Deserialize<'a>>(string: &'a str) -> Result<T, LoadError> {
    serde_json::from_str::<T>(string)
        .map_err(|e| LoadError::new(e.to_string()))
}

pub fn load_from_file<'a, T: DeserializeOwned>(path: &Path) -> Result<T, LoadError> {
    let file = File::open(path)
        .map_err(|err| LoadError::new(format!("Failed to open file: {}", err)))?;

    serde_json::from_reader(file)
        .map_err(|e| LoadError::new(e.to_string()))
}

