use sdl2::keyboard::Scancode;
use std::collections::{HashMap, HashSet};

pub struct Keyboard {
    pub hex_to_scancode: HashMap<u8, Scancode>,
    pub scancode_to_hex: HashMap<Scancode, u8>,
    pressed_keys: HashSet<u8>,
    last_pressed: Option<u8>,
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
            .map(|(hex, scan)| (*scan, *hex))
            .collect::<Vec<(Scancode, u8)>>()
            .try_into()
            .unwrap();

        Keyboard {
            hex_to_scancode: HashMap::from(HEX_TO_SCANCODE),
            scancode_to_hex: HashMap::from(scancode_to_hex),
            pressed_keys: HashSet::new(),
            last_pressed: None,
        }
    }

    pub fn press_key(&mut self, scancode: Scancode) {
        self.release_key(self.last_pressed);

        if let Some(&code) = self.scancode_to_hex.get(&scancode) {
            self.pressed_keys.insert(code);
            self.last_pressed = Some(code);
        };
    }

    pub fn release_key(&mut self, hex_code: Option<u8>) {
        if let Some(hex) = hex_code {
            self.pressed_keys.remove(&hex);
        }
    }

    pub fn is_pressed(&self, keycode: u8) -> bool {
        self.pressed_keys.contains(&keycode)
    }
}
