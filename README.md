# ✏️Jotdown - Todo CLI app using Rust

![version](https://img.shields.io/badge/version-0.1.02-blue) [![Rust](https://github.com/avj2352/jotdown_rust_cli/actions/workflows/rust.yml/badge.svg)](https://github.com/avj2352/jotdown_rust_cli/actions/workflows/rust.yml)

- A modern Todo application with extra features!
- I'd probably go with **Jotdown** until I get a better name!

## 🚧 WORK IN PROGRESS 🚧

## Todo synonyms, alternate names to the app

- jotdown / jot /jd
- action / act
- todov3 / todo

## Important Links

- [Clap Rust Tutorial](https://www.youtube.com/watch?v=Ot3qCA3Iv_8)
- [Clap crate documentation](https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html)
- [Linkedin - Rust file processing](https://www.linkedin.com/learning/practice-it-rust-file-manipulation/preassessment)
- [Rust CLI Github project](https://github.com/shashwatah/jot)
- [Node JS Todo CLI](https://github.com/vesln/todo)

## Run (using Cargo)

To run jotdown as a cli app using Cargo

Jotdown CLI features - 
```bash
 jd: Jotdown cli, todo superpowered

  Usage:

    jd                                  Print todos
    jd help                             Lists the available commands
    jd add Go shopping                  Create a new todo item
    jd add -r Go shopping               Create a new reminder item
    jd add -t 1 Get out of the house    Create a new task for todo item 1
    jd ls                               Print all pending todo items
    jd ls @tag                          Print todo items containing "@tag"
    jd ls ~@tag                         Print todo items not containing "@tag"
    jd ls --all                         Print completed and pending todo items
    jd ls --done                        Print completed todo items
    jd ls --reminder                    Print all reminder todos
    jd check 1                          Mark #1 as completed on todos 
    jd check 1.1                        Mark task #1 as completed on todo #1   
    jd mv 1 42                          Change the id of given todo
    jd undo 1                           Revert #1 to pending
    jd rm 1                             Remove #1 item
    jd clear                            Destroy all todo items
    jd clear --done                     Destroy all completed todo items
    jd renumber                         Re-numbers all todos starting with 1

  Environment variables:

    JOTDOWN_FORMAT=pretty                Specify formatter (simple, pretty, mini) [default: simple]
    JOTDOWN_DB_PATH=~/Dropbox/jotdown.json  Specify DB path [default: ~/.jotdown-db.json]
    
```

```bash
cargo run -- --help
```

## Data structure (jotdown-db.json)

Typically structure of a `todo-db.json`

> NOTE: in `todo` the position of the element within the HashMap is the order in the UI
> 
```json
{
  "tags": ["important", "today", "week"],
  "todos": [
    {
      "id": 1,
      "desc": "PRUDENTIAL: Send email to Denise on few line on the portion of the new AWS enabled data platform @today",
      "status": "pending",
      "modified": "2023-09-25T13:00:04.792Z"
    }
  ],
  "reminders": [
    {
      "id": 1,
      "desc": "Send timesheet from Prudential to JSR, Pamten",
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
      "desc": "Write email first",
      "status": "pending",
      "modified": "2023-09-25T13:00:04.792Z"
    }
  ]
}
```

> NOTE: There are 17 representations of white space in unicode text format. So using `text.split_whitespace()` is much
better alternative to the `.split()` method in Rust

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
