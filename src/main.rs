use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};

const CELL_SIZE: i32 = 30;
const CELL_PADDING: i32 = 5;
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
    
    /// Convert provided grid coordinates to screen coordinates for the top left corner
    fn to_screen(&self) -> Vector2 {
        let x: i32 = BUFFER_SIZE + self.x * CELL_SIZE;
        let y: i32 = BUFFER_SIZE + self.y * CELL_SIZE;
        Vector2 { x, y }
    }

    /// Convert provided screen coordinates in Vector2 to grid coordinates
    /// Returns (-1, -1) if screen coordinates are outside of grid
    fn to_grid(&self, cell_rows: i32, cell_cols: i32) -> Vector2 {
        // check if self.x is outside of grid
        if self.x <= BUFFER_SIZE || self.x >= cell_cols * CELL_SIZE + BUFFER_SIZE {
            return Vector2 { x: -1, y: -1 };
        }
        
        // check if self.y is outside of grid
        if self.y <= BUFFER_SIZE || self.y >= cell_rows * CELL_SIZE + BUFFER_SIZE {
            return Vector2 { x: -1, y: -1 };
        }

        // otherwise, calculate grid coordinates
        let x: i32 = (self.x - BUFFER_SIZE) / CELL_SIZE;
        let y: i32 = (self.y - BUFFER_SIZE) / CELL_SIZE;
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

    fn from_grid(location: Vector2) -> Self {
        let screen_loc: Vector2 = location.to_screen();
        Cell {
            top_left : Vector2::new(screen_loc.x, screen_loc.y),
            top_right: Vector2::new(screen_loc.x + CELL_SIZE, screen_loc.y),
            bottom_left: Vector2::new(screen_loc.x, screen_loc.y + CELL_SIZE),
            bottom_right: Vector2::new(screen_loc.x + CELL_SIZE, screen_loc.y + CELL_SIZE),
        }
    }
}

/// Given the canvas context and a Cell, render a cell within the grid
fn render_cell(canvas: &mut Canvas<Window>, cell: Cell) {
    canvas.set_draw_color(Color::GRAY);
    let cell_start_x: i32 = cell.top_left.x + CELL_PADDING;
    let cell_start_y: i32 = cell.top_left.y + CELL_PADDING;
    let cell_dim: u32 = (CELL_SIZE - 2 * CELL_PADDING) as u32;
    let cell_rect = Rect::new(cell_start_x, cell_start_y, cell_dim, cell_dim);
    canvas.fill_rect(cell_rect).unwrap();
}

/// Given the canvas context, render the grid
/// Returns a Result containing (rows, cols)
fn render_grid(canvas: &mut Canvas<Window>) -> Result<(i32, i32), String> {
    // get screen size and set draw color
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::BLACK);
    
    // calculate number of cells from cell size and available screen size
    // first, ensure screen size is large enough
    if screen_size.0 < MINIMAL_SCREEN_SIZE || screen_size.1 < MINIMAL_SCREEN_SIZE {
        eprintln!("WARNING: Screen not large enough to render grid");
        return Err("Screen not large enough to render grid".to_string());
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

    Ok((rows, cols))
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

    // the number of rows and columns to simulate on the backend
    let simulated_rows = 60;
    let simulated_cols = 60;
    let mut cells: Vec<Vec<bool>> = vec![vec![false; simulated_cols]; simulated_rows];

    // render loop
    'running: loop {
        // determine if a grid can be rendered
        let mut is_rendered = true;
        
        // render ui
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        let mut grid_dim: (i32, i32) = (0, 0);  // (rows, cols)
        match render_grid(&mut canvas) {
            Ok(res) => grid_dim = res,
            Err(_) => is_rendered = false,
        }

        let mut cells_start: (usize, usize) = (0, 0);
        if is_rendered == true {
            // figure out where the grid on the screen maps to the grid on the backend
            let cells_start_x = (simulated_cols / 2) as usize - (grid_dim.1 / 2) as usize;
            let cells_start_y = (simulated_rows / 2) as usize - (grid_dim.0 / 2) as usize;
            cells_start = (cells_start_x, cells_start_y);

            // render all cells in the backend that correspond to the screen
            for i in 0..grid_dim.1 {
                for j in 0..grid_dim.0 {
                    if cells[cells_start_y + i as usize][cells_start_x + j as usize] {
                        let grid_vec = Vector2::new(j, i);
                        let new_cell = Cell::from_grid(grid_vec);
                        render_cell(&mut canvas, new_cell);
                    }
                }
            }
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
                    }
                },
                _ => {}
            }
        }
        
        canvas.present();
    }
}
