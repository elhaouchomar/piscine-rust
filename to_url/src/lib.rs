pub fn to_url(s: &str) -> String {
    let text = String::from(s);
    text.replace(" ", "%20")
}