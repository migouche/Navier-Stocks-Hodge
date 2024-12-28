use std::ops::{Div, Index, IndexMut, Sub};

use super::vector::{Float, Vector};

#[derive(Clone, Copy, Debug)]
pub struct CoordInt<const D: usize>(pub [usize; D]);

pub struct Grid<T, const D: usize> {
    vec: Vec<T>,
    size: CoordInt<D>,
    delta: Float,
}

fn capacity<const D: usize>(size: &CoordInt<D>) -> usize {
    size.0.iter().copied().reduce(|a, b| a * b).unwrap_or(1)
}

impl<T: Default + Clone, const D: usize> Grid<T, D> {
    pub fn new(size: CoordInt<D>, delta: Float) -> Self {
        Grid {
            vec: vec![T::default(); capacity(&size)],
            size: size,
            delta: delta,
        }
    }

    pub fn flatten_index(&self, index: [usize; D]) -> usize {
        index
            .iter()
            .zip(self.size.0.iter())
            .fold(0, |acc, (&i, &dim)| acc * dim + i)
    }

    pub fn get(&self, index: &CoordInt<D>) -> Option<&T> {
        self.vec.get(self.flatten_index(index.0))
    }

    pub fn get_mut(&mut self, index: &CoordInt<D>) -> Option<&mut T> {
        let index = self.flatten_index(index.0);
        self.vec.get_mut(index)
    }
}

impl<const D: usize> Grid<Float, D> {
    pub fn gradient(&self, coord: CoordInt<D>) -> Vector<D> {
        let mut gradient = Vector::<D>::default();
        let coord_val = self.get(&coord).expect("coord not in grid");

        for i in 0..D {
            let modify_coord = |add: bool| {
                coord
                    .0
                    .iter()
                    .zip((0..D).collect::<Vec<usize>>())
                    .map(|(c, c_i)| {
                        if c_i == i {
                            if add {
                                c + 1
                            } else {
                                c - 1
                            }
                        } else {
                            *c
                        }
                    })
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap()
            };

            gradient.0[i] = (self
                .get(&CoordInt::<D>(modify_coord(true)))
                .unwrap_or(coord_val)
                - self
                    .get(&CoordInt::<D>(modify_coord(false)))
                    .unwrap_or(coord_val))
                / (2.0 * self.delta);
        }
        gradient
    }
}

impl<const D: usize> FromIterator<usize> for CoordInt<D> {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut arr = [0; D];
        for i in 0..D {
            arr[i] = iter.next().expect("not enough elements in iterator");
        }
        CoordInt::<D>(arr)
    }
}
