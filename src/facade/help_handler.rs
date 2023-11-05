/**
* Help command handler ***********************************
* consists of handlers for jotdown "help" subcommand
* > jd help
*/

const HELP_TEXT: &'static str = r#"
    ✨ jd: Jotdown cli, A modern Todo application with extra features ✨

    📖 Usage:

    * jd                                  (default) jd help - Lists the available commands
    * jd help                             Lists the available commands
    * jd add Go shopping                  Create a new todo item
    * jd add -r Go shopping               Create a new reminder item
    * jd add -t 1 Get out of the house    Create a new task for todo item 1
    * jd ls                               Print all pending todo items
    * jd ls --todos                       Print completed and pending todo items
    * jd ls --all                         Print all items
    * jd check 1                          Mark #1 as completed on todos
    * jd mv 1 42                          Change the id of given todo
    * jd undo 1                           Revert #1 to pending
    * jd rm 1                             Remove #1 item
    * jd clear                            Destroy all todo items
    * jd clear --done                     Destroy all completed todo items
    * jd sort "@important @high"          Sort Todos in the order of their tags as mentioned

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