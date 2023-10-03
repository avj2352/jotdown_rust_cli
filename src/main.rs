mod util;
mod features;

use clap::{ command };
use features::command::gen_list_command;

fn main() {
    // add commands
    let _ = command!()
        .about("A modern Todo application with extra features!")
        .version("0.1.0")
        .subcommand(gen_list_command())
        .get_matches();
}