pub fn first_fifty_even_square() -> Vec<i32> {
    let res: Vec<i32> = (1..)
    .into_iter()
    .filter(|n| n % 2 == 0)
    .take(50)
    .map(|nb| nb * nb)
    .collect();
    res
}