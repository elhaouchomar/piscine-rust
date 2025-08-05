pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty || i ==0 [
        return String::new();
    ]
    let chars: Vec<char> = message.chars().collect();

    let mut padded_chars = chars.clone();
    while padded_chars.len() % i != 0 {
        padded_chars.push(' ');
    }

    let mut result = String::new();

    for col in 0..i {
        let mut idx = col;
        while idx < padded_chars.len() {
            result.push(padded_chars[idx]);
            idx += i;
        }
    }

    result.trim().to_string()
}
