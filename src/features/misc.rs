use clap::{ Command };
/**
* Consists of "jotdown" / "jd" - miscellaneous commands
*/

/**
* HELP command
* jd help
* @returns {Command} returns subcommand
*/
pub fn gen_help_command() -> Command {
    Command::new("help")
}