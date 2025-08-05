pub fn pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let chars: Vec<char> = word.chars().collect();

    match chars[..] {
        [first, ..] if vowels.contains(&first) => format!("{}ay", word),

        [_, 'q', 'u', ..] => {
            let mut result = String::new();
            for c in chars[3..].iter() {
                result.push(*c);
            }
            result.push(chars[0]);
            result.push_str("quay");
            result
        }

        _ => {
            let mut i = 0;
            while i < chars.len() && !vowels.contains(&chars[i]) {
                i += 1;
            }

            let mut result = String::new();
            for c in chars[i..].iter() {
                result.push(*c);
            }
            for c in chars[..i].iter() {
                result.push(*c);
            }
            result.push_str("ay");
            result
        }
    }
}

