mod util;
mod features;
mod facade;
mod dao;

use clap::{ command };
// util
use util::config::get_db_file_path;
// features
use crate::features::list::gen_list_command;
use crate::features::add::gen_add_command;
use crate::features::misc::{
    gen_check_command,
    gen_move_command,
    gen_remove_command,
    gen_undo_command
};
// handlers
use crate::facade::list_handler::handle_list;
use crate::facade::help_handler::handle_help;
use crate::facade::add_handler::handle_add_todo;
use crate::facade::misc_handler::{
    handle_check_todo_task,
    handle_move_todo_task,
    handle_remove_todo_task,
    handle_revert_todo_task
};

const APP_DESCRIPTION: &str = "A modern Todo application with extra features!";
const VERSION: &str = "0.5.00";



fn main() {
    // init db
    get_db_file_path();
    // add commands
    // override default help command
    // implement default command as help command
    let cli_matches = command!()
        .about(APP_DESCRIPTION)
        .version(VERSION)
        .override_help(handle_help())
        .subcommand(gen_list_command())
        .subcommand(gen_add_command())
        .subcommand(gen_check_command())
        .subcommand(gen_undo_command())
        .subcommand(gen_move_command())
        .subcommand(gen_remove_command())
        .get_matches();
    
    // match commands with handlers
    if let Some(matches) = cli_matches.subcommand_matches("ls") {
        handle_list(matches);
    } else if let Some(matches) = cli_matches.subcommand_matches("add") {
        handle_add_todo(matches);
    } else if let Some(matches) = cli_matches.subcommand_matches("check") {
        handle_check_todo_task(matches);
    } else if let Some(matches) = cli_matches.subcommand_matches("undo") {
        handle_revert_todo_task(matches);
    } else if let Some(matches) = cli_matches.subcommand_matches("mv") {
        handle_move_todo_task(matches);
    } else if let Some(matches) = cli_matches.subcommand_matches("rm") {
        handle_remove_todo_task(matches);
    }
    else {
        // implement default handler as help
        println!("{}", handle_help());
    }
}