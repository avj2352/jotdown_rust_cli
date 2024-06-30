use clap::{arg, ArgAction, Command};
/**
 * Consists of "jd" - clear commands
 * LIST ==============================================================
 */

/**
 * CLEAR command
 * clear - reset todo, task, reminder
 * clear --done - remove all tasks, todos, reminders with status "done"
 * @returns {Command} returns subcommand
 */
pub fn gen_clear_command() -> Command {
    Command::new("clear")
        .about("clear all todos, tasks, reminders")
        .arg(arg!(-t --todos "clear done todos").action(ArgAction::SetTrue))
        .arg(arg!(-a --all "clear all todos, tasks, reminders").action(ArgAction::SetTrue))
        .arg(arg!(-d --done "clear done todos, tasks, reminders").action(ArgAction::SetTrue))
        .arg(arg!(-k --tasks "clear done tasks").action(ArgAction::SetTrue))
        .arg(arg!(-r --reminders "clear done reminders").action(ArgAction::SetTrue))
}
