extern crate minifb;

use crate::simulation::{
    self,
    grid::{CoordInt, Int},
};

pub const WIDTH: usize = 20;
pub const HEIGHT: usize = 20;
pub const CELL_SIZE: usize = 32;

pub struct FluidSimulation {
    grid: simulation::grid::Grid<f32, 2>,
    width: usize,
    height: usize,
}

impl FluidSimulation {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: simulation::grid::Grid::new(
                simulation::grid::CoordInt::<2>([width as Int, height as Int]),
                1.0,
            ),
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        // Update the fluid simulation here
    }

    pub fn draw(&self, buffer: &mut [u32], window_width: usize, window_height: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let density = self.grid.get(&CoordInt([x as Int, y as Int]));
                let color = density
                    .map(|d| self.density_to_color(*d))
                    .unwrap_or(0xff0000ff);
                self.draw_cell(buffer, x, y, color, window_width, window_height);
            }
        }
    }

    fn draw_cell(
        &self,
        buffer: &mut [u32],
        x: usize,
        y: usize,
        color: u32,
        window_width: usize,
        window_height: usize,
    ) {
        let start_x = x * CELL_SIZE;
        let start_y = y * CELL_SIZE;
        for dy in 0..CELL_SIZE {
            for dx in 0..CELL_SIZE {
                let px = start_x + dx;
                let py = start_y + dy;
                if px < window_width && py < window_height {
                    buffer[py * window_width + px] = color;
                }
            }
        }
    }

    pub fn density_to_color(&self, density: f32) -> u32 {
        let intensity = (density * 255.0) as u32;
        (intensity << 16) | (intensity << 8) | intensity
    }

    pub fn handle_mouse(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            //(self.grid[&Coordint[x, y]] + 0.05).min(1.0);
            //self.grid.get_mut(&CoordInt([x, y])) = self.grid.get(&CoordInt([x, y])).map(|v| (v + 0.05).min(1.0));
            if let Some(value) = self.grid.get_mut(&CoordInt([x as Int, y as Int])) {
                *value = (*value + 0.05).min(1.0);
            }
        }
    }
}
