use crate::util::display::{display_err_serializing_json, display_err_writing_to_file};
use crate::util::models::FileRequestResponse;
/**
* Get db JSON file
*/
use colored::Colorize;
use dirs;
use std::env;
use std::fs::{write, File};
use std::path::PathBuf;

// make db file location configurable (default - $HOME)
const DB_JSON_FILE: &str = ".jotdown-db.json";

/**
* initialize model if file is newly created
* @params {&str} path
*/
pub fn initialize_model_to_json(path: &str) {
    let new_model: FileRequestResponse = FileRequestResponse::new();
    let json_string = serde_json::to_string(&new_model).expect(&*display_err_serializing_json());
    write(path, json_string.as_bytes()).expect(&*display_err_writing_to_file());
}

/**
* create jotdown-db.json if file does not exists
* also persists the model into JSON using serde_json
* @param {&str} path string
* @returns {Result<bool, String>} returns a result
*/
pub fn create_file_if_not_exists(path: &str) -> Result<bool, String> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        // create json file
        File::create(&path_buf).expect("Error reading path");
        // persist model to json file
        initialize_model_to_json(path);
        return Ok(false);
    }
    Ok(true)
}

/**
* Get local directory .jotdown-db.json path
* @returns {Option<String>} returns Some(path) if local db exists, None otherwise
*/
pub fn get_local_db_path() -> Option<String> {
    let current_dir = env::current_dir().ok()?;
    let mut local_path = PathBuf::from(current_dir);
    local_path.push(DB_JSON_FILE);

    if local_path.exists() {
        local_path.to_str().map(|s| s.to_string())
    } else {
        None
    }
}

/**
* Get home directory, .jotdown-db.json path
* @returns {String} returns path to home directory
*/
pub fn get_home_db_path() -> String {
    let mut path = PathBuf::new();
    let home_dir = dirs::home_dir().expect("Error getting home dir");
    path.push(home_dir);
    path.push(DB_JSON_FILE);
    path.to_str()
        .expect("Error reading path to string")
        .to_string()
}

/**
* Display database location indicator
* param {bool} is_local - true if using local db, false if using home db
*/
pub fn display_db_location(is_local: bool) {
    match is_local {
        true => {
            let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let dir_name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("current");
            println!(
                "{}",
                format!("📁 Using local database in: {}", dir_name)
                    .bright_black()
                    .italic()
            );
        }
        false => {
            println!(
                "{}",
                format!("🏠 Using glocal database at: ~/{}", DB_JSON_FILE)
                    .bright_black()
                    .italic()
            );
        }
    }
}

/**
* Initialize local .jotdown-db.json() in current directory
* @returns  {Result<String, String> returns ok(path) or Err(msg)}
*/
pub fn initialize_local_db() -> Result<String, String> {
    let current_dir =
        env::current_dir().map_err(|e| format!("Error getting current directory: {}", e))?;
    let mut local_path = PathBuf::from(current_dir);
    local_path.push(DB_JSON_FILE);

    let path_str = local_path
        .to_str()
        .ok_or_else(|| "Error creating path to string".to_string())?
        .to_string();

    if local_path.exists() {
        return Err(format!(
            "{} already exists in current directory",
            DB_JSON_FILE
        ));
    }

    // create file & initialize with empty model
    File::create(&local_path).map_err(|e| format!("Error creating file: {}", e))?;
    initialize_model_to_json(&path_str);
    Ok(path_str)
}

/**
* returns the path for jotdown-db.json
* first checks for local .jotdown-db.json in current directory
* Falls back to home directory if not found
*/
pub fn get_db_file_path() -> String {
    // check for local database first
    if let Some(local_path) = get_local_db_path() {
        return local_path;
    }
    // default: fall back to home directory
    let home_path = get_home_db_path();
    create_file_if_not_exists(&home_path).unwrap();
    home_path
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: to only run this test locally
    #[test]
    #[ignore]
    fn test_db_file_path() {
        let mut path = PathBuf::new();
        let home_dir = dirs::home_dir().expect("Error getting home dir");
        path.push(home_dir);
        path.push(".jotdown-db.json");
        let expected_file_path = path.to_str().expect("Error getting path as string");
        let result = get_db_file_path();
        assert_eq!(expected_file_path, result.as_str());
    }
}
