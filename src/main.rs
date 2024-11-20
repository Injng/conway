pub mod controls;
pub mod draw;
pub mod file;
pub mod life;
pub mod text;
pub mod ui;

use std::time::{Duration, Instant};

use controls::{calc_slider, in_pause, in_play, in_slider, in_upload, render_pause, render_play, render_slider, render_upload};
use file::upload;
use sdl2::image::LoadTexture;
use text::TextCache;
use ui::{Cell, render_cell, render_grid, Vector2};
use life::simulate;

use sdl2::event::Event;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::rwops;
use sdl2::ttf::{Font, Sdl2TtfContext, self};
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};

// speed for the simulation
const MIN_SPEED: u64 = 500;
const MAX_SPEED: u64 = 1;
const DEFAULT_SPEED: u64 = 1;

// font byte array
const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/FiraSans-Regular.ttf");

// icon byte arrays
const UPLOAD_BYTES: &[u8] = include_bytes!("../assets/icons/upload.png");

// size of the simulation
const SIMULATED_ROWS: usize = 60;
const SIMULATED_COLS: usize = 60;

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
    let texture_creator = canvas.texture_creator();
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

    // initialize icon textures
    let upload_texture: Texture = texture_creator.load_texture_bytes(UPLOAD_BYTES).unwrap();
    
    // initialize font and text cache
    let ttf_context: Sdl2TtfContext = ttf::init().expect("Failed to init TTF context");
    let font: Font = ttf_context
        .load_font_from_rwops(
            rwops::RWops::from_bytes(FONT_BYTES).unwrap(),
            16,
        ).unwrap();
    let mut text_cache = TextCache::new(&texture_creator, &font);

    // initialize backend simulation grid
    let mut cells: Vec<Vec<bool>> = vec![vec![false; SIMULATED_COLS]; SIMULATED_ROWS];

    // state variables
    let mut is_simulating = false;
    let mut is_slider_moving = false;
    let mut slider_length: f32 = 1.0;

    // keep track of time between loops to update simulation
    let mut last_updated = Instant::now();
    let mut interval = Duration::from_millis(DEFAULT_SPEED);

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
            let curr_time = Instant::now();
            if curr_time.duration_since(last_updated) > interval {
                cells = simulate(cells);
                last_updated = curr_time;
            }
        }

        // render cells
        let mut cells_start: (usize, usize) = (0, 0);
        if is_rendered == true {
            // figure out where the grid on the screen maps to the grid on the backend
            let cells_start_x = (SIMULATED_COLS / 2) as usize - (grid_dim.1 / 2) as usize;
            let cells_start_y = (SIMULATED_ROWS / 2) as usize - (grid_dim.0 / 2) as usize;
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

        // render slider controls for simulation speed
        let speed_text: &str = &interval.as_millis().to_string();
        render_slider(&mut canvas, &mut text_cache, speed_text, slider_length);

        // render upload icon
        render_upload(&mut canvas, &upload_texture);

        // if slider is in moving state, update slider length and set speed
        if is_slider_moving {
            let mouse_state: MouseState = MouseState::new(&event_pump);
            slider_length = calc_slider(mouse_state.x());
            let new_interval =
                ((MIN_SPEED - MAX_SPEED) as f32 * slider_length) as u64;
            interval = Duration::from_millis(MIN_SPEED - new_interval);
        }

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, .. } => {
                    // if initial click is in slider, set slider moving variable to true
                    if is_rendered && in_slider(&canvas, x, y) {
                        is_slider_moving = true;
                        slider_length = calc_slider(x as i32);
                        let new_interval =
                            ((MIN_SPEED - MAX_SPEED) as f32 * slider_length) as u64;
                        interval = Duration::from_millis(MIN_SPEED - new_interval);
                    }
                },
                Event::MouseButtonUp { x, y, .. } => {
                    if is_rendered {
                        // get click and convert to grid coordinates
                        let click_vec = Vector2::new(x, y);
                        let grid_vec = click_vec.to_grid(grid_dim.0, grid_dim.1);

                        // ensure click is within grid and update backend grid
                        if grid_vec.x >= 0 && grid_vec.y >= 0 {
                            let grid_y = cells_start.1 + grid_vec.y as usize;
                            let grid_x = cells_start.0 + grid_vec.x as usize;
                            cells[grid_y][grid_x] = !cells[grid_y][grid_x];
                        }

                        // check upload icon clicks
                        else if in_upload(x, y) {
                            cells = match upload() {
                                Ok(c) => c,
                                Err(e) => {
                                    eprintln!("ERROR: {}", e);
                                    cells
                                },
                            };
                        }

                        // check play button clicks
                        else if is_simulating {
                            if in_pause(&canvas, x, y) { is_simulating = false; }
                        } else if !is_simulating {
                            if in_play(&canvas, x, y) { is_simulating = true; }
                        }

                        // if slider was in moving state, get it out of moving state
                        if is_slider_moving { is_slider_moving = false; }
                    }
                },
                _ => {}
            }
        }
        
        canvas.present();
    }
}
