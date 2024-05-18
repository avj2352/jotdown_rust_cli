/**
* Help command handler ***********************************
* consists of handlers for jotdown "help" subcommand
* > jd help
*/

const HELP_TEXT: &'static str = r#"
    ✨ Jotdown CLI, v1.1.0. A modern command line Todo application with colorized annotation, written using Rust ✨

    📖 Usage:

    * jd                                        (default) jd help - Lists the available commands
    * jd help                                   Lists the available commands
    * jd add "Go shopping"                      Create a new todo item
    * jd add "Go shopping @today"               Create a new todo item with tag annotation @today
    * jd add "Go shopping @important"           Create a new todo item with tag annotation @important
    * jd add "Go shopping @week"                Create a new todo item with tag annotation @week
    * jd add "Go shopping @month"               Create a new todo item with tag annotation @month
    * jd add "Go shopping @some-annotation"     Create a new todo item with tag annotation @some-annotation
    * jd ls                                     Print all pending todo items
    * jd ls --todos                             Print all pending todo items
    * jd ls --all                               Print all items
    * jd ls --done                              Print all done todo items
    * jd check 1                                Mark #1 as completed on todos
    * jd mv 1 42                                Swap todo item 1 with 42
    * jd rm 1                                   Remove #1 todo item
    * jd clear                                  Destroy all todo items
    * jd clear --done                           Destroy all completed todo items
    * jd sort                                   Sort Todos in the order of their tags as mentioned "@important @today @week @month @some-annotation"

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
