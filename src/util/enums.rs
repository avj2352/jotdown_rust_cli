use serde::{Deserialize, Serialize};
use colored::Colorize;

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

#[derive(Debug)]
pub enum IntFloat {
    Int(i32),
    Float(f64)
}



/// Emoji map for todo status - pending, done
#[derive(Serialize, Deserialize, Debug)]
pub enum TodoStatusType {
    Pending,
    Done
}

impl TodoStatusType {
    pub fn to_string(&self) -> String {
        return match self {
            TodoStatusType::Pending => format!("{}", "pending"),
            TodoStatusType::Done => format!("{}", "done")
        }
    }
}

// Enum for Colorizing
// Emoji Map Type of Task Priority - 🔥, 🚨, 📅, ☁️, 💡
// TODO: Add more variant for different colors
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StatusColorType {
    Important,
    Today,
    Week,
    Month,
    Info
}

// Enum methods
impl StatusColorType {
    pub fn highlight_color(&self, phrase: String) -> String {
        return match *self {
            StatusColorType::Important => format!("{}", phrase.as_str().red().bold()),
            StatusColorType::Today => format!("{}", phrase.as_str().blue().bold()),
            StatusColorType::Week => format!("{}", phrase.as_str().green().bold()),
            StatusColorType::Month => format!("{}", phrase.as_str().cyan().bold()),
            StatusColorType::Info => format!("{}", phrase.as_str().purple().bold()),
        }
    }
}