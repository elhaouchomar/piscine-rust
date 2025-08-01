mod err;
use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;
use json::{parse, JsonValue};

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
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        let parsed: JsonValue = parse(&content)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;

        let title = parsed["title"]
            .as_str()
            .ok_or_else(|| Box::new(ParseErr::Malformed("missing title".into())) as Box<dyn Error>)?
            .to_string();

        let tasks_json = &parsed["tasks"];
        if !tasks_json.is_array() {
            return Err(Box::new(ParseErr::Malformed("tasks is not array".into())));
        }

        if tasks_json.members().count() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks = Vec::new();
        for t in tasks_json.members() {
            let id = t["id"]
                .as_u32()
                .ok_or_else(|| Box::new(ParseErr::Malformed("invalid id".into())))?;
            let description = t["description"]
                .as_str()
                .ok_or_else(|| Box::new(ParseErr::Malformed("invalid description".into())))?
                .to_string();
            let level = t["level"]
                .as_u32()
                .ok_or_else(|| Box::new(ParseErr::Malformed("invalid level".into())))?;

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}
