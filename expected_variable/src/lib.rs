use convert_case::{Case, Casing};

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let compare_lower = compare.to_lowercase();
    let expected_lower = expected.to_lowercase();
    
    if !compare_lower.is_case(Case::Camel) && !compare_lower.is_case(Case::Snake) {
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

pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_len = source.chars().count();
    let target_len = target.chars().count();

    if source_len == 0 {
        return target_len;
    }
    if target_len == 0 {
        return source_len;
    }

    let mut dp = vec![vec![0; target_len + 1]; source_len + 1];

    for i in 0..=source_len {
        dp[i][0] = i;
    }
    for j in 0..=target_len {
        dp[0][j] = j;
    }

    for i in 1..=source_len {
        for j in 1..=target_len {
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };
            dp[i][j] = *[
                dp[i - 1][j] + 1,
                dp[i][j - 1] + 1, 
                dp[i - 1][j - 1] + cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    dp[source_len][target_len]
}   