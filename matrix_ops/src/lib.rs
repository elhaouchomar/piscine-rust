use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }
        let mut result = vec![];
        for (row1, row2) in self.0.iter().zip(rhs.0.iter()) {
            let row: Vec<T> = row1.iter().zip(row2.iter())
                .map(|(&a, &b)| a + b)
                .collect();
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }
        let mut result = vec![];
        for (row1, row2) in self.0.iter().zip(rhs.0.iter()) {
            let row: Vec<T> = row1.iter().zip(row2.iter())
                .map(|(&a, &b)| a - b)
                .collect();
            result.push(row);
        }
        Some(Matrix(result))
    }
}
