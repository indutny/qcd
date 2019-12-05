use crate::complex::*;
use crate::matrix::*;

use std::ops::Index;

pub struct Lattice {
    size: usize,
    links: Vec<[Matrix; 4]>,
}

impl Lattice {
    pub fn cold_with_size(size: usize) -> Self {
        // 4 dimensions
        let len = size * size * size * size;
        let mut links = Vec::with_capacity(len);

        let id = Matrix::identity();
        for i in 0..len {
            links.push([id; 4]);
        }

        Self { size, links }
    }

    pub fn sweep<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut Matrix, &[Matrix; 6]),
    {
        for i in 0..self.links.len() {
            let staples = self.compute_staples(i);

            func(&mut self.links[i][0], &staples);
        }
    }

    pub fn compute_staples(&self, i: usize) -> [Matrix; 6] {
        [Matrix::identity(); 6]
    }
}

type Coordinate = (usize, usize, usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_init() {
        let lattice = Lattice::cold_with_size(8);
    }
}
