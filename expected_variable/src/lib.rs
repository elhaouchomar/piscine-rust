pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let compare_lower = compare.to_lowercase();
    let expected_lower = expected.to_lowercase();
    
    if !is_camel_case(&compare_lower) && !is_snake_case(&compare_lower) {
        return None;
    }
    
    let distance = edit_distance(&compare_lower, &expected_lower);
    
    let expected_len = expected_lower.len();
    if expected_len == 0 {
        return if distance == 0 { Some("100%".to_string()) } else { None };
    }
    
    let similarity_ratio = 1.0 - (distance as f64 / expected_len as f64);
    let percentage = (similarity_ratio * 100.0).round() as i32;
    
    if percentage > 50 {
        Some(format!("{}%", percentage))
    } else {
        None
    }
}

fn is_camel_case(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    
    if chars.is_empty() {
        return false;
    }
    
    if chars[0].is_uppercase() {
        return false;
    }
    
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_letter = false;
    
    for &ch in chars.iter() {
        if ch.is_alphabetic() {
            has_letter = true;
            if ch.is_lowercase() {
                has_lowercase = true;
            } else if ch.is_uppercase() {
                has_uppercase = true;
            }
        } else if !ch.is_numeric() && ch != '_' {
            return false;
        }
    }

    has_letter && (has_lowercase && (has_uppercase || chars.len() > 0))
}

fn is_snake_case(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    
    if chars.is_empty() {
        return false;
    }
    
    let mut has_letter = false;
    
    for &ch in chars.iter() {
        if ch.is_alphabetic() {
            if ch.is_uppercase() {
                return false; 
            }
            has_letter = true;
        } else if !ch.is_numeric() && ch != '_' {
            return false;
        }
    }
    
    has_letter
}

pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            
            let deletion = matrix[i - 1][j] + 1;
            let insertion = matrix[i][j - 1] + 1;
            let substitution = matrix[i - 1][j - 1] + cost;
            
            let mut min_val = deletion;
            if insertion < min_val {
                min_val = insertion;
            }
            if substitution < min_val {
                min_val = substitution;
            }
            
            matrix[i][j] = min_val;
        }
    }
    
    matrix[len1][len2]
}