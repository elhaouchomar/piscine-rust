use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Copy + Clone + PartialEq + Sized {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    
    fn zero() -> Self::Item {
        0u32
    }
    
    fn one() -> Self::Item {
        1u32
    }
}

impl Scalar for u64 {
    type Item = u64;
    
    fn zero() -> Self::Item {
        0u64
    }
    
    fn one() -> Self::Item {
        1u64
    }
}

impl Scalar for i32 {
    type Item = i32;
    
    fn zero() -> Self::Item {
        0i32
    }
    
    fn one() -> Self::Item {
        1i32
    }
}

impl Scalar for i64 {
    type Item = i64;
    
    fn zero() -> Self::Item {
        0i64
    }
    
    fn one() -> Self::Item {
        1i64
    }
}

impl Scalar for f32 {
    type Item = f32;
    
    fn zero() -> Self::Item {
        0.0f32
    }
    
    fn one() -> Self::Item {
        1.0f32
    }
}

impl Scalar for f64 {
    type Item = f64;
    
    fn zero() -> Self::Item {
        0.0f64
    }
    
    fn one() -> Self::Item {
        1.0f64
    }
}