use clap::ArgMatches;
use colored::Colorize;
use crate::dao::read_json::fetch_todos;
use crate::util::display::display_empty_todo;
use crate::util::enums::TodoStatusType;
// custom
use crate::util::helpers::highlight_text;
use crate::util::models::Todo;
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
    } else if matches.get_flag("done") {
        list_completed_todos();
    }
    else {
        // default > "ls -t"
        list_pending_todos();
    }
}

/**
* "ls -t" or default subcommand handler
* @param {&ArgMatches} args
*/
fn list_all_todos() {
    let todo_list: Vec<Todo> = crate::dao::read_json::fetch_todos();
    // guard check
    if todo_list.len() == 0 {
        println!("\n{}\n", display_empty_todo());
        return ();
    }
    println!("\n💡 Todos: \n");
    for (_, item) in todo_list
        .iter()
        .enumerate() {
            let text= highlight_text(&item.desc);
            if item.status == TodoStatusType::Done.to_string() {
                println!("{}.  {}  {}", item.id, "✓".green().bold(), text);
            } else {
                println!("{}.  {}  {}", item.id, "✖".red().bold(), text);
            }
        }
}

/**
 * "ls -t" or default subcommand handler
 * @param {&ArgMatches} args
 */
fn list_pending_todos() {
    let todo_list: Vec<Todo> = fetch_todos();
    // guard check
    if todo_list.len() == 0 {
        println!("\n{}\n", display_empty_todo());
        return ();
    }
    println!("\n💡 Todos: \n");
    for (_, item) in todo_list
            .iter()
            .filter(|item| item.status == "pending")
            .enumerate() {
                let text= highlight_text(&item.desc);
                println!("{}.  {}  {}", item.id, "✖".red().bold(), text);

            }
}

/**
 * "ls -d" list done subcommand handler
 * @param {&ArgMatches} args
 */
fn list_completed_todos() {
    let todo_list: Vec<Todo> = fetch_todos();
    // guard check
    if todo_list.len() == 0 {
        println!("\n{}\n", display_empty_todo());
        return ();
    }
    println!("\n💡 Todos: \n");
    for (_, item) in todo_list
        .iter()
        .filter(|item| item.status == "done")
        .enumerate() {
        let text= highlight_text(&item.desc);
        println!("{}.  {}  {}", item.id, "✓".green().bold(), text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_all_todos() {
        println!("{:?}", list_all_todos());
    }

    #[test]
    fn test_list_pending_todos() {
        println!("{:?}", list_pending_todos());
    }

    #[test]
    fn test_list_completed_todos() {
        println!("{:?}", list_completed_todos());
    }
}