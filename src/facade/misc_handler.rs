use clap::ArgMatches;
use crate::dao::read_json::{fetch_todos, parse_json_from_string};
use crate::dao::write_json::serialize_model_to_json;
use crate::util::display::{
    display_err_invalid_argument,
    display_todo_item_checked,
    display_todo_item_moved,
    display_todo_item_pending,
    display_todo_item_removed};
use crate::util::enums::{IntFloat, TodoStatusType};
use crate::util::helpers::{check_string_is_i32_or_f64, get_current_date_time_iso, read_file_from_path};
// custom
use crate::util::models::{FileRequestResponse, Todo};

/**
* Misc command handler ***********************************
* consists of handlers for jot-down "check, undo" subcommand
* > jd check 1
* > jd check 1.1
* > jd check rem 1
* > jd undo 1
* > jd undo 1.1
* > jd undo rem 1
 */

/**
* function to check conditions for move - start end positions
* TODO: validate for the following requirements
* 1. ensure start end are inbounds of todo list ..check
* 2. ensure only start can also be a task number
* 3. ensure destination can be a whole number
* @params {i32} start position
* @params {i32} end position
* @returns {bool} validation
*/
fn validate_todo_start_end_positions(start: i32, end: i32) -> bool {
    let todos: Vec<Todo> = fetch_todos();
    if start < 0 || start > todos.len() as i32 || end < 0 || end > todos.len() as i32 {
        return false;
    }
    true
}

/**
* function to swap todo items in list
*/
fn swap_todo_items_based_on_idx (start: i32, end: i32) -> Vec<Todo> {
    let mut todos: Vec<Todo> = fetch_todos();
    todos.swap((start-1) as usize, (end-1) as usize);
    // in-place loop
    for (idx, item) in todos.iter_mut().enumerate() {
        item.id = (idx + 1) as i64;
    }
    todos
}


/**
* retrieve todo list from JSON and also perform guard check
* @param {i32} todo index
* @returns {Result<Vec<Todo>, String>} todo list
*/
fn retrieve_todos_guard_check_index (idx: i32) -> Result<Vec<Todo>, String> {
    let todos: Vec<Todo> = fetch_todos();
    if idx < 0 || idx > todos.len() as i32 {
        return Err("argument out of bounds!".to_string());
    }
    Ok(todos)
}

/**
* Mark item in todo list as done / pending
* @params {Vec<Todo>} input list
* @params {i32} idx of the list
* @params {TodoStatusType} status
* @returns {Vec<Todo>} new list
* TODO: Make function in place algorithm, right now creates a new list
*/
fn mark_todo_item_in_list (list: Vec<Todo>, idx: i32, status: TodoStatusType) -> Vec<Todo> {
    let new_list = list.into_iter().map(|mut item| {
        if item.id == idx as i64 {
            item.status = format!("{}",status);
            item.modified = get_current_date_time_iso();
        }
        return item;
    }).collect();
    new_list
}

/**
 * serialize new todo list to JSON file
 * @params {Vec<Todo>} todo list
 * @returns {Result<(), String>}
 */
fn serialize_todos_to_json (list: Vec<Todo>) -> Result<(), String> {
    let json_string: String = read_file_from_path();
    let mut model: FileRequestResponse = parse_json_from_string(json_string);
    model.todos = list;
    serialize_model_to_json(model);
    Ok(())
}


/**
* function to mark todo item as checked
* @params {i32} num
* @returns {Result<(), Err<String>>} result enum
*/
fn check_todo (num: i32) -> Result<(), String> {
    // ..retrieve todo from list
    // ..check inbounds
    let todos = retrieve_todos_guard_check_index(num)?;
    // ..mark status as done
    let new_todos = mark_todo_item_in_list(todos, num, TodoStatusType::Done);
    // ..write to json file
    serialize_todos_to_json(new_todos)?;
    Ok(())
}

/**
 * function to mark todo item as checked
 * @params {i32} num
 * @returns {Result<(), Err<String>>} result enum
 */
