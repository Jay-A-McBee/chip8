pub struct DrawInfo<'a> {
    pub coords: (u8, u8),
    pub row_count: u8,
    pub sprites: &'a [u8],
}

pub struct Display {
    pub canvas: [[u8; 64]; 32],
}

impl Display {
    pub fn new() -> Self {
        Display {
            canvas: [[0; 64]; 32],
        }
    }

    pub fn clear(&mut self) {
        self.canvas = [[0; 64]; 32];
    }

    pub fn draw(
        &mut self,
        DrawInfo {
            coords,
            row_count,
            sprites,
        }: DrawInfo,
    ) -> Option<()> {
        let (x_coord, y_coord) = (coords.0 & 63, coords.1 & 31);

        let mut sprite_idx = 0 as usize;
        let mut bit_shift = 7;
        let mut flipped = None;

        for i in y_coord..(y_coord + row_count) {
            let mut row = self.canvas[i as usize];

            for j in x_coord..(x_coord + 8) {
                let pixel = row[j as usize];
                let sprite_pixel = sprites[sprite_idx];

                // check bit
                let is_on = sprite_pixel >> bit_shift & 1 == 1;

                if is_on && pixel == 0 {
                    row[j as usize] = 1;
                } else if pixel == 1 {
                    row[j as usize] = 0;
                    flipped = Some(());
                }

                bit_shift -= 1;
            }

            self.canvas[i as usize] = row;
            bit_shift = 7;
            sprite_idx += 1;
        }

        let word = self
            .canvas
            .iter()
            .map(|row| {
                row.iter()
                    .map(|val| if *val == 1 { return "X" } else { return "." })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .inspect(|row| println!("{}", row))
            .collect::<Vec<String>>();

        flipped
    }
}

mod tests {
    use super::*;

    #[test]
    fn draws_to_the_canvas() {
        let mut display = Display::new();
        let draw_info = DrawInfo {
            coords: (10, 20),
            row_count: 5,
            sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90], // letter "A",
        };

        display.draw(draw_info);

        // slice out the letter "A"
        // ****
        // *  *
        // ****
        // *  *
        // *  *
        let letter_a: [[u8; 4]; 5] = display.canvas[20..25]
            .iter()
            .map(|row| {
                let pixels = &row[10..15];
                [pixels[0], pixels[1], pixels[2], pixels[3]]
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let expected = [
            [1u8, 1u8, 1u8, 1u8],
            [1u8, 0u8, 0u8, 1u8],
            [1u8, 1u8, 1u8, 1u8],
            [1u8, 0u8, 0u8, 1u8],
            [1u8, 0u8, 0u8, 1u8],
        ];

        assert_eq!(letter_a, expected);
    }

    #[test]
    fn clears_the_canvas() {
        let mut display = Display::new();
        let draw_info = DrawInfo {
            coords: (10, 20),
            row_count: 5,
            sprites: &[0xF0, 0x90, 0xF0, 0x90, 0x90],
        };

        display.draw(draw_info);
        display.clear();

        assert!(display.canvas[20][10] == 0)
    }
}
