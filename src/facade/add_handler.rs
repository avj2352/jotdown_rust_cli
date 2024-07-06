use crate::dao::read_json::{fetch_todos, parse_json_from_string};
use crate::dao::write_json::{serialize_model_to_json, serialize_todos_to_json};
use crate::util::display::display_todo_added_msg;
use crate::util::enums::TodoStatusType;
use crate::util::helpers::{get_current_date_time_iso, get_description_from_text, get_tag_annotation_from_string, read_file_from_path};
use clap::ArgMatches;
// custom
use crate::util::models::{FileRequestResponse, Todo};

/**
 * Add command handler ***********************************
 * consists of handlers for jotdown "add" subcommand
 * > jd add
 * > jd update 1 "new description"
 * > jd tag 2 "@overdue"
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

/**
 * update description for todo item
 * @param {&ArgMatches} args
*/
pub fn handle_update_description(args: &ArgMatches) {
    if let None = args.get_one::<String>("idx") {
        println!("INVALID: Missing / Invalid index");
    }
    if let None = args.get_one::<String>("desc") {
        println!("INVALID: Missing / Invalid description");
    }
    let id = args.get_one::<String>("idx").unwrap();
    let desc = args.get_one::<String>("desc").unwrap();
    // println!("Description is: {}", &desc);
    let todos: Vec<Todo> = fetch_todos();
    let new_list = todos
        .into_iter()
        .map(|mut item| {
            if item.id == id.parse::<i64>().unwrap()  {
                item.desc = format!("{} {}", desc, item.tag);
                item.modified = get_current_date_time_iso();
            }
            return item;
        })
        .collect();
    serialize_todos_to_json(new_list).unwrap();
}

/**
 * update tag annotation for todo item
 * @param {&ArgMatches} args
 */
pub fn handle_update_tag(args: &ArgMatches) {
    if let None = args.get_one::<String>("idx") {
        println!("INVALID: Missing / Invalid index");
        return ();
    }
    if let None = args.get_one::<String>("tag") {
        println!("INVALID: Missing / Invalid tag annotation");
        return ();
    }
    let id = args.get_one::<String>("idx").unwrap();
    let tag = args.get_one::<String>("tag").unwrap();
    if !tag.contains("@") {
        println!("INVALID: Invalid tag annotation");
        return ();
    }
    // println!("Tag annotation is: {}", &tag);
    let todos: Vec<Todo> = fetch_todos();
    let new_list = todos
        .into_iter()
        .map(|mut item| {
            if item.id == id.parse::<i64>().unwrap()  {
                let parsed_desc: String = get_description_from_text(&item.desc);
                item.desc = format!("{} {}", parsed_desc, tag);
                item.tag = format!("{}", tag);
                item.modified = get_current_date_time_iso();
            }
            return item;
        })
        .collect();
    serialize_todos_to_json(new_list).unwrap();
}
