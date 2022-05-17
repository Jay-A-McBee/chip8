use crate::render::{Render, Renderer};
use sdl2::Sdl;

use crate::Result;

pub struct DrawInfo<'a> {
    pub coords: (u8, u8),
    pub row_count: u8,
    pub sprites: &'a [u8],
}

pub struct Display {
    /// 4 x 64 x 32 byte array
    ///
    /// 4 bytes represent a single rgb pixel [r, g, b, 0]
    pub virtual_canvas: [[[u8; 4]; 64]; 32],
    /// Wrapper around a Canvas that implements Render trait
    pub renderer: Renderer,
}

impl From<&Sdl> for Display {
    fn from(sdl_ctx: &Sdl) -> Self {
        let renderer = Renderer::from(sdl_ctx);
        Display {
            renderer,
            virtual_canvas: [[[0; 4]; 64]; 32],
        }
    }
}

impl Display {
    /// Clears the canvas and the virtual byte canvas
    pub fn clear(&mut self) -> Result<()> {
        self.virtual_canvas = [[[0; 4]; 64]; 32];
        self.renderer.clear()
    }

    /// Returns virtual_canvas as a flat byte Vec
    pub fn get_raw_bytes(&mut self) -> Vec<u8> {
        self.virtual_canvas
            .iter()
            .flat_map(|row| row.iter().flatten())
            .copied()
            .collect::<Vec<u8>>()
    }

    /// Updates virtual_canvas and renders to window canvas
    pub fn draw<F: FnMut(bool)>(
        &mut self,
        DrawInfo {
            coords,
            row_count,
            sprites,
        }: DrawInfo,
        mut flipped_bits_callback: F,
    ) -> Result<()> {
        let (x_coord, y_coord) = (coords.0 & 63, coords.1 & 31);

        let final_row = if y_coord + row_count >= 32 {
            32
        } else {
            y_coord + row_count
        };

        let final_column = if x_coord + 8 >= 64 { 64 } else { x_coord + 8 };

        let rows = (y_coord..final_row).into_iter().enumerate();
        let mut flipped = false;

        for (sprite_idx, row_idx) in rows {
            let mut row = self.virtual_canvas[row_idx as usize];
            let sprite_pixel = sprites[sprite_idx];

            for (idx, column_idx) in (x_coord..final_column).into_iter().enumerate() {
                let pixel = row[column_idx as usize];
                let bit_shift = 7 - idx;
                // check bit
                let is_on = (sprite_pixel >> bit_shift) & 1 == 1;

                if is_on {
                    if pixel == [5, 110, 5, 0] {
                        row[column_idx as usize] = [0, 0, 0, 0];
                        flipped = true;
                    } else {
                        row[column_idx as usize] = [5, 110, 5, 0];
                    }
                }
            }

            self.virtual_canvas[row_idx as usize] = row;
        }

        let bytes = self.get_raw_bytes();
        self.renderer.render(&bytes)?;
        flipped_bits_callback(flipped);
        Ok(())
    }
}

mod tests {
    use super::*;

    pub struct MockCanvas {}

    impl Render for MockCanvas {
        fn render(&mut self, _bytes: &[u8]) -> Result<()> {
            Ok(())
        }

        fn clear(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[allow(dead_code)]
    fn setup() -> Display {
        let renderer = Renderer {
            canvas: Box::new(MockCanvas {}),
        };

        Display {
            renderer,
            virtual_canvas: [[[0; 4]; 64]; 32],
        }
    }

    #[test]
    fn draws_to_virtual_canvas() {
        let mut display = setup();
        let draw_info = DrawInfo {
            coords: (10, 20),
            row_count: 5,
            sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90], // letter "A",
        };
        let mock_cb = |_bool_arg| {};

        let _ = display.draw(draw_info, mock_cb);

        assert!(display.virtual_canvas[20][10] == [5, 110, 5, 0])
    }

    #[test]
    fn clears_virtual_canvas() {
        let mut display = setup();
        let mock_cb = |_bool_arg| {};
        let draw_info = DrawInfo {
            coords: (10, 20),
            row_count: 5,
            sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90],
        };

        let _ret1 = display.draw(draw_info, mock_cb);
        let _ret2 = display.clear();

        assert!(display.virtual_canvas[20][10] == [0, 0, 0, 0])
    }

    #[test]
    fn handles_x_coord_oob() {
        let mut display = setup();
        let mock_cb = |_bool_arg| {};
        let draw_info = DrawInfo {
            coords: (60, 20),
            row_count: 5,
            sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90],
        };

        let _ret1 = display.draw(draw_info, mock_cb);

        assert!(true == true);
    }

    #[test]
    fn handles_y_coord_oob() {
        let mut display = setup();
        let mock_cb = |_bool_arg| {};
        let draw_info = DrawInfo {
            coords: (60, 30),
            row_count: 5,
            sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90],
        };

        let _ret1 = display.draw(draw_info, mock_cb);

        assert!(true == true);
    }
}
