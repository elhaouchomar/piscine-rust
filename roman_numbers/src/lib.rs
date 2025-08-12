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
    fn from(num: u32) -> Self {
        match num {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid roman digit value"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut digits = Vec::new();

        while num >= 1000 {
            digits.push(M);
            num -= 1000;
        }

        if num >= 900 {
            digits.push(C);
            digits.push(M);
            num -= 900;
        }

        if num >= 500 {
            digits.push(D);
            num -= 500;
        }

        if num >= 400 {
            digits.push(C);
            digits.push(D);
            num -= 400;
        }

        while num >= 100 {
            digits.push(C);
            num -= 100;
        }

        if num >= 90 {
            digits.push(X);
            digits.push(C);
            num -= 90;
        }

        if num >= 50 {
            digits.push(L);
            num -= 50;
        }

        if num >= 40 {
            digits.push(X);
            digits.push(L);
            num -= 40;
        }

        while num >= 10 {
            digits.push(X);
            num -= 10;
        }

        if num >= 9 {
            digits.push(I);
            digits.push(X);
            num -= 9;
        }

        if num >= 5 {
            digits.push(V);
            num -= 5;
        }

        if num >= 4 {
            digits.push(I);
            digits.push(V);
            num -= 4;
        }

        while num >= 1 {
            digits.push(I);
            num -= 1;
        }

        RomanNumber(digits)
    }
}