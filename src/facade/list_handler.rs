use crate::dao::read_json::fetch_todos;
use crate::util::display::{display_empty_todo, display_progress_styled, display_todo_header};
use crate::util::enums::TodoStatusType;
use clap::ArgMatches;
use colored::Colorize;
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
pub fn handle_list(matches: &ArgMatches) {
    if matches.get_flag("all") {
        list_all_todos();
    } else if matches.get_flag("todos") {
        list_pending_todos();
    } else if matches.get_flag("done") {
        list_completed_todos();
    } else {
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
    // display todos header
    println!("{}", display_todo_header());
    // display progress bar
    let completed_count = todo_list
        .iter()
        .filter(|item| item.status == "done")
        .count();
    display_progress_styled(completed_count as u64, todo_list.len() as u64);
    for (_, item) in todo_list.iter().enumerate() {
        let text = highlight_text(&item.desc);
        if item.status == TodoStatusType::Done.to_string() {
            println!("{}.\t{}  {}", item.id, "✓".green().bold(), text);
        } else {
            println!("{}.\t{}  {}", item.id, "✖".red().bold(), text);
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
    // display Todos header
    println!("{}", display_todo_header());
    // display progress bar
    let completed_count = todo_list
        .iter()
        .filter(|item| item.status == "done")
        .count();
    display_progress_styled(completed_count as u64, todo_list.len() as u64);

    // display todo list
    for (_, item) in todo_list
        .iter()
        .filter(|item| item.status == "pending")
        .enumerate()
    {
        let text = highlight_text(&item.desc);
        println!("{}.\t{}  {}", item.id, "✖".red().bold(), text);
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
    let completed_count = todo_list
        .iter()
        .filter(|item| item.status == "done")
        .count();
    // display Todos header
    println!("{}", display_todo_header());
    // display progress bar
    display_progress_styled(completed_count as u64, todo_list.len() as u64);
    // display todo list
    for (_, item) in todo_list
        .iter()
        .filter(|item| item.status == "done")
        .enumerate()
    {
        let text = highlight_text(&item.desc);
        println!("{}.\t{}  {}", item.id, "✓".green().bold(), text);
    }
}

// TODO: to only run this test locally
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_list_all_todos() {
        println!("{:?}", list_all_todos());
    }

    #[test]
    #[ignore]
    fn test_list_pending_todos() {
        println!("{:?}", list_pending_todos());
    }

    #[test]
    #[ignore]
    fn test_list_completed_todos() {
        println!("{:?}", list_completed_todos());
    }
}
