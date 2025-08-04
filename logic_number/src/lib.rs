pub fn number_logic(mut num: u32) -> bool {
    let mut temp = num;
    let mut digits = 0;

    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

    let mut sum = 0;
    temp = num;

    while temp > 0 {
        let digit = temp % 10;
        sum += digit.pow(digits);
        temp /= 10;
    }

    sum == num
}
