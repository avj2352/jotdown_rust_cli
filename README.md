# ✏️ Jotdown - Todo CLI app using Rust

Jotdown CLI, `v1.2.0`. A modern command line Todo application with colorized annotations, written using Rust

![version](https://img.shields.io/badge/version-1.2.0-blue) [![Rust](https://github.com/avj2352/jotdown_rust_cli/actions/workflows/rust.yml/badge.svg)](https://github.com/avj2352/jotdown_rust_cli/actions/workflows/rust.yml)

- A modern Todo application with extra features!
- I'd probably go with **Jotdown** until I get a better name!

Jotdown version `1.2.0` CLI features -

```bash
 jd: Jotdown
       __      __      __                  
      / /___  / /_____/ /___ _      ______ 
 __  / / __ \/ __/ __  / __ \ | /| / / __ \
/ /_/ / /_/ / /_/ /_/ / /_/ / |/ |/ / / / /
\____/\____/\__/\__,_/\____/|__/|__/_/ /_/ 
                                           
  Usage:

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
    * jd update 1 "Go shopping today"           Update the description of existing todo item by its position
    * jd tag 1 "@overdue"                       Change the tag of existing todo item by its position

  Environment variables:

    JOTDOWN_FORMAT=pretty                Specify formatter (simple, pretty, mini) [default: simple]
    JOTDOWN_DB_PATH=~/Dropbox/jotdown.json  Specify DB path [default: ~/.jotdown-db.json]

```

## 🚨 RELEASE v1.2.0 available

- Release executable `v1.2.0` available under `release` folder.
- Copy the executable or run the shell script `build_release.sh`
- to generate the latest build (warning: using the shell script, generates a canary build)

## Todo synonyms, alternate names to the app

- jotdown / jot /jd
- action / act
- todov3 / todo

## Important Links

- [Rust - 100 exercises in Rust!](https://rust-exercises.com/)
- [Medium - Useful Terminal applications](https://levelup.gitconnected.com/awesome-terminal-applications-e4a06022dffa)
- [Rust CLI - Traversable options using crossterms KeyEvent](https://www.reddit.com/r/rust/comments/hz3j1h/how_to_make_a_cli_in_rust_that_allows_the_user_to/)
- [Makefile - Instructions for automating compiling & linking of a file](https://g.co/bard/share/01ed6bb954c6)
- [Considering Rust for Government Projects and Decision support systems - DSS](https://www.reddit.com/r/rust/comments/17s1o2l/is_rust_a_good_language_for_government_systems/)
- [⏰JSON to serde_json struct - online generator](https://transform.tools/json-to-rust-serde)
- [Clap Rust Tutorial](https://www.youtube.com/watch?v=Ot3qCA3Iv_8)
- [Clap crate documentation](https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html)
- [Linkedin - Rust file processing](https://www.linkedin.com/learning/practice-it-rust-file-manipulation/preassessment)
- [Rust CLI Github project](https://github.com/shashwatah/jot)
- [Node JS Todo CLI](https://github.com/vesln/todo)
- [How to include Rust CLI application to PATH](https://stackoverflow.com/questions/60944480/how-do-i-make-a-rust-program-which-can-be-executed-without-using-cargo-run)
- [Optimize your Rust project release](https://github.com/johnthagen/min-sized-rust)
- [Explore Microsoft Graph API SDK - Python](https://learn.microsoft.com/en-us/graph/sdks/sdk-installation?context=graph%2Fapi%2F1.0&view=graph-rest-1.0#install-the-microsoft-graph-javascript-sdk)
- [Codecrafters - practical Rust projects](https://app.codecrafters.io/pay)
- [Youtube - Jon Gjengset](https://www.youtube.com/watch?v=jf_ddGnum_4&t=864s)
- [Github repo - Job Gjengset](https://github.com/jonhoo/codecrafters-bittorrent-rust)
- [Rust - All algorithms](https://github.com/TheAlgorithms/Rust)
- [ClI - Typography](https://patorjk.com/software/taag/#p=display&f=Slant&t=Jotdown)

## NOTE: About the file - .jotdown-db.json

Jotdown remembers your tasks, todos and reminders by persisting them under `$HOME/.jotdown-db.json`
The default store of the JSON file is in $HOME location. This could vary based on the operating system:

- Linux OS: $HOME = `/home/alice`
- Windows OS: $HOME = `C:\Users\Alice`
- Mac OS: $HOME = `/Users/Alice`

> In the upcoming release 1.2.0 you can configure your own path to store `.jotdown-db.json`, by setting the env variable

```bash
# set datastore path
JOTDOWN_DB_PATH=~/Dropbox/jotdown.json  Specify DB path [default: $HOME/.jotdown-db.json]
```

## Run (using Cargo)

To run jotdown as a cli app using Cargo

```bash
cargo run -- --help
```

### release 1.0.0 🏁

- make crate / rust project executable
- Add installation instructions
- Make create / rust project scalable / easy to update

````

## Data structure (jotdown-db.json)

Typically structure of a `todo-db.json`

> NOTE: in `todo` the position of the element within the HashMap is the order in the UI

```json
{
  "tags": ["important", "today", "week"],
  "todos": [
    {
      "id": 1,
      "desc": "HOME: Buy groceries for the week @today",
      "status": "pending",
      "modified": "2023-09-25T13:00:04.792Z"
    }
  ],
  "reminders": [
    {
      "id": 1,
      "desc": "WORK: Send Email to Denise",
      "status": "open",
      "due": "2023-10-03T13:00:04.792Z",
      "type": "weekly",
      "modified": "2023-09-25T13:00:04.792Z"
    }
  ],
  "tasks": [
    {
      "id": 1,
      "parent": 1,
      "desc": "Order on Amazon Fresh",
      "status": "pending",
      "modified": "2023-09-25T13:00:04.792Z"
    }
  ]
}
````

> NOTE: There are 17 representations of white space in unicode text format. So using `text.split_whitespace()` is much
> better alternative to the `.split()` method in Rust

# Rust - Read and Write to JSON

To store a HashMap as a JSON file in Rust, you can use the `serde_json` crate.
This crate provides a serializer and deserializer for JSON data, which can be used to convert Rust data structures
to and from JSON.

To use `serde_json`, you will first need to add it to your project's dependencies:

```bash
# add serde_json to cargo.toml
cargo add serde_json
```

Once you have added `serde_json`, you can start using it to serialize and
deserialize your HashMap. To serialize a HashMap to a JSON file, you can use the
`.to_writer()` function. This function takes a writer as an argument and writes the serialized JSON data to it.

For example, the following code shows how to serialize a HashMap to a JSON file named my_hashmap.json:

```rust
/**
* Write HashMap to JSON file
*/
use serde_json::{Result, Value};
use std::fs::File;

fn main() -> Result<()> {
let mut hashmap = HashMap::new();
hashmap.insert("key1", "value1");
hashmap.insert("key2", "value2");

    let file = File::create("my_hashmap.json")?;
    serde_json::to_writer(file, &hashmap)?;

    Ok(())
}
```

To deserialize a JSON file into a HashMap, you can use the `.from_reader()` function.
This function takes a reader as an argument and reads the JSON data from it, returning a deserialized Rust data structure.

For example, the following code shows how to deserialize a JSON file named my_hashmap.json into a HashMap:

```rust
/**
 * Read from JSON file
 */
use serde_json::{Result, Value};
use std::fs::File;

fn main() -> Result<()> {
    let file = File::open("my_hashmap.json")?;
    let hashmap: HashMap<String, String> = serde_json::from_reader(file)?;
    // Do something with the hashmap...
    Ok(())
}
```

## Rust print line with a strikethrough

There is no built-in way to print with strikethrough in Rust. However, there are a few workarounds.

One workaround is to use the ANSI escape sequence for strikethrough.
The ANSI escape sequence for strikethrough is `\x9f`.
To use it, you can prefix the text you want to strikethrough with the `ANSI escape sequence`.
For example, the following code will print the text "Hello, world!" with strikethrough:

```rust
fn main() {
  println!("\x9fHello, world!\x9f");
}

// ̶H̶e̶l̶l̶o̶,̶ ̶w̶o̶r̶l̶d̶!̶
```

Another workaround is to use a library such as `crossterm`.
The `crossterm` library provides a number of functions for controlling the terminal,
including a function for printing with `strikethrough`. To use the `crossterm` library to print with strikethrough,
you can use the following code:

```rust
use crossterm::terminal;

fn main() {
  terminal::enable_raw_mode().unwrap();
  // Print the text "Hello, world!" with strikethrough.
  terminal::emit_strikethrough().unwrap();
  println!("Hello, world!");
  terminal::emit_reset().unwrap();
  terminal::disable_raw_mode().unwrap();
}
```

# Push Notification Features - Exploration

## 1. OneSignal

- [OneSignal documentation](https://documentation.onesignal.com/docs)

## 2. Microsoft Graph API (Todo app)

- [Microsoft Graph API](https://login.microsoftonline.com/common/oauth2/nativeclient)

## 🏁Complete feature list - with status 🏁

..for my todo cli app to be complete 😎

```bash
### release 0.1.1
- JOTDOWN initialize DB ...✅
- JOTDOWN handle empty file ...✅
- jd                                             Print todos ...✅
- jd help                                        Lists the available commands ...✅
- jd ls                                          Print all pending todo items ...✅
- jd ls                                          Handle display if no todo items present ...✅
- jd ls --todos                                  Print all pending todo items ...✅
- jd ls --all                                    Print completed and pending todo items ...✅

## release 1.0.0 (complete replacement of Node's todo cli 😎)

- jd ls --done                                   Print completed todo items ...✅
- jd add Go shopping                             Create a new todo item ...✅
- jd add Go shopping @annotate                   Create a new todo item with annotation ...✅
- jd check 1                                     Mark #1 as completed on todos ...✅
- jd undo 1                                      Revert #1 to pending ...✅
- jd mv 1 42                                     Change the id of given todo ...✅
- jd rm 1                                        Remove #1 item ...✅
- jd clear                                       Destroy all todo items ...✅
- jd clear --done                                Destroy all completed todo items ...✅
- jd sort "@important @high"                     Sort Todos in the order of their tags as mentioned ...✅

### release 1.2.0 (Introducing Tasks)
- jd ls @tag                                     Print todo items containing "@tag"
- jd ls ~@tag                                    Print todo items not containing "@tag"
- jd update --t 5 "New sentence @week"           Edit todo #5 with new sentence
- jd ls                                          Print all pending todo items with their respective tasks (if any)
- jd add -t 1 Get out of the house               Create a new task for todo item 1
- jd check 1.1                                   Mark task #1 as completed on todo #1

### release 1.3.0 (Introducing reminders)
- jd ls --reminder                               Print all reminder todos
- jd add -g another_tag                          Create a new tag by name "another_tag" and persist
- jd add -r Go shopping                          Create a new reminder item
```
