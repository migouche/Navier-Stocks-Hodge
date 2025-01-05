use super::{grid::Grid, vector::{Float, Vector}};

struct Simulation<const D: usize>{
    pub densities: Grid<Float, D>,
    pub velocities: Grid<Vector<D>, D>,
}