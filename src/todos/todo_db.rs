use std::path::Path;

use super::TodoRecord;


#[derive(Debug)]
pub struct TodoDb {
    pub(super) records: Vec<TodoRecord>,
    pub(super) path: String,
}

impl TodoDb {
    pub fn load(path: &str) -> Result<TodoDb, std::io::Error> {
        let mut todo_db = TodoDb {
            records: Vec::new(),
            path: path.to_string(),
        };

        let path = Path::new(path);
        if !path.exists() {
            return Ok(todo_db);
        }
        let mut reader = csv::Reader::from_path(path)?;
        for result in reader.deserialize() {
            todo_db.records.push(result?);
        }
        Ok(todo_db)
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let path = Path::new(path);
        let mut writer = csv::Writer::from_path(path)?;
        for record in &self.records {
            writer.serialize(record)?;
        }
        writer.flush()?;
        Ok(())
    }
}

