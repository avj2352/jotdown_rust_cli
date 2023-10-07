use std::io::{self, BufRead, Read};
use std::fs::{self, File};
use crate::util::config::get_db_file_path;

/**
* Helper methods ***********************************
* ...contains lessons from Linkedin - Rust - File Manipulation
*/

/**
* Read file contents to string
* @returns {String} file contents as string
*/
pub fn read_file_from_path() -> String {
    let file_path: String = get_db_file_path();
    let mut file: File = File::open(file_path).unwrap();
    let mut json_string: String = String::new();
    file.read_to_string(&mut json_string).unwrap();
    json_string
}

/**
* Using BufReader for reading large files
* FOR TESTING PURPOSE
*/
pub fn read_file_with_lines() -> Result<Vec<String>, String> {
    let source_json = "jotdown-db.json";
    let file:File = File::open(source_json).unwrap();
    // another approach - using io::BufReader for large files
    let reader = io::BufReader::new(file);
    let result: Vec<String> = reader.lines().into_iter()
                                .map(|item| item.unwrap()).collect();
    Ok(result)
}

/**
 * Using Vectors fith file reads
 * FOR TESTING PURPOSE
 */
pub fn return_lines_only_if_contains_string(test: &str) -> Result<Vec<String>, String> {
    let source_json = "jotdown-db.json";
    let file: String = fs::read_to_string(source_json).unwrap();
    let mut results: Vec<String> = Vec::new();
    for line in file.lines() {
        if line.contains(test) {
            results.push(line.to_string());
        }
    }
    Ok(results)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_contains_default_tags() {
        let result: String = read_file_from_path();
        assert_eq!(true, result.as_str().contains("tags"));
        assert_eq!(true, result.as_str().contains("important"));
        assert_eq!(true, result.as_str().contains("today"));
        assert_eq!(true, result.as_str().contains("week"));
    }

    #[test]
    fn test_read_file_into_strings_list() {
        let result: Vec<String> = read_file_with_lines().unwrap();
        assert_eq!(true, result.len() > 0);
    }

    #[test]
    fn test_return_lines_only_if_contains_string() {
        let result: Vec<String> = return_lines_only_if_contains_string("important").unwrap();
        // println!("result is: {:?}", &result);
        assert_eq!(true, result.len() > 0);
    }
}