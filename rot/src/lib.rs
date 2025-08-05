pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let base = if c.is_ascii_lowercase() {
            b'a'
        } else if c.is_ascii_uppercase() {
            b'A'
        } else {
            result.push(c);
            continue;
        };

        let rotated = ((c as u8 - base) as i8 + key).rem_euclid(26);
        result.push((base + rotated as u8) as char);
    }

    result
}


