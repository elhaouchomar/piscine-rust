#[derive(Debug, Clone, PartialEq)]
pub struct RomanNumber {
    pub value: u64,
    pub digits: Vec<&'static str>,
}

impl RomanNumber {
    pub fn from(n: u64) -> Self {
        RomanNumber {
            value: n,
            digits: RomanNumber::to_roman(n),
        }
    }

    fn to_roman(mut n: u64) -> Vec<&'static str> {
        let mut result = Vec::new();
        let symbols = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        for &(val, sym) in symbols.iter() {
            while n >= val {
                n -= val;
                result.push(sym);
            }
        }
        result
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<RomanNumber> {
        self.value += 1;
        let next_digits = RomanNumber::to_roman(self.value);
        let next_number = RomanNumber {
            value: self.value,
            digits: next_digits,
        };
        Some(next_number)
    }
}
