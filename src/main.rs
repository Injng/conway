pub mod controls;
pub mod draw;
pub mod life;
pub mod ui;

use controls::{in_pause, in_play, render_pause, render_play};
use ui::{Cell, render_cell, render_grid, Vector2};
use life::simulate;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};

fn main() {
    // initialize SDL contexts and windows
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem.window("Test", 1280, 720)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();

    // the number of rows and columns to simulate on the backend
    let simulated_rows = 60;
    let simulated_cols = 60;
    let mut cells: Vec<Vec<bool>> = vec![vec![false; simulated_cols]; simulated_rows];

    // if we want to start simulating
    let mut is_simulating = false;

    // render loop
    'running: loop {
        // determine if a grid can be rendered
        let mut is_rendered = true;
        
        // render grid
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        let mut grid_dim: (i32, i32) = (0, 0);  // (rows, cols)
        match render_grid(&mut canvas) {
            Ok(res) => grid_dim = res,
            Err(_) => is_rendered = false,
        }
        
        // simulate Life
        if is_simulating {
            cells = simulate(cells);
        }

        // render cells
        let mut cells_start: (usize, usize) = (0, 0);
        if is_rendered == true {
            // figure out where the grid on the screen maps to the grid on the backend
            let cells_start_x = (simulated_cols / 2) as usize - (grid_dim.1 / 2) as usize;
            let cells_start_y = (simulated_rows / 2) as usize - (grid_dim.0 / 2) as usize;
            cells_start = (cells_start_x, cells_start_y);

            // render all cells in the backend that correspond to the screen
            for i in 0..grid_dim.0 {
                for j in 0..grid_dim.1 {
                    if cells[cells_start_y + i as usize][cells_start_x + j as usize] {
                        let grid_vec = Vector2::new(j, i);
                        let new_cell = Cell::from_grid(grid_vec);
                        render_cell(&mut canvas, new_cell);
                    }
                }
            }
        }

        // render pause and play buttons according to simulation state
        if is_simulating {
            render_pause(&mut canvas);
        } else {
            render_play(&mut canvas);
        }

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running
                },
                Event::MouseButtonUp { x, y, .. } => {
                    if is_rendered == true {
                        // get click and convert to grid coordinates
                        let click_vec = Vector2::new(x, y);
                        let grid_vec = click_vec.to_grid(grid_dim.0, grid_dim.1);

                        // ensure click is within grid and update backend grid
                        if grid_vec.x >= 0 && grid_vec.y >= 0 {
                            let grid_y = cells_start.1 + grid_vec.y as usize;
                            let grid_x = cells_start.0 + grid_vec.x as usize;
                            cells[grid_y][grid_x] = !cells[grid_y][grid_x];
                        }

                        // otherwise, handle control clicks
                        else if is_simulating {
                            if in_pause(&canvas, x, y) { is_simulating = false; }
                        } else if !is_simulating {
                            if in_play(&canvas, x, y) { is_simulating = true; }
                        }
                    }
                },
                _ => {}
            }
        }
        
        canvas.present();
    }
}
