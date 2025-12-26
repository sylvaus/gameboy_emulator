use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StorageVersion {
    Version1,
}

#[derive(Debug)]
pub struct StoreError {
    pub message: String,
}

impl StoreError {
    pub fn new(message: String) -> Self {
        Self {message}
    }
}

#[derive(Debug)]
pub struct LoadError {
    pub message: String,
}

impl LoadError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub trait SaveState {
    fn save(&self) -> Result<String, StoreError>;

    fn load(&mut self, state: &str, version: StorageVersion) -> Result<(), LoadError>;
}