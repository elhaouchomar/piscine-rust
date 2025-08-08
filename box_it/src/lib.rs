pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
    .filter_map(|tok| {
        let has_k = tok.ends_with('k');
        let cleaned = tok.trim_matches(|c| c == 'k' );
        match cleaned.parse::<f32>() {
            Ok(nb) => {
                let scaled = if !has_k  {
                    nb as u32
                }else {
                     (nb * 1000.0) as u32 
                };
                Some(Box::new(scaled))
            }
            Err(_) => None,
        }
    })
    .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter()
    .map(|boxed_val| *boxed_val)
    .collect()
}
