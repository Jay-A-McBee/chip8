use sdl2::keyboard::Scancode;
use std::collections::HashMap;

pub struct Keyboard {
    pub hex_to_scancode: HashMap<u8, Scancode>,
    pub scancode_to_hex: HashMap<Scancode, u8>,
    pub pressed_keys: HashMap<u8, bool>,
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
            .map(|(hex, scan)| (scan.clone(), hex.clone()))
            .collect::<Vec<(Scancode, u8)>>()
            .try_into()
            .unwrap();

        let pressed_keys: [(u8, bool); 16] = HEX_TO_SCANCODE
            .iter()
            .map(|(val, _)| (val.clone(), false))
            .collect::<Vec<(u8, bool)>>()
            .try_into()
            .unwrap();

        Keyboard {
            hex_to_scancode: HashMap::from(HEX_TO_SCANCODE),
            scancode_to_hex: HashMap::from(scancode_to_hex),
            pressed_keys: HashMap::from(pressed_keys),
            last_pressed: None,
        }
    }

    pub fn press_key(&mut self, scancode: Scancode) {
        if let Some(code) = self.last_pressed {
            self.pressed_keys.insert(code, false);
        }

        self.scancode_to_hex.get(&scancode).and_then(|&code| {
            println!("PRESSED:{code}");
            self.pressed_keys.insert(code, true);
            self.last_pressed = Some(code);
            Some(())
        });
    }

    pub fn release_key(&mut self, scancode: Scancode) {
        println!("IN RELEASE:{scancode}");
        self.scancode_to_hex.get(&scancode).and_then(|code| {
            self.pressed_keys.insert(*code, false);
            Some(())
        });
    }

    pub fn is_pressed(&self, keycode: u8) -> bool {
        println!(
            "IN IS_PRESSED:{keycode}::{}",
            *self.pressed_keys.get(&keycode).unwrap()
        );
        *self.pressed_keys.get(&keycode).unwrap()
    }
}
