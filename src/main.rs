mod simulation;
mod ui;

use std::time::{Duration, Instant};

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use ui::ui::{FluidSimulation, CELL_SIZE, HEIGHT, WIDTH};

fn main() {
    let window_width = WIDTH * CELL_SIZE;
    let window_height = HEIGHT * CELL_SIZE;

    let mut window = Window::new(
        "Fluid Simulation",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; window_width * window_height];
    let mut simulation = FluidSimulation::new(WIDTH, HEIGHT);

    let mut last_update = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_update.elapsed() >= Duration::from_millis(16) {
            simulation.update();
            last_update = Instant::now();
        }

        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                simulation.handle_mouse(mouse_x as usize / CELL_SIZE, mouse_y as usize / CELL_SIZE);
            }
        }

        simulation.draw(&mut buffer, window_width, window_height);
        window
            .update_with_buffer(&buffer, window_width, window_height)
            .unwrap();
    }
}
