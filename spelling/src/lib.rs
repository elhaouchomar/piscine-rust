
pub fn spell(n: u64) -> String {
    let below_20 = [
        "zero", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine", "ten", "eleven", "twelve",
        "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    if n == 1_000_000 {
        return "one million".to_string();
    }

    if n < 20 {
        return below_20[n as usize].to_string();
    }
    if n < 100 {
        let t = (n / 10) as usize;
        let r = (n % 10) as usize;
        if r == 0 {
            return tens[t].to_string();
        } else {
            return format!("{}-{}", tens[t], below_20[r]);
        }
    }
    if n < 1000 {
        let h = n / 100;
        let r = n % 100;
        if r == 0 {
            return format!("{} hundred", below_20[h as usize]);
        } else {
            return format!("{} hundred {}", below_20[h as usize], spell(r));
        }
    }

    let thousands = n / 1000;
    let rest = n % 1000;

    if rest == 0 {
        return format!("{} thousand", spell(thousands));
    } else {
        return format!("{} thousand {}", spell(thousands), spell(rest));
    }
}
