use serde::{Deserialize, Serialize};

/**
* Collection of all CLI related enums
*/
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum JotType {
    Tag,
    Todo,
    Reminder,
    Task
}

/// Enum for Reminder repetition
#[derive(Serialize, Deserialize, Debug)]
pub enum ReminderType {
    Yearly,
    HalfYearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
/// Emoji Map Type of Task Priority - 🔥, ⏰, 📅, ☀️, 😎
pub enum PriorityType {
    Burning,
    High,
    Medium,
    Low,
    Casual
}

/// Emoji map for todo status - pending, done
#[derive(Serialize, Deserialize, Debug)]
pub enum TodoStatusType {
    Pending,
    Done
}