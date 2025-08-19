#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.v <= 1 {
            return None;
        }
        let current = self.v;
        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }
        Some(current)
    }
}

pub fn collatz(n: u64) -> usize {
    if n <= 1 {
        return 0;
    }
    Collatz::new(n).count()
}