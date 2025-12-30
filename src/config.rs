use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

use crate::{config::yaml_parser::YamlValue, os_helper};

pub mod yaml_parser;

#[derive(Debug)]
pub struct OnoConfig {
    pub data_dir: Option<String>,
    pub editor: Option<String>,
    pub history_file: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum OnoConfigError {
    FileMalformed,
}

static CONFIG_FOLDER: &str = "ono";
static CONFIG_FILE: &str = "ono.yaml";

pub fn get_config() -> Result<OnoConfig, OnoConfigError> {
    // try to read the config from file
    let path = get_full_config_path(true);
    let config_map = if let Ok(true) = fs::exists(&path) {
        yaml_parser::parse_yaml_from_file(&path)
    } else {
        Ok(get_default_config())
    };

    println!("{:?}", config_map);

    match config_map {
        Ok(map) => {
            let history_file = match map.get("history_file") {
                Some(Some(v)) => v.get_string(),
                _ => None,
            };

            let data_dir = match map.get("data_dir") {
                Some(Some(v)) => v.get_string(),
                _ => None,
            };

            let editor = match map.get("editor") {
                Some(Some(v)) => v.get_string(),
                _ => None,
            };

            Ok(OnoConfig {
                data_dir,
                editor,
                history_file,
            })
        }
        Err(_) => Err(OnoConfigError::FileMalformed),
    }
}

fn get_full_config_path(include_filename: bool) -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or(PathBuf::from("./"));
    path.push(CONFIG_FOLDER);

    if include_filename {
        path.push(CONFIG_FILE);
    }

    path
}

fn get_default_config() -> HashMap<String, Option<YamlValue>> {
    let editor = os_helper::get_editor();
    let data_dir = os_helper::get_data_dir();

    let mut result: HashMap<String, Option<YamlValue>> = HashMap::new();

    result.insert(
        "history_file".to_string(),
        os_helper::get_history_file().map(YamlValue::String),
    );
    result.insert("editor".to_string(), Some(YamlValue::String(editor)));
    result.insert("data_dir".to_string(), Some(YamlValue::String(data_dir)));
    result
}
