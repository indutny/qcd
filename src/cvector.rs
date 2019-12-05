use crate::complex::*;

use std::default::Default;
use std::ops::{Add, Index, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CVector {
    values: [Complex; 3],
}

impl CVector {
    pub fn new(values: [Complex; 3]) -> Self {
        Self { values }
    }

    pub fn norm_sqr(&self) -> f64 {
        self.values.iter().fold(0.0, |acc, x| acc + x.norm_sqr())
    }

    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }

    pub fn rescale(&self, factor: Complex) -> CVector {
        let mut result = CVector::default();
        for i in 0..self.values.len() {
            result.values[i] = self.values[i] * factor;
        }
        return result;
    }

    pub fn dot(lhs: &CVector, rhs: &CVector) -> Complex {
        let mut value: Complex = Complex::from(0.0);

        for i in 0..lhs.values.len() {
            value = value + lhs.values[i].conj() * rhs.values[i];
        }
        return value;
    }
}

impl Default for CVector {
    fn default() -> Self {
        Self {
            values: [Complex::from(0.0); 3],
        }
    }
}

impl From<&[f64; 3]> for CVector {
    fn from(value: &[f64; 3]) -> Self {
        let mut result = CVector::default();
        for (i, &elem) in value.iter().enumerate() {
            result.values[i] = Complex::from(elem);
        }
        return result;
    }
}

impl Add for CVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = CVector::default();

        for i in 0..result.values.len() {
            result.values[i] = self.values[i] + rhs.values[i];
        }
        return result;
    }
}

impl Sub for CVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = CVector::default();

        for i in 0..result.values.len() {
            result.values[i] = self.values[i] - rhs.values[i];
        }
        return result;
    }
}

impl Index<usize> for CVector {
    type Output = Complex;

    fn index(&self, index: usize) -> &Complex {
        &self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_norm() {
        let a = CVector::from(&[2.0, 3.0, 6.0]);

        assert_eq!(a.norm(), 7.0);
    }

    #[test]
    fn it_should_rescale() {
        let a = CVector::from(&[2.0, 3.0, 6.0]);

        let b = a.rescale(Complex::from(1.0 / 7.0));

        assert_eq!((b.norm() * 100.0).round() / 100.0, 1.0);
        assert_eq!((b[0].norm() * 100.0).round() / 100.0, 0.29);
    }

    #[test]
    fn it_should_dot() {
        let a = CVector::new([
            Complex::new(1.0, 1.0),
            Complex::new(0.0, 1.0),
            Complex::new(1.0, 0.0),
        ]);
        let b = CVector::new([
            Complex::new(2.0, 1.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 1.0),
        ]);

        assert_eq!(CVector::dot(&a, &a).re, a.norm_sqr());
        assert_eq!(CVector::dot(&b, &b).re, b.norm_sqr());

        assert_eq!(CVector::dot(&a, &b).re, 4.0);
        assert_eq!(CVector::dot(&a, &b).im, -1.0);
        assert_eq!(CVector::dot(&b, &a).re, 4.0);
        assert_eq!(CVector::dot(&b, &a).im, 1.0);
    }

    #[test]
    fn it_should_add() {
        let a = CVector::from(&[1.0, 2.0, 3.0]);
        let b = CVector::from(&[4.0, 5.0, 6.0]);

        let c = a + b;
        assert_eq!(c[0].re, 5.0);
        assert_eq!(c[1].re, 7.0);
        assert_eq!(c[2].re, 9.0);
    }

    #[test]
    fn it_should_sub() {
        let a = CVector::from(&[1.0, 2.0, 3.0]);
        let b = CVector::from(&[6.0, 5.0, 4.0]);

        let c = a - b;
        assert_eq!(c[0].re, -5.0);
        assert_eq!(c[1].re, -3.0);
        assert_eq!(c[2].re, -1.0);
    }
}
