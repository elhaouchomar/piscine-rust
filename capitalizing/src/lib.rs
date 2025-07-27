pub fn capitalize_first(input: &str) -> String {
    let mut s = String::new();
    if !input.is_empty() {
        for (i, c) in input.chars().enumerate() {
            if i == 0 {
                s.push_str(&c.to_uppercase().to_string());
            } else {
                s.push(c);
            }
        }
    }
    s
}


pub fn title_case(input: &str) -> String {
    input
        .split_inclusive(|c: char| c.is_ascii_whitespace())
        .map(capitalize_first)
        .collect::<Vec<String>>()
        .concat()
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}