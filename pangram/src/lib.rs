use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let letters: HashSet<char> = s
    .chars()
    .filter(|c| c.is_ascii_alphabetic())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    letters.len() == 26
}