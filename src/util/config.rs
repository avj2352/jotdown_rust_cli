/**
* Get db JSON file
*/
use std::fs::{File, write};
use std::path::PathBuf;
use crate::util::display::{display_err_serializing_json, display_err_writing_to_file};
use crate::util::models::FileRequestResponse;

// TODO: make db file location configurable
const CURRENT_DIRECTORY: &str = ".";
const DB_JSON_FILE: &str = "jotdown-db.json";


/**
* initialize model if file is newly created
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
* returns the path for jotdown-db.json
*/
pub fn get_db_file_path() -> String {
    let mut result: String = String::new();
    let mut path = PathBuf::new();
    path.push(CURRENT_DIRECTORY);
    path.push(DB_JSON_FILE);
    match path.to_str() {
        None => {println!("Error")}
        Some(val) => {
            create_file_if_not_exists(val).unwrap();
            result = val.to_string();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_file_path() {
        let expected_file_path = "./jotdown-db.json";
        let result = get_db_file_path();
        assert_eq!(expected_file_path, result.as_str());
    }
}