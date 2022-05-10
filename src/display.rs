use crate::sys_handles::video::Video;
use sdl2::pixels;

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
    /// sdl::video handle
    pub video: Video,
}

impl Display {
    /// Inits the sdl window, canvas and virtual byte canvas
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Display {
            virtual_canvas: [[[0; 4]; 64]; 32],
            video: Video::new(sdl_context),
        }
    }

    /// Clears the canvas and the virtual byte canvas
    pub fn clear(&mut self) {
        self.video.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.video.clear();
        self.virtual_canvas = [[[0; 4]; 64]; 32];
    }

    /// Returns virtual_canvas as a flat byte Vec
    pub fn get_raw_bytes(&mut self) -> Vec<u8> {
        self.virtual_canvas
            .iter()
            .flat_map(|row| row.iter().flat_map(|row| row))
            .copied()
            .collect::<Vec<u8>>()
    }

    /// Updates virtual_canvas and renders to window canvas
    pub fn draw<F: FnMut() -> ()>(
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

        for (sprite_idx, row_idx) in rows {
            let mut row = self.virtual_canvas[row_idx as usize];
            let sprite_pixel = sprites[sprite_idx];

            for column_idx in (x_coord..final_column) {
                let pixel = row[column_idx as usize];
                // check bit
                let is_on = (sprite_pixel >> final_column - column_idx - 1) & 1 == 1;

                if is_on {
                    row[column_idx as usize] = [5, 110, 5, 0];
                } else if u32::from_be_bytes(pixel) == u32::from_be_bytes([5, 110, 5, 0]) {
                    row[column_idx as usize] = [0, 0, 0, 0];
                    flipped_bits_callback();
                }
            }

            self.virtual_canvas[row_idx as usize] = row;
        }

        let bytes = self.get_raw_bytes();
        self.video.render_texture(&bytes)?;

        Ok(())
    }
}

// mod tests {
//     use super::*;

//     #[test]
//     fn draws_to_the_canvas() {
//         let mut display = Display::new();
//         let draw_info = DrawInfo {
//             coords: (10, 20),
//             row_count: 5,
//             sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90], // letter "A",
//         };

//         display.draw(draw_info);

//         // slice out the letter "A"
//         // ****
//         // *  *
//         // ****
//         // *  *
//         // *  *
//         let letter_a: [[u8; 4]; 5] = display.video[20..25]
//             .iter()
//             .map(|row| {
//                 let pixels = &row[10..15];
//                 [pixels[0], pixels[1], pixels[2], pixels[3]]
//             })
//             .collect::<Vec<_>>()
//             .try_into()
//             .unwrap();

//         let expected = [
//             [1u8, 1u8, 1u8, 1u8],
//             [1u8, 0u8, 0u8, 1u8],
//             [1u8, 1u8, 1u8, 1u8],
//             [1u8, 0u8, 0u8, 1u8],
//             [1u8, 0u8, 0u8, 1u8],
//         ];

//         assert_eq!(letter_a, expected);
//     }

//     #[test]
//     fn clears_the_canvas() {
//         let mut display = Display::new();
//         let draw_info = DrawInfo {
//             coords: (10, 20),
//             row_count: 5,
//             sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90],
//         };

//         display.draw(draw_info);
//         display.clear();

//         assert!(display.video[20][10] == 0)
//     }
// }
