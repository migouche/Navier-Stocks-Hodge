use std::ops::{Div, Index, IndexMut, Sub};

use super::vector::{Float, Vector};

#[derive(Clone, Copy, Debug)]
pub struct CoordInt<const D: usize>(pub [usize; D]);

pub struct Grid<T, const D: usize> {
    vec: Vec<T>,
    size: CoordInt::<D>,
    delta: Float
}

fn capacity<const D: usize>(size: &CoordInt::<D>) -> usize{
    size.0.iter().copied().reduce(|a, b| a * b).unwrap_or(1)
}

impl<T: Default + Clone, const D: usize> Grid<T, D>{
    pub fn new(size: CoordInt::<D>, delta: Float) -> Self{
        Grid{
            vec: vec![T::default(); capacity(&size)],
            size: size,
            delta: delta
        }
    }

    pub fn flatten_index(&self, index: [usize; D]) -> usize {
        index.iter().zip(self.size.0.iter()).fold(0, |acc, (&i, &dim)| acc * dim + i)
    }

    pub fn get(&self, index: &CoordInt::<D>) -> Option<&T>{
        self.vec.get(self.flatten_index(index.0))
    }

    pub fn get_mut(&mut self, index: &CoordInt::<D>) -> Option<&mut T>{
        let index = self.flatten_index(index.0);
        self.vec.get_mut(index)
    }

    
}

impl <const D: usize> Grid<Float, D>{
    pub fn gradient(&self, coord: CoordInt<D>) -> Vector<D>{
        let mut gradient = Vector::<D>::default();
        let coord_val = self.get(&coord).expect("coord not in grid");
        for i in 0..D{
            let mut coord_plus = coord;
            coord_plus.0[i]+=1;
            // lets fix this thing one day
            //let coord_plus_better = CoordInt::<D> (coord.0.iter().zip((0..D).collect()).map(|(c, c_i) | c+{if i==c_i {1} else{0}}).collect());
            let mut coord_minus = coord;
            coord_minus.0[i]-=1;
            // (self[&coord_plus].unwrap_or(coord_val) - self[&coord_minus].unwrap_or(coord_val)) / (2.0 * self.delta);
            gradient.0[i] = (self.get(&coord_plus).unwrap_or(coord_val) - self.get(&coord_minus).unwrap_or(coord_val)) / (2.0 * self.delta);
        }
        gradient
    }
}



// gradient = grid([..]).unwrap_or(0)