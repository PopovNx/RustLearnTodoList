use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoRecord {
   pub(super) id: i32,
   pub(super) title: String,
   pub(super) completed: bool,
   pub(super) created_at: i64,
}

impl std::fmt::Display for TodoRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let completed = if self.completed { "X" } else { " " };
        let timestamp_from_unix = chrono::DateTime::from_timestamp(self.created_at, 0);
        let timestamp_from_unix = timestamp_from_unix.unwrap().format("%Y-%m-%d %H:%M:%S");
        write!(f, "[{:0>3}][{}] {:30} - {}", self.id, completed, self.title, timestamp_from_unix)
    }
}