use std::{
    iter::Product, ops::{Add, Div, Mul, Sub}, path::Iter, process::Output
};

use super::vector::{Float, Vector};

pub type Int = i32;
#[derive(Clone, Copy, Debug)]
pub struct CoordInt<const D: usize>(pub [Int; D]);

impl<const D: usize> Default for CoordInt<D> {
    fn default() -> Self {
        CoordInt([0; D])
    }
}

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

impl<
        T: Default + Clone + Add<Output = T> + Mul<Float, Output = T> + Product<Float>,
        const D: usize,
    > Grid<T, D>
{
    pub fn get_at(&self, pos: &Vector<D>) -> T {
        // interpolate D-dimensionally between the 2^D closest points
        let mut index = CoordInt::<D>::default();
        let mut weights = [0.0; D];
        for i in 0..D {
            let coord = (pos.0[i] / self.delta)
                .max(0.0)
                .min(self.size.0[i] as Float - 1.0);
            let lower = (coord.floor() as Int).min(self.size.0[i] - 2);

            index.0[i] = lower;
            weights[i] = coord - lower as Float;
        }

        let mut sum = T::default();
        for i in 0..1 << D {
            let mut index = index.clone();
            for j in 0..D {
                if i & (1 << j) != 0 {
                    index.0[j] += 1;
                }
            }
            let weight: Float = (0..D)
                .map(|j| {
                    if i & (1 << j) != 0 {
                        weights[j]
                    } else {
                        1.0 - weights[j]
                    }
                })
                .product();
            sum = sum + self.get(&index).unwrap().clone() * weight;
        }
        sum
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

impl<
        T: Default
            + Clone
            + Mul<Float, Output = T>
            + Sub<Output = T>
            + Div<Float, Output = T>
            + Add<Output = T>,
        const D: usize,
    > Grid<T, D>
where
    for<'a> &'a T: Add<Output = T>,
{
    // laplace
    pub fn laplace(&self, coord: CoordInt<D>) -> T {
        let mut acc = T::default();
        let coord_val = self.get(&coord).expect("coord not in grid");
        for i in 0..D {
            let modify_coord = |add: Int| {
                coord
                    .0
                    .iter()
                    .zip((0..D).collect::<Vec<usize>>())
                    .map(|(c, c_i)| if c_i == i { c + add } else { *c })
                    .collect::<Vec<Int>>()
                    .try_into()
                    .unwrap()
            };

            acc = acc
                + self
                    .get(&CoordInt::<D>(modify_coord(1)))
                    .unwrap_or(coord_val)
                    .clone()
                + self
                    .get(&CoordInt::<D>(modify_coord(-1)))
                    .unwrap_or(coord_val)
                    .clone();
        }
        (acc - coord_val.clone() * 2.0 * D as Float) / (self.delta * self.delta)
    }
}

impl<
        T: Default + Clone + Add<Output = T> + Mul<Float, Output = T> + Product<Float>,
        const D: usize,
    > Grid<T, D>
{
    pub fn advect(&self, velocity: &Grid<Vector<D>, D>, coord: CoordInt<D>, dt: Float) -> T {
        let velocity = velocity.get(&coord).expect("coord not in grid");
        let new_pos = Vector::from_coord_int(coord, self.delta) - velocity.clone() * dt;
        self.get_at(&new_pos)
    }
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

pub struct GridIter<'b, T: Clone + Default, const D: usize> {
    grid: &'b Grid<T, D>,
    coord: CoordInt<D>,
    start: bool
}

impl<'a, T: Clone + Default, const D: usize> Iterator for GridIter<'a, T, D> {
    type Item = (CoordInt<D>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        println!("coord: {:?}", self.coord);
        if !self.start {
            self.start = true;
            return self.grid.get(&self.coord).map(|r| (self.coord, r));
        }

        self.coord.0[D - 1] += 1;
        for i in (0..D).rev() {
            if self.coord.0[i] >= self.grid.size.0[i] {
                if i == 0 {
                    return None;
                }
                self.coord.0[i] = 0;
                self.coord.0[i - 1] += 1;
            }
        }
        self.grid.get(&self.coord).map(|r| (self.coord, r))
    }
}

impl<'a, T: Clone + Default, const D: usize> IntoIterator for &'a Grid<T, D> {
    type Item = (CoordInt<D>, &'a T);
    type IntoIter = GridIter<'a, T, D>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            coord: CoordInt::<D>::default(),
            start: false
        }
    }
}