use clap::ArgMatches;
use serde_json::from_str;
// custom
use crate::util::helpers::{highlight_text, read_file_from_path};
use crate::util::models::{FileRequestResponse, Task, Todo};
/**
* List command handler ***********************************
* consists of handlers for jotdown "ls" subcommand
* > jd ls
*/

/**
 * "ls -t" or default subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_list (matches: &ArgMatches) {
    if matches.get_flag("all") {
        list_all_todos();
    } else if matches.get_flag("todos") {
        list_pending_todos();
    }
    else {
        // default > "ls -t"
        list_pending_todos();
    }
}


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
pub fn fetch_todo() -> Vec<Todo> {
    let json_string: String = read_file_from_path();
    let response = parse_json_from_string(json_string);
    response.todos
}

/**
* list task subcommand handler
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

/**
* "ls -t" or default subcommand handler
* @param {&ArgMatches} args
*/
pub fn list_all_todos() {
    let todo_list: Vec<Todo> = fetch_todo();
    println!("\n💡 Todos: \n");
    for (index, item) in todo_list
        .iter()
        .enumerate() {
            let text= highlight_text(&item.desc);
            if item.status == "done" {
                println!("{}.  ✅  {}", item.id, text);
            } else {
                println!("{}.  ❌  {}", item.id, text);
            }
        }
}

/**
 * "ls -t" or default subcommand handler
 * @param {&ArgMatches} args
 */
pub fn list_pending_todos() {
    let todo_list: Vec<Todo> = fetch_todo();
    println!("\n💡 Todos: \n");
    for (index, item) in todo_list
            .iter()
            .filter(|item| item.status == "pending")
            .enumerate() {
                let text= highlight_text(&item.desc);
                if item.status == "done" {
                    println!("{}.  ✅  {}", item.id, text);
                } else {
                    println!("{}.  ❌  {}", item.id, text);
                }
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