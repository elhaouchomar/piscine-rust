use std::collections::HashMap;


pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    fn count_chars(s: &str) -> HashMap<char, usize> {
        let mut map = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) + 1;
        }
        map
    }

    count_chars(s1) == count_chars(s2)
}
