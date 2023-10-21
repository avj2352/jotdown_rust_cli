use clap::{ arg, ArgAction, Command };
/**
* Consists of "jotdown" / "jd" - list commands
* LIST ==============================================================
*/

/**
* LIST command
* lists all todos - jd list --todos
* lists all reminders - jd list --reminders
* lists all tasks of a todo - jd list --tasks 1
* list everything - jd list --a
* @returns {Command} returns subcommand
*/
pub fn gen_list_command() -> Command {
    Command::new("ls")
        .about("lists all todos, tasks, reminders")
        .arg(arg!(-t --todos "lists pending todos").action(ArgAction::SetTrue))
        .arg(arg!(-a --all "lists all todos, tasks, reminders").action(ArgAction::SetTrue))
        .arg(arg!(-d --done "lists completed todos").action(ArgAction::SetTrue))
        .arg(arg!(-k --tasks "lists pending tasks").action(ArgAction::SetTrue))
        .arg(arg!(-r --reminders "lists pending reminders").action(ArgAction::SetTrue))
}
