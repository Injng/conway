use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};

const CELL_SIZE: i32 = 30;
const BUFFER_SIZE: i32 = 60;
const MINIMAL_SCREEN_SIZE: u32 = 2 * BUFFER_SIZE as u32 + CELL_SIZE as u32;

/// Struct to convert between grid coordinates and screen coordinates
struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Self {
        Vector2 { x, y }
    }

    fn to_screen(&self) -> Vector2 {
        let x: i32 = BUFFER_SIZE + self.x * CELL_SIZE;
        let y: i32 = BUFFER_SIZE + self.y * CELL_SIZE;
        Vector2 { x, y }
    }   
}

impl Into<Point> for Vector2 {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}

/// Struct to contain information about all critical points for a cell
struct Cell {
    top_left: Vector2,
    top_right: Vector2,
    bottom_left: Vector2,
    bottom_right: Vector2,
}

impl Cell {
    fn from_top_left(location: Vector2) -> Self {
        Cell {
            top_left: Vector2::new(location.x, location.y),
            top_right: Vector2::new(location.x + CELL_SIZE, location.y),
            bottom_left: Vector2::new(location.x, location.y + CELL_SIZE),
            bottom_right: Vector2::new(location.x + CELL_SIZE, location.y + CELL_SIZE),
        }
    }
}

/// Given the canvas context, render the grid
fn render_grid(canvas: &mut Canvas<Window>) {
    // get screen size and set draw color
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::BLACK);
    
    // calculate number of cells from cell size and available screen size
    // first, ensure screen size is large enough
    if screen_size.0 < MINIMAL_SCREEN_SIZE || screen_size.1 < MINIMAL_SCREEN_SIZE {
        eprintln!("WARNING: Screen not large enough to render grid");
        return;
    }
    // then, calculate available space for cells
    let available_width: i32 = screen_size.0 as i32 - 2 * BUFFER_SIZE;
    let available_height: i32 = screen_size.1 as i32 - 2 * BUFFER_SIZE;
    let rows: i32 = available_height / CELL_SIZE;
    let cols: i32 = available_width / CELL_SIZE;

    // draw the lines for the rows
    for i in 0..=rows {
        let start_point: Vector2 = Cell::from_top_left(Vector2::new(0, i as i32)
            .to_screen())
            .top_left;
        let end_point: Vector2 = Cell::from_top_left(Vector2::new(cols - 1, i as i32)
            .to_screen())
            .top_right;
        canvas.draw_line(start_point, end_point).unwrap();
    }

    // draw the lines for the columns
    for i in 0..=cols {
        let start_point: Vector2 = Cell::from_top_left(Vector2::new(i as i32, 0)
            .to_screen())
            .top_left;
        let end_point: Vector2 = Cell::from_top_left(Vector2::new(i as i32, rows - 1)
            .to_screen())
            .bottom_left;
        canvas.draw_line(start_point, end_point).unwrap();
    }        
}


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

    // render loop
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        render_grid(&mut canvas);

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        canvas.present();
    }
}
