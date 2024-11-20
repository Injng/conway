use crate::draw::{fill_triangle, interpolate};
use crate::text::TextCache;
use crate::ui::{BUFFER_SIZE, Vector2};

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

// control dimension constants
const HEIGHT: i32 = BUFFER_SIZE / 2;
const PLAY_BUTTON_WIDTH: i32 = ((HEIGHT / 2) as f32 * 1.7321) as i32;
const PAUSE_BUTTON_WIDTH: i32 = HEIGHT / 4;
const PAUSE_BUTTON_DIST: i32 = HEIGHT / 3;

// padding on top and on the bottom for the controls
const PADDING_TOP: i32 = (BUFFER_SIZE - HEIGHT) / 2;
const PADDING_BOTTOM: i32 = BUFFER_SIZE - PADDING_TOP - HEIGHT;

// location of slider
const SLIDER_X: i32 = 60;
const SLIDER_WIDTH: i32 = 120;
const SLIDER_PADDING: i32 = 2;

// icon size and locations
const ICON_SIZE: i32 = 20;
const UPLOAD_X: i32 = 60;

/// Render a upload icon in the top buffer
pub fn render_upload(canvas: &mut Canvas<Window>, icon: &Texture) {
    // create rectangle for icon
    let upload_rect = Rect::new(UPLOAD_X,
                                (BUFFER_SIZE - ICON_SIZE) / 2,
                                ICON_SIZE as u32,
                                ICON_SIZE as u32);

    // copy texture
    canvas.copy(icon, None, Some(upload_rect)).unwrap();
}

/// Render a play button in the bottom buffer
pub fn render_play(canvas: &mut Canvas<Window>) {
    // get screen size and set draw color
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    let screen_width = screen_size.0 as i32;
    let screen_height = screen_size.1 as i32;
    canvas.set_draw_color(Color::BLACK);

    // points for the play button triangle
    let a = Vector2::new((screen_width - PLAY_BUTTON_WIDTH) / 2, 
        screen_height - BUFFER_SIZE + PADDING_TOP);
    let b = Vector2::new((screen_width - PLAY_BUTTON_WIDTH) / 2, 
        screen_height - PADDING_BOTTOM);
    let c = Vector2::new((screen_width + PLAY_BUTTON_WIDTH) / 2, 
        screen_height - PADDING_BOTTOM - HEIGHT / 2);

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

    // create the rectangles
    let left_rect = Rect::new((screen_width - PAUSE_BUTTON_DIST) / 2 - PAUSE_BUTTON_WIDTH,
        screen_height - PADDING_BOTTOM - HEIGHT,
        PAUSE_BUTTON_WIDTH as u32,
        HEIGHT as u32);
    let right_rect = Rect::new((screen_width + PAUSE_BUTTON_DIST) / 2,
        screen_height - PADDING_BOTTOM - HEIGHT,
        PAUSE_BUTTON_WIDTH as u32,
        HEIGHT as u32);

    // render the rectangles
    canvas.fill_rect(left_rect).unwrap();
    canvas.fill_rect(right_rect).unwrap();
}

/// Render a slider for controlling the speed of the simulation
/// The length of the inner slider is controlled by a number from 0 to 1
pub fn render_slider(
    canvas: &mut Canvas<Window>, 
    text_cache: &mut TextCache,
    speed_text: &str,
    slider_len: f32,
) {
    // get screen size and set draw color
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    let screen_height = screen_size.1 as i32;
    canvas.set_draw_color(Color::BLACK);

    // create the rectangles
    let outer_rect = Rect::new(SLIDER_X,
        screen_height - PADDING_BOTTOM - HEIGHT,
        SLIDER_WIDTH as u32,
        HEIGHT as u32);
    let inner_rect = Rect::new(SLIDER_X + SLIDER_PADDING,
        screen_height - PADDING_BOTTOM - HEIGHT + SLIDER_PADDING,
        ((SLIDER_WIDTH - 2 * SLIDER_PADDING) as f32 * slider_len) as u32,
        (HEIGHT - 2 * SLIDER_PADDING) as u32);
    
    // render the rectangles
    canvas.draw_rect(outer_rect).unwrap();
    canvas.set_draw_color(Color::RGB(192, 192, 192));
    canvas.fill_rect(inner_rect).unwrap();

    // render the speed text
    let dimensions: (i32, i32) = text_cache.get_dimensions(speed_text);
    let text_texture: &Texture = text_cache.render_text(speed_text);
    let text_x: i32 =
        SLIDER_X + ((SLIDER_WIDTH - dimensions.0) as f32 / 2.0) as i32;
    let text_y: i32 =
        screen_height - PADDING_BOTTOM - ((HEIGHT + dimensions.1) as f32 / 2.0) as i32;
    canvas
        .copy(
            text_texture,
            None,
            Some(Rect::new(text_x, text_y, dimensions.0 as u32, dimensions.1 as u32)),
        )
        .unwrap();
}

