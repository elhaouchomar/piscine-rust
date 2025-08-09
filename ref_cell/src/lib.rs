use std::{cell::RefCell, rc::Rc};

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>, 
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value(&self, val: &Rc<i32>) {
        let count = Rc::strong_count(val);
        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            *self.value.borrow_mut() = count;
            let percent = (count as f64 / self.max as f64) * 100.0;
            if percent > 70.0 {
                self.messages
                    .borrow_mut()
                    .push(format!("Warning: You have used up over {:.0}% of your quota!", percent));
            }
        }
    }

    pub fn peek(&self, val: &Rc<i32>) {
        let count = Rc::strong_count(val);
        let percent = (count as f64 / self.max as f64) * 100.0;
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {:.0}% of your quota",
            percent
        ));
    }
}
