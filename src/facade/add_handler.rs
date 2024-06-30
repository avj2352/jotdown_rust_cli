use crate::dao::read_json::{fetch_todos, parse_json_from_string};
use crate::dao::write_json::serialize_model_to_json;
use crate::util::display::display_todo_added_msg;
use crate::util::enums::TodoStatusType;
use crate::util::helpers::{
    get_current_date_time_iso, get_tag_annotation_from_string, read_file_from_path,
};
use clap::ArgMatches;
// custom
use crate::util::models::{FileRequestResponse, Todo};

/**
 * Add command handler ***********************************
 * consists of handlers for jotdown "add" subcommand
 * > jd add
 */

fn create_todo(text: String, tag: String) -> Todo {
    let todos: Vec<Todo> = fetch_todos();
    let count = todos.len();
    let record: Todo = Todo {
        id: (count + 1) as i64,
        desc: text.to_string(),
        tag: tag.to_string(),
        status: TodoStatusType::Pending.to_string(),
        modified: get_current_date_time_iso(),
    };
    record
}

/**
 * "add "some todo item"" subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_add_todo(args: &ArgMatches) {
    let argument = args.get_one::<String>("todo").unwrap();
    let (_, tag, _) = get_tag_annotation_from_string(argument);
    let record: Todo = create_todo(argument.to_string(), tag);
    let json_string: String = read_file_from_path();
    let mut model: FileRequestResponse = parse_json_from_string(json_string);
    model.update_todo_list(record);
    serialize_model_to_json(model);
    println!("{}", display_todo_added_msg());
}
