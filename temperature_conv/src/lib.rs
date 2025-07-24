pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32 as f64) / (9f64/5f64)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9f64/5f64) + 32f64
}