pub fn sum(a: &[i32]) -> i32 {
    let mut total = 0;
    for  nb in a {
        total += nb;
    }
    total
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}