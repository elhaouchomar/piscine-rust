pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound => match server {
            Ok(val) => val.to_string(),
            Err(e) => format!("Not found: {}", e),
        },
        Security::UnexpectedUrl => match server {
            Err(e) => e.to_string(),
            Ok(url) => panic!("{}", url),
        },
    }
}
