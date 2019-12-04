use std::default::Default;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::complex::*;

const WIDTH: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix {
    values: [Complex; 9],
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            values: [Complex::from(0.0); 9],
        }
    }
}

impl From<&[f64; 9]> for Matrix {
    fn from(values: &[f64; 9]) -> Self {
        let mut result = Matrix::default();

        for (i, &elem) in values.iter().enumerate() {
            result.values[i] = Complex::from(elem);
        }
        return result;
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Matrix::default();

        for i in 0..result.values.len() {
            result.values[i] = self.values[i] + rhs.values[i];
        }
        return result;
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = Matrix::default();

        for i in 0..result.values.len() {
            result.values[i] = self.values[i] - rhs.values[i];
        }
        return result;
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix::default();

        for row in 0..WIDTH {
            for column in 0..WIDTH {
                let mut acc = Complex::from(0.0);
                for i in 0..WIDTH {
                    acc = acc + self[(row, i)] * rhs[(i, column)];
                }
                result[(row, column)] = acc;
            }
        }

        return result;
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Complex;

    fn index(&self, index: (usize, usize)) -> &Complex {
        let (row, col) = index;

        &self.values[row * WIDTH + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Complex {
        let (row, col) = index;

        &mut self.values[row * WIDTH + col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_result(actual: &Matrix, expected: Vec<f64>) {
        let values: Vec<f64> = actual.values.iter().map(|c| c.re).collect();
        assert_eq!(values, expected);
    }

    #[test]
    fn it_should_add() {
        let a = Matrix::from(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let b = Matrix::from(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);

        let c = a + b;

        check_result(&c, vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9]);
    }

    #[test]
    fn it_should_sub() {
        let a = Matrix::from(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let b = Matrix::from(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);

        let c = a - b;

        check_result(&c, vec![0.9, 1.8, 2.7, 3.6, 4.5, 5.4, 6.3, 7.2, 8.1]);
    }

    #[test]
    fn it_should_mul() {
        let a = Matrix::from(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let b = Matrix::from(&[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0]);

        let c = a * b;

        check_result(
            &c,
            vec![84.0, 90.0, 96.0, 201.0, 216.0, 231.0, 318.0, 342.0, 366.0],
        );
    }
}
