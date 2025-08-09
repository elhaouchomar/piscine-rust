use std::{cell::RefCell, rc::Rc};

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            max,
        }
    }

    pub fn set_value(&self, val: &Rc<i32>) {
        let count = Rc::strong_count(val);
        let percent = ((count as f64 / self.max as f64) * 100.0).floor() as i32;

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else if percent > 100 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                percent
            ));
        } else if percent > 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                percent
            ));
        }
    }

    pub fn peek(&self, val: &Rc<i32>) {
        let count = Rc::strong_count(val);
        let percent = ((count as f64 / self.max as f64) * 100.0).floor() as i32;
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percent
        ));
    }
}
