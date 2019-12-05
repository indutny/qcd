use crate::color::*;
use crate::complex::*;
use crate::matrix::*;

pub struct Lattice {
    color: Color,
    links: Vec<Matrix>,
}

impl Lattice {
    pub fn cold_with_size(size: usize) -> Self {
        // 4 dimensions, 4 directions
        let len = size * size * size * size * 4;
        let mut links = Vec::with_capacity(len);

        let id = Matrix::identity();
        for i in 0..len {
            links.push(id);
        }

        Self {
            color: Color::default(),
            links,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_init() {
        let lattice = Lattice::cold_with_size(8);
    }
}
