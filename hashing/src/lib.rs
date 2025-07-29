use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum(); 
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> f64 {
    let mut sorted = list.to_vec(); 
    sorted.sort(); 
    let len = sorted.len();
    let mid = len / 2;

    if len % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
    } else {
        sorted[mid] as f64
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