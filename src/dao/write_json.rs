use std::fs::write;
use crate::util::config::get_db_file_path;
use crate::util::display::{display_err_serializing_json, display_err_writing_to_file};
use crate::util::models::FileRequestResponse;

/**
 * update model and serialize to json
 * @params {&str} path
 */
pub fn serialize_model_to_json(model: FileRequestResponse) {
    let file_path: String = get_db_file_path();
    let json_string = serde_json::to_string(&model).expect(&*display_err_serializing_json());
    write(file_path, json_string.as_bytes()).expect(&*display_err_writing_to_file());
}