use crate::draw::fill_triangle;
use crate::ui::{BUFFER_SIZE, Vector2};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// control dimension constants
const HEIGHT: i32 = BUFFER_SIZE / 2;
const PLAY_BUTTON_WIDTH: i32 = ((HEIGHT / 2) as f32 * 1.7321) as i32;
const PAUSE_BUTTON_WIDTH: i32 = HEIGHT / 4;
const PAUSE_BUTTON_DIST: i32 = HEIGHT / 3;

// padding on top and on the bottom for the controls
const PADDING_TOP: i32 = (BUFFER_SIZE - HEIGHT) / 2;
const PADDING_BOTTOM: i32 = BUFFER_SIZE - PADDING_TOP - HEIGHT;

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

