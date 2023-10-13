use clap::{ arg, ArgAction, Command };
/**
 * Consists of "jotdown" / "jd" - add command
 * ADD ==============================================================
 */

/**
 * ADD command
 * add a todo item
 * add a task item
 * add a reminder item
 * @returns {Command} returns subcommand
 */
pub fn gen_add_command() -> Command {
    Command::new("add")
        .about("add a todo, task, reminder")
        .arg(
            clap::Arg::new("todo")
                .index(1)
        )
}
