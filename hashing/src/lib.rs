use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum(); 
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> f64 {
    let mut sorted = list.to_vec();
    sorted.sort();

    let len = sorted.len();
    let result = if len % 2 == 1 {
        sorted[len / 2] as f64
    } else {
        let mid1 = sorted[len / 2];
        let mid2 = sorted[len / 2 - 1];
        (mid1 as f64 + mid2 as f64) / 2.0
    };

    if result.fract() == 0.0 {
        result.trunc()
    } else {
        result
    }
}



pub fn mode(list: &[i32]) -> i32 {
    let mut freq = HashMap::new();

    for &num in list {
        *freq.entry(num).or_insert(0) += 1;
    }

    freq.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .unwrap_or(0)
}