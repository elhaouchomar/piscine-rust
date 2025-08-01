#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}
pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut expected = String::new();
    for c in original.chars() {
        if c.is_ascii_uppercase() {
            let mirored = (b'Z' - (c as u8 - b'A')) as char;
            expected.push(mirored)
        } else if c.is_ascii_lowercase() {
            let mirored = (b'z' - (c as u8 - b'a' )) as char;
            expected.push(mirored);
        } else {
            expected.push(c);
        }
    }

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError {expected})
    }
}