use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRequestResponse {
    pub tags: Vec<String>,
    pub todos: Vec<Todo>,
    pub tasks: Vec<Task>,
}

impl FileRequestResponse {
    pub fn new() -> Self {
        Self {
            tags: vec![],
            todos: vec![],
            tasks: vec![],
        }
    }

    pub fn update_todo_list(&mut self, record: Todo) {
        self.todos.push(record);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: i64,
    pub desc: String,
    pub tag: String,
    pub status: String,
    pub modified: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub parent: i64,
    pub desc: String,
    pub status: String,
    pub modified: String,
}
