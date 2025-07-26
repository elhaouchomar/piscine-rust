pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            }
            '+' => {
                let mut count = 1; 
                while chars.peek() == Some(&'+') {
                    chars.next();
                    count += 1;
                }
                for _ in 0..count {
                    chars.next(); 
                }
            }
            _ => result.push(c), 
        }
    }

    *s = result.into_iter().collect();
}
pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        let result = if expr.contains('+') {
            expr.split('+')
                .map(|n| n.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        } else {
            let nums: Vec<i32> = expr.split('-')
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect();
            nums[0] - nums[1]
        };
        *expr = result.to_string();
    }
}
