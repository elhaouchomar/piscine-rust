use convert_case::{Case, Casing};
use edit_distance::edit_distance;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    if !(compare.is_case(Case::Camel) || compare.is_case(Case::Snake)) {
        return None;
    }

    let distance = edit_distance(&compare.to_lowercase(), &expected.to_lowercase());

    let expected_len = expected.len().max(1); 
    let similarity = 100 - (distance * 100 / expected_len);

    (similarity >= 50).then(|| format!("{similarity}%"))
}
