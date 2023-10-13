mod util;
mod features;
mod facade;
mod dao;

use clap::{ command };
// util
use util::config::get_db_file_path;
// features
use features::list::gen_list_command;
use features::add::gen_add_command;
// handlers
use facade::list_handler::handle_list;
use facade::help_handler::handle_help;
use facade::add_handler::handle_add_todo;

const APP_DESCRIPTION: &str = "A modern Todo application with extra features!";
const VERSION: &str = "0.4.01";



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
        .get_matches();
    // match commands with handlers
    if let Some(matches) = cli_matches.subcommand_matches("ls") {
        handle_list(matches);
    } else if let Some(matches) = cli_matches.subcommand_matches("add") {
        handle_add_todo(matches);
    }
    else {
        // implement default handler as help
        println!("{}", handle_help());
    }
}