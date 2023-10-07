use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRequestResponse {
    pub tags: Vec<String>,
    pub todos: Vec<Todo>,
    pub reminders: Vec<Reminder>,
    pub tasks: Vec<Task>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: i64,
    pub desc: String,
    pub status: String,
    pub modified: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    pub id: i64,
    pub desc: String,
    pub status: String,
    pub due: String,
    #[serde(rename = "reminder_type")]
    pub reminder_type: String,
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
