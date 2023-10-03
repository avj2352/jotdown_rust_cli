use serde_derive::{Deserialize, Serialize};
// custom
use crate::util::enums::{ TodoStatusType, ReminderType };

/**
* Collection of structs used for serialization, deserialization
*/

/**
* Todo struct
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub desc: String,
    pub status: TodoStatusType,
    pub modified: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    id: i32,
    desc: String,
    status: TodoStatusType,
    due: String,
    reminder_type: ReminderType,
    modified: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: i32,
    parent: i32,
    desc: String,
    status: TodoStatusType,
    modified: String
}