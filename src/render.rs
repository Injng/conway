use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

const CELL_SIZE: i32 = 30;
const CELL_PADDING: i32 = 2;
const BUFFER_SIZE: i32 = 60;
const MINIMAL_SCREEN_SIZE: u32 = 2 * BUFFER_SIZE as u32 + CELL_SIZE as u32;

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
    pub fn to_screen(&self) -> Vector2 {
        let x: i32 = BUFFER_SIZE + self.x * CELL_SIZE;
        let y: i32 = BUFFER_SIZE + self.y * CELL_SIZE;
        Vector2 { x, y }
    }

    /// Convert provided screen coordinates in Vector2 to grid coordinates
    /// Returns (-1, -1) if screen coordinates are outside of grid
    pub fn to_grid(&self, cell_rows: i32, cell_cols: i32) -> Vector2 {
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
pub struct Cell {
    pub top_left: Vector2,
    pub top_right: Vector2,
    pub bottom_left: Vector2,
    pub bottom_right: Vector2,
}

impl Cell {
    pub fn from_top_left(location: Vector2) -> Self {
        Cell {
            top_left: Vector2::new(location.x, location.y),
            top_right: Vector2::new(location.x + CELL_SIZE, location.y),
            bottom_left: Vector2::new(location.x, location.y + CELL_SIZE),
            bottom_right: Vector2::new(location.x + CELL_SIZE, location.y + CELL_SIZE),
        }
    }

    pub fn from_grid(location: Vector2) -> Self {
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
pub fn render_cell(canvas: &mut Canvas<Window>, cell: Cell) {
    canvas.set_draw_color(Color::GRAY);
    let cell_start_x: i32 = cell.top_left.x + CELL_PADDING;
    let cell_start_y: i32 = cell.top_left.y + CELL_PADDING;
    let cell_dim: u32 = (CELL_SIZE - 2 * CELL_PADDING + 1) as u32;
    let cell_rect = Rect::new(cell_start_x, cell_start_y, cell_dim, cell_dim);
    canvas.fill_rect(cell_rect).unwrap();
}

/// Given the canvas context, render the grid
/// Returns a Result containing (rows, cols)
pub fn render_grid(canvas: &mut Canvas<Window>) -> Result<(i32, i32), String> {
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

/// Given three points, render the outline of a triangle abc
pub fn draw_triangle(canvas: &mut Canvas<Window>, a: Vector2, b: Vector2, c: Vector2) {
    canvas.draw_line(a, b).unwrap();
    canvas.draw_line(b, c).unwrap();
    canvas.draw_line(c, a).unwrap();
}

/// Given two points that form a line, and a y-coordinate, interpolate the x-coordinate 
fn interpolate(a: Vector2, b: Vector2, y: i32) -> i32 {
    let m = (b.y - a.y) as f32 / (b.x - a.x) as f32;
    ((y - a.y) as f32 / m) as i32 + a.x
} 

/// Given three points, fill the triangle abc using a scanline algorithm
pub fn fill_triangle(canvas: &mut Canvas<Window>, a: Vector2, b: Vector2, c: Vector2) {
    let mut points: Vec<Vector2> = vec![a, b, c];
    points.sort_by(|first, second| first.y.cmp(&second.y));

    // iterate through each y-level
    for y in points[0].y..=points[2].y {
        // compute active edges
        let left_edge: (Vector2, Vector2);
        let right_edge: (Vector2, Vector2);

        /*
        configuration where the intermediate point is to the right of the lowest point:
         -
         |\
         | \
         |  -
         | /
         |/
         -
         */
        if points[0].x <= points[1].x && y <= points[1].y {
            left_edge = (points[0], points[2]);
            right_edge = (points[0], points[1]);
        } else if points[0].x <= points[1].x {
            left_edge = (points[0], points[2]);
            right_edge = (points[1], points[2]);
        }

        /*
        configuration where the intermediate point is to the left of the lowest point:
            -
           /|
          / |
         -  |
          \ |
           \|
            -
         */
        else if y <= points[1].y {
            left_edge = (points[0], points[1]);
            right_edge = (points[0], points[2]);
        } else {
            left_edge = (points[1], points[2]);
            right_edge = (points[0], points[2]);
        }

        // get start and end points of line to draw
        let start: Vector2 = Vector2::new(interpolate(left_edge.0, left_edge.1, y), y);
        let end: Vector2 = Vector2::new(interpolate(right_edge.0, right_edge.1, y), y);

        // draw the line
        canvas.draw_line(start, end).unwrap();
    }
}

/// Render a play button in the bottom buffer
pub fn render_play(canvas: &mut Canvas<Window>) {
    // get screen size and set draw color
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    let screen_width = screen_size.0 as i32;
    let screen_height = screen_size.1 as i32;
    canvas.set_draw_color(Color::BLACK);

    // dimensions for the play button triangle, using equilateral triangle lengths
    let height: i32 = BUFFER_SIZE / 2;
    let width: i32 = ((height as f32 / 2.0) * (3 as f32).sqrt()) as i32;

    // padding on top and on the bottom for the play button triangle
    let p_top: i32 = (BUFFER_SIZE - height) / 2;
    let p_bottom: i32 = BUFFER_SIZE - p_top - height;

    // points for the play button triangle
    let a = Vector2::new((screen_width - width) / 2, screen_height - BUFFER_SIZE + p_top);
    let b = Vector2::new((screen_width - width) / 2, screen_height - p_bottom);
    let c = Vector2::new((screen_width + width) / 2, screen_height - p_bottom - height / 2);

    // render the triangle
    fill_triangle(canvas, a, b, c);
}

/// Render a pause button in the bottom buffer
pub fn render_pause(canvas: &mut Canvas<Window>) { 
    // get screen size and set draw color
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    let screen_width = screen_size.0 as i32;
    let screen_height = screen_size.1 as i32;
    canvas.set_draw_color(Color::BLACK);

    // dimensions for the pause button, including distance between the two rectangles
    let height: i32 = BUFFER_SIZE / 2;
    let width: i32 = height / 4;
    let distance: i32 = height / 3;

    // padding on top and on the bottom for the pause button
    let p_top: i32 = (BUFFER_SIZE - height) / 2;
    let p_bottom: i32 = BUFFER_SIZE - p_top - height;

    // create the rectangles
    let left_rect = Rect::new((screen_width - distance) / 2 - width,
        screen_height - p_bottom - height,
        width as u32,
        height as u32);
    let right_rect = Rect::new((screen_width + distance) / 2,
        screen_height - p_bottom - height,
        width as u32,
        height as u32);

    // render the rectangles
    canvas.fill_rect(left_rect).unwrap();
    canvas.fill_rect(right_rect).unwrap();
}

