mod util;
mod features;
mod facade;

use clap::{ command };
// custom
use features::list::gen_list_command;
use facade::list_handler::handle_list_todo;

const APP_DESCRIPTION: &str = "A modern Todo application with extra features!";
const VERSION: &str = "0.1.03";



fn main() {
    // add commands
    let cli_matches = command!()
        .about(APP_DESCRIPTION)
        .version(VERSION)
        .subcommand(gen_list_command())
        .get_matches();
    // match list
    if let Some(matches) = cli_matches.subcommand_matches("list") {
        handle_list_todo(matches);
    }
}