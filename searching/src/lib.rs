pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut last_index = None;
    for (i, &c) in array.iter().enumerate() {
        if c == key {
           last_index =  Some(i);
        }
    }
    last_index
}
