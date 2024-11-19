use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use std::collections::HashMap;

pub struct TextCache<'creator, 'font> {
    texture_creator: &'creator TextureCreator<WindowContext>,
    font: &'font Font<'font, 'static>,
    textures: HashMap<String, Texture<'creator>>,
}

impl<'creator, 'font> TextCache<'creator, 'font> {
    pub fn new(
        texture_creator: &'creator TextureCreator<WindowContext>, 
        font: &'font Font<'font, 'static>
    ) -> Self {
        TextCache {
            texture_creator,
            font,
            textures: HashMap::new(),
        }
    }

    /// Render text by getting the texture from the hashmap, or creating a new one
    pub fn render_text(&mut self, text: &str) -> &Texture<'creator> {
        if self.textures.contains_key(text) {
            return &self.textures[text];
        } else {
            let text_surface: Surface = self.font
                .render(text)
                .blended(Color::BLACK)
                .expect("Failed to render text");
            let text_texture: Texture = self
                .texture_creator
                .create_texture_from_surface(text_surface)
                .unwrap();
            self.textures.insert(text.to_string(), text_texture);
            return &self.textures[text];
        }
    }

    /// Get dimensions of string rendered with Font
    pub fn get_dimensions(&self, text: &str) -> (i32, i32) {
        let dimensions = self.font.size_of(text).unwrap();
        (dimensions.0 as i32, dimensions.1 as i32)
    }
}

