pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 0 || n == 1 {
        return vec![];
    }

    let mut result = vec![1; n];

    let mut prefix = 1;
    for i in 0..n {
        result[i] = prefix;
        prefix *= arr[i];
    }

    let mut suffix = 1;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= arr[i];
    }

    result
}
