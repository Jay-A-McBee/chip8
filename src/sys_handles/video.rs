use sdl2::{pixels, render::WindowCanvas, Sdl};

use crate::Result;

pub trait Renderable {
    fn render(&mut self, bytes: &[u8]) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
}

struct Canvas<T> {
    canvas: T,
}

impl From<&Sdl> for Canvas<WindowCanvas> {
    fn from(sdl_ctx: &Sdl) -> Self {
        let video_subsystem = sdl_ctx.video().unwrap();
        let window = video_subsystem
            .window("Chip8", 64 * 10, 32 * 10)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync() //< this means the screen cannot
            // render faster than your display rate (usually 60Hz or 144Hz)
            .build()
            .unwrap();

        Canvas { canvas }
    }
}

impl Renderable for Canvas<WindowCanvas> {
    /// creates a texture from byte array and renders onto canvas
    fn render(&mut self, bytes: &[u8]) -> Result<()> {
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

    fn clear(&mut self) -> Result<()> {
        self.canvas.set_draw_color(pixels::Color::RGB(255, 0, 20));
        self.canvas.clear();
        Ok(())
    }
}

pub struct Renderer {
    pub canvas: Box<dyn Renderable>,
}

impl Renderer {
    pub fn clear(&mut self) -> Result<()> {
        self.canvas.clear()
    }

    pub fn render(&mut self, bytes: &[u8]) -> Result<()> {
        self.canvas.render(bytes)
    }
}

impl From<&Sdl> for Renderer {
    fn from(sdl_ctx: &Sdl) -> Self {
        Renderer {
            canvas: Box::new(Canvas::from(sdl_ctx)),
        }
    }
}
