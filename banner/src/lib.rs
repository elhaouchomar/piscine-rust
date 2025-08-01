use std::{collections::HashMap, num::ParseFloatError};

#[derive(Debug)]
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}




impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand.clone(), func);
        self.flags.insert(flag.long_hand.clone(), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        match self.flags.get(input) {
            Some(call) => {
                if argv.len() > 2 {
                    return Err("Not enough arguments".to_string());
            }
            call(argv[0], argv[1]).map_err(|e| e.to_string())
        }
        None => Err("Flag not found".to_string()),
    }
}
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;

    Ok((a /b ).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
        let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;

    Ok((a %b ).to_string())
}