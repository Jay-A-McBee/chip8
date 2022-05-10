use sdl2::{pixels, render, video, Sdl};

use crate::Result;

pub struct Video {
    canvas: render::Canvas<video::Window>,
}

impl Video {
    /// returns a new Video instance
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Chip8", 64 * 20, 32 * 20)
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync() //< this means the screen cannot
            // render faster than your display rate (usually 60Hz or 144Hz)
            .build()
            .unwrap();

        Video { canvas }
    }

    /// creates a texture from byte array and renders onto canvas
    pub fn render_texture(&mut self, bytes: &[u8]) -> Result<()> {
        let creator = self.canvas.texture_creator();
        // create a texture that matches our virtual display dimensions
        let mut texture =
            creator.create_texture_streaming(creator.default_pixel_format(), 64, 32)?;
        // pitch - bytes per row - 4 bytes per pixel
        texture.update(None, bytes, 64 * 4)?;

        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();

        Ok(())
    }

    /// sets draw color for canvas
    pub fn set_draw_color(&mut self, color: pixels::Color) {
        self.canvas.set_draw_color(color);
    }

    /// sets draw color to black and clears the canvas
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
}
