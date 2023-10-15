use clap::{ Command };
/**
* Consists of "jotdown" / "jd" - miscellaneous commands
* check command
* undo command
*/

/**
* CHECK command
* mark a todo item as done
* mark a task as done
* mark a reminder as done
* @returns {Command} returns subcommand
*/
pub fn gen_check_command() -> Command {
    Command::new("check")
        .about("mark as done - todo, task, reminder")
        .arg(
            clap::Arg::new("todo")
                .index(1)
        )
    // TODO: add task and reminder commands here...
}


/**
 * UNDO command
 * mark a todo item back to pending
 * mark a task as pending
 * mark a reminder as pending
 * @returns {Command} returns subcommand
 */
pub fn gen_undo_command() -> Command {
    Command::new("undo")
        .about("uncheck - todo, task, reminder")
        .arg(
            clap::Arg::new("todo")
                .index(1)
        )
    // TODO: add task and reminder commands here...
}

/**
 * MOVE command
 * move a todo task reminder item up / down the list
 * @returns {Command} returns subcommand
*/
pub fn gen_move_command() -> Command {
    Command::new("mv")
        .about("move task [start] [destination]")
        .arg(
            clap::Arg::new("start")
                .index(1)
        )
        .arg(
            clap::Arg::new("end")
                .index(2)
        )
}

/**
 * MOVE command
 * move a todo task reminder item up / down the list
 * @returns {Command} returns subcommand
 */
pub fn gen_remove_command() -> Command {
    Command::new("rm")
        .about("remove - todo, task, reminder")
        .arg(
            clap::Arg::new("index")
                .index(1)
        )
}

