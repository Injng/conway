use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::cmp::min;

const CELL_PADDING: i32 = 1;
pub const BUFFER_SIZE: i32 = 60;

/// Struct to convert between grid coordinates and screen coordinates
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2 { x, y }
    }
    
    /// Convert provided grid coordinates to screen coordinates for the top left corner
    pub fn to_screen(&self, cell_size: i32) -> Vector2 {
        let x: i32 = BUFFER_SIZE + self.x * cell_size;
        let y: i32 = BUFFER_SIZE + self.y * cell_size;
        Vector2 { x, y }
    }

    /// Convert provided screen coordinates in Vector2 to grid coordinates
    /// Returns (-1, -1) if screen coordinates are outside of grid
    pub fn to_grid(&self, cell_rows: i32, cell_cols: i32, cell_size: i32) -> Vector2 {
        // check if self.x is outside of grid
        if self.x <= BUFFER_SIZE || self.x >= cell_cols * cell_size + BUFFER_SIZE {
            return Vector2 { x: -1, y: -1 };
        }
        
        // check if self.y is outside of grid
        if self.y <= BUFFER_SIZE || self.y >= cell_rows * cell_size + BUFFER_SIZE {
            return Vector2 { x: -1, y: -1 };
        }

        // otherwise, calculate grid coordinates
        let x: i32 = (self.x - BUFFER_SIZE) / cell_size;
        let y: i32 = (self.y - BUFFER_SIZE) / cell_size;
        Vector2 { x, y }
    }
}

impl Into<Point> for Vector2 {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}

/// Struct to contain information about all critical points for a cell
pub struct Cell {
    pub top_left: Vector2,
    pub top_right: Vector2,
    pub bottom_left: Vector2,
    pub bottom_right: Vector2,
}

impl Cell {
    pub fn from_top_left(location: Vector2, cell_size: i32) -> Self {
        Cell {
            top_left: Vector2::new(location.x, location.y),
            top_right: Vector2::new(location.x + cell_size, location.y),
            bottom_left: Vector2::new(location.x, location.y + cell_size),
            bottom_right: Vector2::new(location.x + cell_size, location.y + cell_size),
        }
    }

    pub fn from_grid(location: Vector2, cell_size: i32) -> Self {
        let screen_loc: Vector2 = location.to_screen(cell_size);
        Cell {
            top_left : Vector2::new(screen_loc.x, screen_loc.y),
            top_right: Vector2::new(screen_loc.x + cell_size, screen_loc.y),
            bottom_left: Vector2::new(screen_loc.x, screen_loc.y + cell_size),
            bottom_right: Vector2::new(screen_loc.x + cell_size, screen_loc.y + cell_size),
        }
    }
}

/// Given the canvas context and a Cell, render a cell within the grid
pub fn render_cell(canvas: &mut Canvas<Window>, cell: Cell, cell_size: i32) {
    canvas.set_draw_color(Color::GRAY);
    let cell_start_x: i32 = cell.top_left.x + CELL_PADDING;
    let cell_start_y: i32 = cell.top_left.y + CELL_PADDING;
    let cell_dim: u32 = (cell_size - 2 * CELL_PADDING + 1) as u32;
    let cell_rect = Rect::new(cell_start_x, cell_start_y, cell_dim, cell_dim);
    canvas.fill_rect(cell_rect).unwrap();
}

/// Given the canvas context, render the grid
/// Returns a Result containing (rows, cols)
pub fn render_grid(
    canvas: &mut Canvas<Window>, 
    cell_size: i32,
    max_rows: i32,
    max_cols: i32,
) -> Result<(i32, i32), String> {
    // get screen size and set draw color
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::BLACK);
    
    // calculate number of cells from cell size and available screen size
    // first, ensure screen size is large enough
    let minimal_screen_size: u32 = 2 * BUFFER_SIZE as u32 + cell_size as u32;
    if screen_size.0 < minimal_screen_size || screen_size.1 < minimal_screen_size {
        eprintln!("WARNING: Screen not large enough to render grid");
        return Err("Screen not large enough to render grid".to_string());
    }
    // then, calculate available space for cells
    let available_width: i32 = screen_size.0 as i32 - 2 * BUFFER_SIZE;
    let available_height: i32 = screen_size.1 as i32 - 2 * BUFFER_SIZE;
    let rows: i32 = min(max_rows, available_height / cell_size);
    let cols: i32 = min(max_cols, available_width / cell_size);

    // draw the lines for the rows
    for i in 0..=rows {
        let start_point: Vector2 = Cell::from_top_left(Vector2::new(0, i as i32)
            .to_screen(cell_size), cell_size)
            .top_left;
        let end_point: Vector2 = Cell::from_top_left(Vector2::new(cols - 1, i as i32)
            .to_screen(cell_size), cell_size)
            .top_right;
        canvas.draw_line(start_point, end_point).unwrap();
    }

    // draw the lines for the columns
    for i in 0..=cols {
        let start_point: Vector2 = Cell::from_top_left(Vector2::new(i as i32, 0)
            .to_screen(cell_size), cell_size)
            .top_left;
        let end_point: Vector2 = Cell::from_top_left(Vector2::new(i as i32, rows - 1)
            .to_screen(cell_size), cell_size)
            .bottom_left;
        canvas.draw_line(start_point, end_point).unwrap();
    }

    Ok((rows, cols))
}

