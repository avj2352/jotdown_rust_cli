/**
* Help command handler ***********************************
* consists of handlers for jotdown "help" subcommand
* > jd help
*/

const HELP_TEXT: &'static str = r#"
    ✨ jd: Jotdown cli, A modern Todo application with extra features ✨

    📖 Usage:

    * jotdown                                  Print todos
    * jotdown help                             Lists the available commands
    * jotdown add Go shopping                  Create a new todo item
    * jotdown add -r Go shopping               Create a new reminder item
    * jotdown add -t 1 Get out of the house    Create a new task for todo item 1
    * jotdown ls                               Print all pending todo items
    * jotdown ls @tag                          Print todo items containing "@tag"
    * jotdown ls ~@tag                         Print todo items not containing "@tag"
    * jotdown ls --all                         Print completed and pending todo items
    * jotdown ls --done                        Print completed todo items
    * jotdown ls --reminder                    Print all reminder todos
    * jotdown check 1                          Mark #1 as completed on todos
    * jotdown check 1.1                        Mark task #1 as completed on todo #1
    * jotdown mv 1 42                          Change the id of given todo
    * jotdown undo 1                           Revert #1 to pending
    * jotdown rm 1                             Remove #1 item
    * jotdown clear                            Destroy all todo items
    * jotdown clear --done                     Destroy all completed todo items
    * jotdown renumber                         Re-numbers all todos starting with 1
    * jotdown sort "@important @high"          Sort Todos in the order of their tags as mentioned

  Environment variables:

    JOTDOWN_FORMAT=pretty                Specify formatter (simple, pretty, mini) [default: simple]
    JOTDOWN_DB_PATH=~/Dropbox/jotdown.json  Specify DB path [default: ~/.jotdown-db.json]
"#;

/**
* "help" subcommand handler
* prints command manifest
*/
pub fn handle_help() -> String {
    format!("{}", HELP_TEXT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_help() {
        handle_help();
    }
}