/// Given x and y coordinates, check to see if it is within the play button
pub fn in_play(canvas: &Canvas<Window>, x: i32, y: i32) -> bool {
    // get screen size
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    let screen_width = screen_size.0 as i32;
    let screen_height = screen_size.1 as i32;

    // points for the play button triangle
    let a = Vector2::new((screen_width - PLAY_BUTTON_WIDTH) / 2, 
        screen_height - BUFFER_SIZE + PADDING_TOP);
    let b = Vector2::new((screen_width - PLAY_BUTTON_WIDTH) / 2, 
        screen_height - PADDING_BOTTOM);
    let c = Vector2::new((screen_width + PLAY_BUTTON_WIDTH) / 2, 
        screen_height - PADDING_BOTTOM - HEIGHT / 2);

    // sort the points in increasing order by y-level
    let mut points: Vec<Vector2> = vec![a, b, c];
    points.sort_by(|first, second| first.y.cmp(&second.y));

    // ensure given y-value is within the least and greatest y-values of the triangle
    if y < points[0].y || y > points[2].y {
        return false;
    }

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
    
    // get start and end x-values of the play button in the given y-level
    let start: i32 = interpolate(left_edge.0, left_edge.1, y);
    let end: i32 = interpolate(right_edge.0, right_edge.1, y);

    x >= start && x <= end
}

/// Given x and y coordinates, check to see if it is within the pause button
pub fn in_pause(canvas: &Canvas<Window>, x: i32, y: i32) -> bool {
    // get screen size and click point
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    let screen_width = screen_size.0 as i32;
    let screen_height = screen_size.1 as i32;
    let click = Point::new(x, y);

    // create the pause bounding rectangle
    let pause_rect = Rect::new((screen_width - PAUSE_BUTTON_DIST) / 2 - PAUSE_BUTTON_WIDTH,
        screen_height - PADDING_BOTTOM - HEIGHT,
        (2 * PAUSE_BUTTON_WIDTH + PAUSE_BUTTON_DIST) as u32,
        HEIGHT as u32);

    pause_rect.contains_point(click)
}

/// Given x and y coordinates, check to see if it is within the upload icon
pub fn in_upload(x: i32, y: i32) -> bool {
    // create the bounding rectangle for the upload icon
    let upload_rect = Rect::new(UPLOAD_X,
                                (BUFFER_SIZE - ICON_SIZE) / 2,
                                ICON_SIZE as u32,
                                ICON_SIZE as u32);

    upload_rect.contains_point(Point::new(x, y))
}

/// Given x and y coordinates, check to see if it is within the slider
pub fn in_slider(canvas: &Canvas<Window>, x: i32, y: i32) -> bool {
    // get screen size and click point
    let screen_size: (u32, u32) = canvas.output_size().unwrap();
    let screen_height = screen_size.1 as i32;
    let click = Point::new(x, y);

    // create the pause bounding rectangle
    let outer_rect = Rect::new(SLIDER_X,
        screen_height - PADDING_BOTTOM - HEIGHT,
        SLIDER_WIDTH as u32,
        HEIGHT as u32);

    outer_rect.contains_point(click)
}

/// Given x coordinates, calculate how long the slider should be
pub fn calc_slider(x: i32) -> f32 {
    // calculate bounds
    let left_bound: i32 = SLIDER_X;
    let right_bound: i32 = SLIDER_X + SLIDER_WIDTH;

    if x <= left_bound {
        return 0.0;
    } else if x >= right_bound {
        return 1.0;
    } else {
        return (x - SLIDER_X) as f32 / SLIDER_WIDTH as f32;
    }
}

