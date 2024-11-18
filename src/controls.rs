use crate::draw::fill_triangle;
use crate::ui::{BUFFER_SIZE, Vector2};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

