use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
/**
* Displays all terminal msgs / errors
*/

// info msgs
const EMPTY_TODO_LIST: &str =
    r#"🏁 There are no TODOS, please create a new TODO task or refer to the docs --help 🏁"#;
// error msgs
const ERR_READING_FILE: &str = r#"error reading file contents!"#;
const ERR_WRITING_TO_FILE: &str = r#"error writing to the JSON file"#;
const ERR_SERIALIZING_JSON: &str = r#"error serializing json file"#;
const TODO_ADDED_MSG: &str = "Todo added...";
const ERROR_INVALID_ARGUMENT: &str = "Invalid Argument!";
const TODO_ITEM_CHECKED: &str = "Todo item marked as done!";
const TODO_ITEM_PENDING: &str = "Todo item reverted to pending!";
const TODO_ITEM_MOVED: &str = "Todo item moved!";

const TODO_ITEM_REMOVED: &str = "Todo item removed!";

fn display_msg(text: &str) -> String {
    format!("{}", text)
}

fn display_msg_success(text: String) -> String {
    format!("{}", text.as_str().cyan().italic())
}

fn display_error(err_text: &str) -> String {
    format!("🚨: {}", err_text.red().italic())
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

pub fn display_todo_added_msg() -> String {
    display_msg_success(TODO_ADDED_MSG.to_string())
}

pub fn display_err_invalid_argument() -> String {
    display_error(ERROR_INVALID_ARGUMENT)
}

pub fn display_todo_item_checked() -> String {
    display_msg_success(TODO_ITEM_CHECKED.to_string())
}

pub fn display_todo_item_pending() -> String {
    display_msg_success(TODO_ITEM_PENDING.to_string())
}

pub fn display_todo_item_moved() -> String {
    display_msg_success(TODO_ITEM_MOVED.to_string())
}

pub fn display_todo_item_removed() -> String {
    display_msg_success(TODO_ITEM_REMOVED.to_string())
}

pub fn display_todo_header() -> String {
    format!("{}", "\n ✏️ Todos: \n".cyan().bold())
}

pub fn display_sort_todos() -> String {
    format!("{}", "Todo items sorted!".cyan().italic())
}

pub fn display_err_sort_todos() -> String {
    format!("{}", "Error sorting todos!!".red().bold())
}

/**
* Displays progress bar with styled output in terminal
* @param {u64} total
* @param {u64} remaining
*/
pub fn display_progress_styled(remaining: u64, total: u64) {
    let bar = ProgressBar::new(total).with_style(
        ProgressStyle::default_bar()
            .template("Completed [{bar:20.cyan/black}] {completed}/{total} {percent}%")
            .unwrap(),
    );
    bar.finish();

    bar.set_length(total);
    bar.set_position(remaining);
}
