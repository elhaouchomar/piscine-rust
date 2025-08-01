mod err;

pub use err::{ParseErr, ReadErr};
use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        let parsed = json::parse(&content).map_err(|e| {
            Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>
        })?;

        let title = parsed["title"]
            .as_str()
            .ok_or_else(|| Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing or invalid title field"
            )))) as Box<dyn Error>)?
            .to_string();

        let tasks_json = &parsed["tasks"];
        
        if !tasks_json.is_array() {
            return Err(Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing or invalid tasks field"
            )))) as Box<dyn Error>);
        }

        if tasks_json.len() == 0 {
            return Err(Box::new(ParseErr::Empty) as Box<dyn Error>);
        }

        let mut tasks = Vec::new();
        for i in 0..tasks_json.len() {
            let task_json = &tasks_json[i];
            let id = task_json["id"]
                .as_u32()
                .ok_or_else(|| Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid task id"
                )))) as Box<dyn Error>)?;

            let description = task_json["description"]
                .as_str()
                .ok_or_else(|| Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid task description"
                )))) as Box<dyn Error>)?
                .to_string();

            let level = task_json["level"]
                .as_u32()
                .ok_or_else(|| Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid task level"
                )))) as Box<dyn Error>)?;

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}