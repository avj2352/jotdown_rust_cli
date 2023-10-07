use std::io::Read;
use clap::ArgMatches;
use serde_json::from_str;
// custom
use crate::util::helpers::read_file_from_path;
use crate::util::models::{FileRequestResponse, Task, Todo};
/**
* List command handler ***********************************
* consists of handlers for jotdown list subcommand
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
 * list todo handler
 * @returns {Vec<String>} task list
 */
pub fn list_todo() -> Vec<String> {
    let json_string: String = read_file_from_path();
    let response = parse_json_from_string(json_string);
    let todo_list: Vec<Todo> = response.todos;
    if todo_list.len() == 0  {
        return vec![];
    }
    todo_list.iter().map(|item| String::from(&item.desc)).collect()
}

/**
* list task handler
* @returns {Vec<String>} task list
*/
pub fn list_task() -> Vec<String> {
    let json_string: String = read_file_from_path();
    let response = parse_json_from_string(json_string);
    let task_list: Vec<Task> = response.tasks;
    if task_list.len() == 0  {
        return vec![];
    }
    task_list.iter().map(|item| String::from(&item.desc)).collect()
}

pub fn handle_list_todo(matches: &ArgMatches) {
    // "$ myapp test" was run
    if matches.get_flag("todos") {
        // "$ myapp test -l" was run
        let todo_list: Vec<String> = list_todo();
        println!("✏️ Todos: ");
        for item in todo_list {
            println!("❌ {}", item.as_str());
        }
    } else {
        println!("Not printing list...");
    }
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