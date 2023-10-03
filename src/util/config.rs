/**
* Get db JSON file
*/
use std::fs::File;
use std::path::PathBuf;

// TODO: make db file location configurable
const CURRENT_DIRECTORY: &str = ".";
const DB_JSON_FILE: &str = "jotdown-db.json";

pub fn create_file_if_not_exists(path: &str) -> Result<bool, String> {
    let path = PathBuf::from(path);
    if !path.exists() {
        File::create(path).expect("Error reading path");
        return Ok(false);
    }
    Ok(true)
}
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