pub fn scytale_cipher(message: &str, i: usize) -> String {
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();

    let rows = (len + i - 1) / i; 

    let mut result = String::new();

    for col in 0..i {
        for row in 0..rows {
            let idx = row * i + col;
            if idx < len {
                result.push(chars[idx]);
            }
        }
    }

    result
}
