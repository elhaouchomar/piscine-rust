pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = (c.abs() as f64).ln();
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let res = a
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .map(|n| n.exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
        (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res = b
        .iter()
        .map(|&n| (n.abs() as f64).ln())
        .collect::<Vec<f64>>();
        (b, res)
}