use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;

use crate::{Task, TaskliteResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppData {
    pub tasks: BTreeMap<usize, Task>,
    pub tags: HashMap<String, Vec<usize>>,
    pub next_id: usize,
    pub config: AppConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub date_format: String,
}

const DEFAULT_APP_DATA_FILE: &str = "tasklite.json";
const DEFAULT_DATE_FORMAT: &str = "%d-%m-%Y";
const DEFAULT_APP_DATA_DIRECTORY: &str = ".tasklite";

fn get_app_data_dir_path() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(DEFAULT_APP_DATA_DIRECTORY);
    path
}

fn get_app_data_file_path() -> PathBuf {
    let mut path = get_app_data_dir_path();
    path.push(DEFAULT_APP_DATA_FILE);
    path
}

impl AppData {
    pub fn init() -> TaskliteResult<Self> {
        println!("Initializing app data...");
        let app_directory = get_app_data_dir_path();
        if !app_directory.exists() {
            println!("Creating directory: {:?}", app_directory);
            create_dir(app_directory)?;
        }
        let app_data_file = get_app_data_file_path();
        if !app_data_file.exists() {
            println!("Creating file: {:?}", app_data_file);
            let mut file = File::create(app_data_file)?;
            let app_data = AppData {
                tasks: BTreeMap::new(),
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
            println!("App is already initialized..");
            let file = File::open(app_data_file)?;
            let app_data: AppData = serde_json::from_reader(file)?;
            Ok(app_data)
        }
    }

    pub fn save(&self) -> TaskliteResult<()> {
        let app_data_file = get_app_data_file_path();
        let mut file = File::create(app_data_file)?;
        let json = serde_json::to_string(&self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
