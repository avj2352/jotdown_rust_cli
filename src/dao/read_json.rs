use serde_json::from_str;
// custom
use crate::util::models::{FileRequestResponse, Todo};
use crate::util::helpers::read_file_from_path;

/**
 * DAO access layer to read from JSON file
 */


/**
 * parse FileRequestResponse from string
 * returns {FileStructure} returns file struct
 */
pub fn parse_json_from_string(input: String) -> FileRequestResponse {
    let response: FileRequestResponse = from_str(input.as_str()).unwrap();
    response
}


/**
 * list all todos - pending & done
 * @returns {Vec<String>} task list
 */
pub fn fetch_todos() -> Vec<Todo> {
    let json_string: String = read_file_from_path();
    let response = parse_json_from_string(json_string);
    response.todos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_from_file() {
        let json_string: String = read_file_from_path();
        let response: FileRequestResponse = parse_json_from_string(json_string);
        println!("Response is: {:?}", response);
    }
}