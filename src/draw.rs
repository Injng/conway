use crate::ui::Vector2;

use sdl2::render::Canvas;
use sdl2::video::Window;

/// Given two points that form a line, and a y-coordinate, interpolate the x-coordinate 
pub fn interpolate(a: Vector2, b: Vector2, y: i32) -> i32 {
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

