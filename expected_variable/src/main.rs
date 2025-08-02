use convert_case::{Case, Casing};
use edit_distance::edit_distance;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    if !(compare.is_case(Case::Camel) || compare.is_case(Case::Snake)) {
        return None;
    }

    let expected_len = expected.len();
    if expected_len == 0 {
        return None;
    }

    let distance = edit_distance(&compare.to_lowercase(), &expected.to_lowercase());

    let percentage = (distance * 100) / expected_len;
    let similarity = if percentage > 100 { 0 } else { 100 - percentage };

    (similarity >= 50).then(|| format!("{similarity}%"))
}
