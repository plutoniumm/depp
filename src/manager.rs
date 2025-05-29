use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Manager {
    pub name: String,
    pub add: String,
    pub remove: String,
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct ManagersMap(HashMap<String, Manager>);

pub struct Managers;

impl Managers {
    pub fn load() -> HashMap<String, Manager> {
        let data = fs::read_to_string("managers.json").unwrap();
        serde_json::from_str(&data).unwrap()
    }

    pub fn pnpm() -> Manager {
        Self::load().remove("js").unwrap()
    }

    pub fn pip() -> Manager {
        Self::load().remove("py").unwrap()
    }
}