fn remove_todo (num: i32) -> Result<(), String> {
    // ..retrieve todo from list
    // ..check inbounds
    let todos = retrieve_todos_guard_check_index(num)?;
    // ..mark status as done
    let new_todos = todos.into_iter().filter(|item| {
        item.id != num as i64
    }).collect();
    // ..write to json file
    serialize_todos_to_json(new_todos)?;
    Ok(())
}

/**
 * function to mark todo item as checked
 * @params {i32} num
 * @returns {Result<(), Err<String>>} result enum
 */
fn revert_todo (num: i32) -> Result<(), String> {
    // ..retrieve todo from list
    // ..check inbounds
    let todos = retrieve_todos_guard_check_index(num)?;
    // ..mark status as pending
    let new_todos = mark_todo_item_in_list(todos, num, TodoStatusType::Pending);
    // ..write to json file
    serialize_todos_to_json(new_todos)?;
    Ok(())
}


/**
 * "check "some todo item"" subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_check_todo_task (args: &ArgMatches) {
    // check type
    let argument = args.get_one::<String>("todo").unwrap();
    // println!("you entered: {}", argument);
    if let Some(num) = check_string_is_i32_or_f64(argument) {
        match num {
            IntFloat::Int(int_val) => {
                match check_todo(int_val) {
                    Ok(()) => println!("{}", display_todo_item_checked()),
                    Err(val) => println!("Error! error marking todo item as checked: {}", val)
                }
            },
            IntFloat::Float(flt_val) => println!("You've entered a float: {}", flt_val)
        }
    } else {
        println!("{}", display_err_invalid_argument());
    }
}

/**
 * "undo todo item" subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_revert_todo_task (args: &ArgMatches) {
    // check type
    let argument = args.get_one::<String>("todo").unwrap();
    // println!("you entered: {}", argument);
    if let Some(num) = check_string_is_i32_or_f64(argument) {
        match num {
            IntFloat::Int(int_val) => {
                match revert_todo(int_val) {
                    Ok(()) => println!("{}", display_todo_item_pending()),
                    Err(val) => println!("Error! error reverting todo item: {}", val)
                }
            },
            IntFloat::Float(flt_val) => println!("You've entered a float: {}", flt_val)
        }
    } else {
        println!("{}", display_err_invalid_argument());
    }
}

/**
 * "move todo item" subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_move_todo_task (args: &ArgMatches) {
    // check type
    let start_pos = args.get_one::<String>("start").expect("Invalid task position [start]");
    let end_pos = args.get_one::<String>("end").expect("Invalid task position [destination]");
    let start: Option<IntFloat> = check_string_is_i32_or_f64(start_pos);
    let end: Option<IntFloat> = check_string_is_i32_or_f64(end_pos);
    match (start, end) {
        (Some(a), Some(b)) => {
            match (a, b) {
                (IntFloat::Int(a_pos), IntFloat::Int(b_pos)) => {
                    if !validate_todo_start_end_positions(a_pos, b_pos) {
                        println!("Error: invalid index range!");
                        return ();
                    }
                    let todos = swap_todo_items_based_on_idx(a_pos, b_pos);
                    serialize_todos_to_json(todos).unwrap();
                    println!("{}", display_todo_item_moved());
                },
                _ => {
                    //TODO: implement move task feature
                    println!("Move task feature coming as part of release 1.2.0");
                }
            }
        },
        (None, None) => println!("You've entered an invalid argument pair!"),
        (Some(_), None) => println!("You've entered an invalid argument pair!"),
        (None, Some(_)) => println!("You've entered an invalid argument pair!")
    };
}

/**
 * "remove todo, task item" subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_remove_todo_task (args: &ArgMatches) {
    // check type
    let argument = args.get_one::<String>("index").unwrap();
    // println!("you entered: {}", argument);
    if let Some(num) = check_string_is_i32_or_f64(argument) {
        match num {
            IntFloat::Int(int_val) => {
                match remove_todo(int_val) {
                    Ok(()) => println!("{}", display_todo_item_removed()),
                    Err(val) => println!("Error! error removing todo item: {}", val)
                }
            },
            IntFloat::Float(flt_val) => println!("You've entered a float: {}", flt_val)
        }
    } else {
        println!("{}", display_err_invalid_argument());
    }
}
