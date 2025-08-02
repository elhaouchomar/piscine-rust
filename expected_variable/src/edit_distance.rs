
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