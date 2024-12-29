use super::vector::{Float, Vector};

pub type Int = i32;
#[derive(Clone, Copy, Debug)]
pub struct CoordInt<const D: usize>(pub [Int; D]);

pub struct Grid<T, const D: usize> {
    vec: Vec<T>,
    size: CoordInt<D>,
    delta: Float,
}

fn capacity<const D: usize>(size: &CoordInt<D>) -> usize {
    size.0.iter().copied().reduce(|a, b| a * b).unwrap_or(1) as usize
}

impl<T: Default + Clone, const D: usize> Grid<T, D> {
    pub fn new(size: CoordInt<D>, delta: Float) -> Self {
        Grid {
            vec: vec![T::default(); capacity(&size)],
            size: size,
            delta: delta,
        }
    }

    pub fn flatten_index(&self, index: &CoordInt<D>) -> usize {
        index
            .0
            .iter()
            .zip(self.size.0.iter())
            .fold(0, |acc, (&i, &dim)| acc * dim as usize + i as usize)
    }

    pub fn get(&self, index: &CoordInt<D>) -> Option<&T> {
        if index
            .0
            .iter()
            .zip(self.size.0.iter())
            .any(|(&i, &dim)| i >= dim as Int || i < 0)
        {
            None
        } else {
            self.vec.get(self.flatten_index(index))
        }
    }

    pub fn get_mut(&mut self, index: &CoordInt<D>) -> Option<&mut T> {
        let index = self.flatten_index(index);
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
                    .collect::<Vec<Int>>()
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

impl<const D: usize> Grid<Vector<D>, D> {
    pub fn divergence(&self, coord: CoordInt<D>) -> Float {
        let mut divergence = 0.0;
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
                    .collect::<Vec<Int>>()
                    .try_into()
                    .unwrap()
            };

            divergence += (self
                .get(&CoordInt::<D>(modify_coord(true)))
                .unwrap_or(&coord_val)
                .0[i]
                - self
                    .get(&CoordInt::<D>(modify_coord(false)))
                    .unwrap_or(&coord_val)
                    .0[i])
                / (2.0 * self.delta);
        }
        divergence
    }
}

impl<T: Default + Clone, const D: usize> Grid<T, D> {
    // laplace
}

impl<const D: usize> TryFrom<Vec<usize>> for CoordInt<D> {
    type Error = &'static str;

    fn try_from(vec: Vec<usize>) -> Result<Self, Self::Error> {
        if vec.len() == D {
            let arr: [usize; D] = vec.try_into().map_err(|_| "Size mismatch")?;
            let arr: [Int; D] = arr.map(|x| x as Int);
            Ok(CoordInt(arr))
        } else {
            Err("Size mismatch")
        }
    }
}

impl<const D: usize> FromIterator<usize> for CoordInt<D> {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let vec: Vec<usize> = iter.into_iter().collect();
        vec.try_into().expect("Size mismatch")
    }
}

