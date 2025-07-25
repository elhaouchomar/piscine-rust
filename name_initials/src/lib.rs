pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::new();

    for name in names {
        let words: Vec<&str> = name.split_whitespace().collect();
        let mut initials = String::new();
        for i in 0..words.len() {
            if i > 0 {
                initials.push(' ');
            }
            initials.push(words[i].chars().next().unwrap());
            initials.push('.')
        }
        res.push(initials);
    }
    res
}