use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }
        let roman_nb: &[(u32, &[RomanDigit])] = &[
            (1000, &[M]),
            (900,  &[C, M]),
            (500,  &[D]),
            (400,  &[C, D]),
            (100,  &[C]),
            (90,   &[X, C]),
            (50,   &[L]),
            (40,   &[X, L]),
            (10,   &[X]),
            (9,    &[I, X]),
            (5,    &[V]),
            (4,    &[I, V]),
            (1,    &[I]),
        ];
        
        let mut digits = Vec::new();

        for (value, roman_digits) in roman_nb {
            while num >= *value {
                digits.extend_from_slice(roman_digits);
                num -= value;
            }
        }

        RomanNumber(digits)
    }
}


impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0[0] == Nulla {
            return Some(RomanNumber::from(1));
        }
        let roman_nb: &[(u32, &[RomanDigit])] = &[
            (1000, &[M]),
            (900, &[C, M]),
            (500, &[D]),
            (400, &[C, D]),
            (100, &[C]),
            (90, &[X, C]),
            (50, &[L]),
            (40, &[X, L]),
            (10, &[X]),
            (9, &[I, X]),
            (5, &[V]),
            (4, &[I, V]),
            (1, &[I]),
        ];

        let mut n = 0;
        let mut i = 0;
        let digits = &self.0;
        let mut matched = false;

        while i < digits.len() {
            matched = false;

            for (value, roman_digits) in roman_nb {
                if digits[i..].starts_with(roman_digits) {
                    n += value;
                    i += roman_digits.len();
                    matched = true;
                    break;
                }
            }

            if !matched {
                return None;
            }
        }

        if !matched {
            return None;
        }

        Some(RomanNumber::from(n + 1))
    }
}