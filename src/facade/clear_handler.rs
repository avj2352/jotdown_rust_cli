use clap::ArgMatches;
use colored::Colorize;
// custom
use crate::dao::write_json::serialize_todos_to_json;
use crate::util::display::display_empty_todo;
use crate::util::models::Todo;
/**
 * Clear command handler ***********************************
 * consists of handlers for jotdown "clear" subcommand
 * > jd clear
 */

/**
 * "clear" or default subcommand handler
 * @param {&ArgMatches} args
 */
pub fn handle_clear_list(matches: &ArgMatches) {
    if matches.get_flag("done") {
        clear_all_done_items().unwrap();
    } else if matches.get_flag("todos") {
        clear_all_done_items().unwrap();
    } else {
        clear_all_items().unwrap();
    }
}

/**
* "clear --all"
* @param {&ArgMatches} args
*/
fn clear_all_items() -> Result<(), String> {
    let new_todo_list: Vec<Todo> = vec![];
    serialize_todos_to_json(new_todo_list).unwrap();
    let success = format!("{}", "Cleared all items..!".cyan().italic());
    println!("{}", success);
    Ok(())
}

/**
 * "clear --done"
 * @param {&ArgMatches} args
 */
fn clear_all_done_items() -> Result<(), String> {
    let todo_list: Vec<Todo> = crate::dao::read_json::fetch_todos();
    // guard check
    if todo_list.is_empty() {
        println!("\n{}\n", display_empty_todo());
        return Ok(());
    }
    let new_todo_list = todo_list
        .into_iter()
        .filter(|item| item.status == "pending")
        .collect();
    serialize_todos_to_json(new_todo_list).unwrap();
    let success = format!("{}", "Cleared done items..!".cyan().italic());
    println!("{}", success);
    Ok(())
}

// TODO: to only run this test locally
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_clear_done_items() {
        println!("{:?}", clear_all_done_items());
    }

    #[test]
    #[ignore]
    fn test_clear_all_items() {
        println!("{:?}", clear_all_items());
    }
}
