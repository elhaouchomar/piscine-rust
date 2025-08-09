#[derive(Debug,PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]

pub struct WorkEnvironment {
    pub grade: Link,
}

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|boxed_worker| {
            self.grade = boxed_worker.next;
            boxed_worker.name
        })
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade
            .as_ref()
            .map(|worker| (worker.name.clone(), worker.role.clone()))
    }
}