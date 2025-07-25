pub fn first_subword(mut s: String) -> String {
    let mut res: String = String::new();
    for (i, c) in s.chars().enumerate() {
        if c == '_' || (c.is_uppercase() && i != 0) {
            break
        }
        res.push(c)
    }
    res
}