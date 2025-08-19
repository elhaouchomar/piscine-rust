use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::hash::Hash + Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let len = keys.len().min(values.len());

    keys.iter()
        .zip(values.iter())
        .take(len)
        .collect()
}