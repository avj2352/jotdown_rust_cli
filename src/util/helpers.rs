use std::io::{self, BufRead};
use std::fs::{self, File};
use crate::util::enums::TodoStatusType;
use crate::util::models::Todo;

/**
* Helper methods
* Lessons from Linkedin - Rust - File Manipulation
*/
pub fn read_file_from_path() -> Result<String, String> {
    let source_json = "jotdown-db.json";
    let contents: String = fs::read_to_string(source_json).unwrap();
    Ok(contents)
}

/**
* Using BufReader for reading large files
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

pub fn generate_todo_json() -> String {
    let todo: Todo = Todo {
        id: 1,
        desc: "this is a test pojo".to_string(),
        status: TodoStatusType::Pending,
        modified: "2023-10-02".to_string()
    };
    serde_json::to_string(&todo).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_contains_default_tags() {
        let result: String = read_file_from_path().unwrap();
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

    #[test]
    fn test_serializing_json() {
        let result: String = generate_todo_json();
        println!("Serialized json: {}", &result);
        assert_eq!(true, result != "");
    }
}