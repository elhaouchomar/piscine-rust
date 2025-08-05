pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - b'A';
    let mut lines = Vec::new();

    for i in 0..=n {
        lines.push(build_line(i, n));
    }
    for i in (0..n).rev() {
        lines.push(build_line(i, n));
    }

    lines
}

fn build_line(i: u8, n: u8) -> String {
    let ch = (b'A' + i) as char;
    let outer_spaces = (n - i) as usize;
    let inner_spaces = if i == 0 { 0 } else { (2 * i - 1) as usize };

    if i == 0 {
        format!("{}{}{}", " ".repeat(outer_spaces), ch, " ".repeat(outer_spaces))
    } else {
        format!(
            "{}{}{}{}{}",
            " ".repeat(outer_spaces),
            ch,
            " ".repeat(inner_spaces),
            ch,
            " ".repeat(outer_spaces)
        )
    }
}
