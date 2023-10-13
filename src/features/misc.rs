use clap::{ Command };
/**
* Consists of "jotdown" / "jd" - miscellaneous commands
*/

/**
* HELP command
* jd help
* @returns {Command} returns subcommand
*/
#[allow(dead_code)]
pub fn gen_help_command() -> Command {
    Command::new("help")
}

