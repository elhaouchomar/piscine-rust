pub fn number_logic(num: u32) -> bool {
    let mut n = num;
    let check = n.to_string().len();
    let mut res = 0;
    while n > 0 {
       res += (n % 10) as usize * check;
       n /= 10;
    }
    true
}