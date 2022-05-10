use sdl2::keyboard::Scancode;
use std::{collections::HashMap, hash::Hash};

pub struct Keyboard {
    pub hex_to_scancode: HashMap<u8, Scancode>,
    pub scancode_to_hex: HashMap<Scancode, u8>,
}

const HEX_TO_SCANCODE: [(u8, Scancode); 16] = [
    (1, Scancode::Num1),
    (2, Scancode::Num2),
    (3, Scancode::Num3),
    (12, Scancode::Num4),
    (4, Scancode::Q),
    (5, Scancode::W),
    (6, Scancode::E),
    (13, Scancode::R),
    (7, Scancode::A),
    (8, Scancode::S),
    (9, Scancode::D),
    (14, Scancode::F),
    (10, Scancode::Z),
    (0, Scancode::X),
    (11, Scancode::C),
    (15, Scancode::V),
];

impl Keyboard {
    pub fn new() -> Self {
        let scancode_to_hex: [(Scancode, u8); 16] = HEX_TO_SCANCODE
            .iter()
            .map(|(hex, scan)| (scan.clone(), hex.clone()))
            .collect::<Vec<(Scancode, u8)>>()
            .try_into()
            .unwrap();

        Keyboard {
            hex_to_scancode: HashMap::from(HEX_TO_SCANCODE),
            scancode_to_hex: HashMap::from(scancode_to_hex),
        }
    }
}
