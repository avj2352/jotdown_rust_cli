use std::fs::write;
use crate::dao::read_json::parse_json_from_string;
use crate::util::config::get_db_file_path;
use crate::util::display::{display_err_serializing_json, display_err_writing_to_file};
use crate::util::helpers::read_file_from_path;
use crate::util::models::{FileRequestResponse, Todo};

/**
 * update model and serialize to json
 * @params {&str} path
 */
pub fn serialize_model_to_json(model: FileRequestResponse) {
    let file_path: String = get_db_file_path();
    let json_string = serde_json::to_string(&model).expect(&*display_err_serializing_json());
    write(file_path, json_string.as_bytes()).expect(&*display_err_writing_to_file());
}

/**
 * serialize new todo list to JSON file
 * @params {Vec<Todo>} todo list
 * @returns {Result<(), String>}
 */
pub fn serialize_todos_to_json (list: Vec<Todo>) -> Result<(), String> {
    let json_string: String = read_file_from_path();
    let mut model: FileRequestResponse = parse_json_from_string(json_string);
    model.todos = list;
    serialize_model_to_json(model);
    Ok(())
}