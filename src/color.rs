extern crate rand;

use crate::complex::*;
use crate::matrix::*;
use rand::prelude::*;
use std::default::Default;

// TODO(indutny): tweak me
const SPREAD: f64 = 0.5;

const EPSILON: f64 = 1e-23;

pub struct Color {
    rng: ThreadRng,
}

impl Default for Color {
    fn default() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Color {
    pub fn gen(&mut self) -> Matrix {
        let r2 = self.gen_su2();
        let s2 = self.gen_su2();
        let t2 = self.gen_su2();

        let zero = Complex::from(0.0);
        let one = Complex::from(1.0);

        let r = Matrix::from(&[r2[0], r2[1], zero, r2[2], r2[3], zero, zero, zero, one]);
        let s = Matrix::from(&[s2[0], zero, s2[1], zero, one, zero, s2[2], zero, s2[3]]);
        let t = Matrix::from(&[one, zero, zero, zero, t2[0], t2[1], zero, t2[2], t2[3]]);

        r * s * t
    }

    fn gen_su2(&mut self) -> [Complex; 4] {
        let mut res = [0f64; 4];

        for i in 0..res.len() {
            res[i] = self.rng.gen_range(-0.5, 0.5);
        }

        let norm = (res[1] * res[1] + res[2] * res[2] + res[3] * res[3]).sqrt();
        let rescale = SPREAD / (norm + EPSILON);

        res[0] = (1.0 - SPREAD * SPREAD).sqrt() * res[0].signum();
        res[1] *= rescale;
        res[2] *= rescale;
        res[3] *= rescale;

        return [
            Complex::new(res[0], res[3]),
            Complex::new(res[2], res[1]),
            Complex::new(-res[2], res[1]),
            Complex::new(res[0], -res[3]),
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add() {
        let mut c = Color::default();

        let su3 = c.gen();

        // NOTE: It is not an exact science
        assert_eq!((su3.det().re * 100.0).round() / 100.0, 1.0);
        assert_eq!((su3.det().im * 100.0).round() / 100.0, 0.0);
    }
}
