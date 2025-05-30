use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Manager {
    pub add: String,
    pub remove: String,
    pub file: String,
}

pub struct Managers;

impl Managers {
    fn load() -> HashMap<String, Vec<Manager>> {
        let data = fs::read_to_string("managers.json").unwrap();
        serde_json::from_str(&data).unwrap()
    }

    pub fn detect(lang: &str) -> Option<Manager> {
        let manager = Self::load()
            .get(lang)?
            .iter()
            .find(|m| fs::metadata(&m.file).is_ok())
            .cloned();

        if let Some(manager) = manager {
            return Some(manager);
        } else {
            return None;
        }
    }
}
