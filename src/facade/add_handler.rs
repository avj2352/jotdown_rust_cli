use clap::ArgMatches;
use crate::dao::read_json::{fetch_todos, parse_json_from_string};
use crate::dao::write_json::serialize_model_to_json;
use crate::util::display::display_todo_added_msg;
use crate::util::helpers::{get_current_date_time_iso, read_file_from_path};
// custom
use crate::util::models::{FileRequestResponse, Todo};

/**
 * Add command handler ***********************************
 * consists of handlers for jotdown "add" subcommand
 * > jd add
 */

fn create_todo(text: String) -> Todo {
    let mut todos: Vec<Todo> = fetch_todos();
    let count = todos.len();
    let record: Todo = Todo {
        id: (count + 1) as i64,
        desc: text.to_string(),
        status: "pending".to_string(),
        modified: get_current_date_time_iso()
    };
    record
}

/**
 * "add "some todo item"" subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_add_todo (args: &ArgMatches) {
    let argument = args.get_one::<String>("todo").unwrap();
    println!("you entered: {}", argument);
    let record: Todo = create_todo(argument.to_string());
    let json_string: String = read_file_from_path();
    let mut model: FileRequestResponse = parse_json_from_string(json_string);
    model.update_todo_list(record);
    serialize_model_to_json(model);
    println!("{}", display_todo_added_msg());
}

/**
* persist text to JSON
*/
fn write_to_json(text: String) {

}


