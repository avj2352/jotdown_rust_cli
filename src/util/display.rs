/**
* Displays all terminal msgs / errors
*/

// info msgs
const EMPTY_TODO_LIST: &str = r#"🏁 There are no TODOS, please create a new TODO task or refer to the docs --help 🏁"#;

// error msgs
const ERR_READING_FILE: &str = r#"error reading file contents!"#;
const ERR_WRITING_TO_FILE: &str = r#"error writing to the JSON file"#;
const ERR_SERIALIZING_JSON: &str = r#"error serializing json file"#;

fn display_msg(text: &str) -> String {
    format!("{}", text)
}

fn display_error(err_text: &str) -> String {
    format!("Error: {}", err_text)
}


// ..display info msgs
pub fn display_empty_todo() -> String {
    display_msg(EMPTY_TODO_LIST)
}

// ...display error msgs

pub fn display_err_serializing_json() -> String {
    display_error(ERR_SERIALIZING_JSON)
}

pub fn display_err_writing_to_file() -> String {
    display_error(ERR_WRITING_TO_FILE)
}

pub fn display_err_reading_file() -> String {
    display_error(ERR_READING_FILE)
}


