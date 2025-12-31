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
    let config_map = yaml_parser::parse_yaml_from_file(&path).unwrap_or(get_default_config());
    let get_val = |key: &str| {
        config_map
            .get(key)
            .and_then(|v| v.as_str())
            .map(String::from)
    };

    Ok(OnoConfig {
        data_dir: get_val("data_dir"),
        history_file: get_val("history_file"),
        editor: get_val("editor"),
    })
}

fn get_full_config_path(include_filename: bool) -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or(PathBuf::from("./"));
    path.push(CONFIG_FOLDER);

    if include_filename {
        path.push(CONFIG_FILE);
    }

    path
}

fn get_default_config() -> HashMap<String, YamlValue> {
    let editor = os_helper::get_editor();
    let data_dir = os_helper::get_data_dir();

    let mut result: HashMap<String, YamlValue> = HashMap::new();

    result.insert(
        "history_file".to_string(),
        os_helper::get_history_file().map_or_else(|| YamlValue::Null, YamlValue::String),
    );
    result.insert("editor".to_string(), YamlValue::String(editor));
    result.insert("data_dir".to_string(), YamlValue::String(data_dir));
    result
}
