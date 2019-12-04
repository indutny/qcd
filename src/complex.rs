use std::default::Default;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn norm_sqr(self: Self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(self: Self) -> f64 {
        self.norm_sqr().sqrt()
    }

    pub fn conj(self: Self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Complex { re: 0.0, im: 0.0 }
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex { re: value, im: 0.0 }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let n = rhs.norm_sqr();
        let result = self * rhs.conj();

        Complex {
            re: result.re / n,
            im: result.im / n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_conjugate() {
        let a = Complex::new(1.0, 2.0);
        let b = a.conj();

        assert_eq!(b.re, 1.0);
        assert_eq!(b.im, -2.0);
    }

    #[test]
    fn it_should_norm_sqr() {
        let a = Complex::new(1.0, 2.0);
        let n = a.norm_sqr();

        assert_eq!(n, 5.0);
    }

    #[test]
    fn it_should_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);

        let c = a + b;

        assert_eq!(c.re, 4.0);
        assert_eq!(c.im, 6.0);
    }

    #[test]
    fn it_should_sub() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);

        let c = a - b;

        assert_eq!(c.re, -2.0);
        assert_eq!(c.im, -2.0);
    }

    #[test]
    fn it_should_multiply() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);

        let c = a * b;

        assert_eq!(c.re, -5.0);
        assert_eq!(c.im, 10.0);
    }

    #[test]
    fn it_should_multiply_conjugates() {
        let a = Complex::new(1.0, 1.0);
        let b = a.conj();

        let c = a * b;

        assert_eq!(c.re, 2.0);
        assert_eq!(c.im, 0.0);
    }

    #[test]
    fn it_should_divide() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(2.0, 1.0);

        let c = a / b;

        assert_eq!(c.re, 0.8);
        assert_eq!(c.im, 0.6);
    }
}
