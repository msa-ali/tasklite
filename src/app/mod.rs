use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;

use crate::{Task, TaskliteResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppData {
    pub tasks: HashMap<usize, Task>,
    pub tags: HashMap<String, Vec<usize>>,
    pub next_id: usize,
    pub config: AppConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub date_format: String,
}

const DEFAULT_APP_DATA_FILE: &str = "tasklite.json";
const DEFAULT_DATE_FORMAT: &str = "%d-%m-%Y %H:%M:%S";
const DEFAULT_APP_DATA_DIRECTORY: &str = ".tasklite";

fn get_app_data_directory() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(DEFAULT_APP_DATA_DIRECTORY);
    path
}

fn get_app_data_file() -> PathBuf {
    let mut path = get_app_data_directory();
    path.push(DEFAULT_APP_DATA_FILE);
    path
}

impl AppData {
    pub fn init() -> TaskliteResult<Self> {
        // read user's home/.tasklite/tasklite.json
        let app_directory = get_app_data_directory();
        if !app_directory.exists() {
            create_dir(app_directory)?;
        }
        let app_data_file = get_app_data_file();
        // if file doesn't exist, create it
        if !app_data_file.exists() {
            let mut file = File::create(app_data_file)?;
            let app_data = AppData {
                tasks: HashMap::new(),
                tags: HashMap::new(),
                next_id: 1,
                config: AppConfig {
                    date_format: DEFAULT_DATE_FORMAT.to_string(),
                },
            };
            let json = serde_json::to_string(&app_data)?;
            file.write_all(json.as_bytes())?;
            Ok(app_data)
        } else {
            let file = File::open(app_data_file)?;
            let app_data: AppData = serde_json::from_reader(file)?;
            Ok(app_data)
        }
    }
}
