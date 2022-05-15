use crate::display::{Display, DrawInfo};
use crate::instruction::Instruction;
use crate::ram::{Ram, Timer};
use crate::sys_handles::keyboard::Keyboard;

use sdl2::{event, keyboard, EventPump};
use std::time::{Duration, Instant};

pub struct Emulator {
    display: Display,
    pub event_pump: EventPump,
    keyboard: Keyboard,
    last_cycle: Option<Instant>,
    loaded_ram: Ram,
}

impl Emulator {
    const CYCLE_RATE: u128 = Duration::from_millis(1000 / 60).as_millis();

    pub fn new(program: Vec<u8>) -> Self {
        let sdl_ctx = sdl2::init().unwrap();
        let mut event_pump = sdl_ctx.event_pump().unwrap();

        let mut display = Display::from(&sdl_ctx);
        let mut kb = Keyboard::new();
        let mut loaded_ram = Ram::load(program.as_slice());

        Emulator {
            display,
            event_pump,
            keyboard: kb,
            loaded_ram,
            last_cycle: None,
        }
    }

    pub fn start(&mut self) {
        'running: loop {
            for ev in self.event_pump.poll_iter() {
                match ev {
                    event::Event::Quit { .. } => {
                        // break 'running;
                    }
                    event::Event::KeyDown {
                        scancode: Some(code),
                        ..
                    } => self.keyboard.press_key(code),
                    _ => {
                        println!("EVENT::{ev:?}");
                    }
                }
            }

            self.cycle();
        }
    }

    fn get_nums(value: &u8) -> (u8, u8, u8) {
        let hundreds_digit = value / 100;
        let tens_digit = (value / 10) % 10;
        let ones_digit = value % 10;

        (hundreds_digit, tens_digit, ones_digit)
    }

    fn is_pressed(&self, key_code: u8) -> bool {
        println!("IS PRESSED::{}", key_code);
        self.keyboard.is_pressed(key_code)
    }

    pub fn cycle(&mut self) {
        let time_elapsed = if let Some(instant) = self.last_cycle {
            instant.elapsed().as_millis()
        } else {
            Self::CYCLE_RATE
        };

        if time_elapsed >= Self::CYCLE_RATE {
            self.last_cycle = Some(Instant::now());
            self.process_instruction();
        }
    }

    pub fn process_instruction(&mut self) {
        if self.loaded_ram.delay_timer > 0 {
            self.loaded_ram.delay_timer -= 1;
        }

        if self.loaded_ram.sound_timer > 0 {
            self.loaded_ram.sound_timer -= 1;
        }

        let instruction_bytes = self.loaded_ram.get_next_instruction();

        let parsed_instruction = Instruction::from(instruction_bytes);

        let Instruction {
            first_nibble,
            x,
            y,
            n,
            nn,
            nnn,
            ..
        } = parsed_instruction;
        // println!("instructions::{}", format!("{}", parsed_instruction));
        match (first_nibble, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.display.clear();
            }
            (0x0, 0x0, 0xE, 0xE) => {
                self.loaded_ram.remove_addr().and_then(|return_addr| {
                    self.loaded_ram.PC = return_addr as usize;
                    Some(())
                });
            }
            (0x1, _, _, _) => {
                self.loaded_ram.PC = nnn as usize;
            }
            (0x2, _, _, _) => {
                self.loaded_ram.store_addr(self.loaded_ram.PC);
                self.loaded_ram.PC = nnn as usize;
            }
            (0x3, _, _, _) => {
                if self.loaded_ram.V[x as usize] == nn {
                    self.loaded_ram.PC += 2;
                }
            }
            (0x4, _, _, _) => {
                if self.loaded_ram.V[x as usize] != nn {
                    self.loaded_ram.PC += 2;
                }
            }
            (0x5, _, _, 0) => {
                if self.loaded_ram.V[x as usize] == self.loaded_ram.V[y as usize] {
                    self.loaded_ram.PC += 2;
                }
            }
            (0x6, _, _, _) => self.loaded_ram.set_register(x as usize, nn),
            (0x7, _, _, _) => {
                let (updated, _) = self.loaded_ram.V[x as usize].overflowing_add(nn);
                self.loaded_ram.V[x as usize] = updated;
            }
            (0x8, _, _, _) => match n {
                0 => self.loaded_ram.V[x as usize] = self.loaded_ram.V[y as usize],
                1 => self.loaded_ram.V[x as usize] |= self.loaded_ram.V[y as usize],
                2 => self.loaded_ram.V[x as usize] &= self.loaded_ram.V[y as usize],
                3 => self.loaded_ram.V[x as usize] ^= self.loaded_ram.V[y as usize],
                4 => {
                    let (updated, did_overflow) = self.loaded_ram.V[x as usize]
                        .overflowing_add(self.loaded_ram.V[y as usize]);
                    self.loaded_ram.V[x as usize] = updated;
                    self.loaded_ram.update_vf_register(did_overflow);
                }
                5 => {
                    self.loaded_ram.update_vf_register(
                        self.loaded_ram.V[x as usize] > self.loaded_ram.V[y as usize],
                    );
                    let (updated, _) = self.loaded_ram.V[x as usize]
                        .overflowing_sub(self.loaded_ram.V[y as usize]);
                    self.loaded_ram.V[x as usize] = updated;
                }
                6 => {
                    // value that will be shifted out - first big end bit
                    let is_one = self.loaded_ram.V[x as usize] & 1 == 1;
                    self.loaded_ram.update_vf_register(is_one);
                    self.loaded_ram.V[x as usize] = self.loaded_ram.V[x as usize] >> 1;
                }
                7 => {
                    self.loaded_ram.update_vf_register(
                        self.loaded_ram.V[y as usize] > self.loaded_ram.V[x as usize],
                    );
                    let (updated, _) = self.loaded_ram.V[y as usize]
                        .overflowing_sub(self.loaded_ram.V[x as usize]);
                    self.loaded_ram.V[x as usize] = updated;
                }
                14 => {
                    let is_one = self.loaded_ram.V[x as usize] >> 7 == 1;
                    // value that will be shifted out - final big end bit
                    self.loaded_ram.update_vf_register(is_one);
                    self.loaded_ram.V[x as usize] = self.loaded_ram.V[x as usize] << 1;
                }
                _ => println!("MISS::{}", n),
            },
            (0x9, _, _, _) => {
                if self.loaded_ram.V[x as usize] != self.loaded_ram.V[y as usize] {
                    self.loaded_ram.PC += 2;
                }
            }
            (0xA, _, _, _) => {
                self.loaded_ram.set_i_register(nnn);
            }
            (0xB, _, _, _) => {
                self.loaded_ram.PC = (self.loaded_ram.V[0] as u16 + nnn) as usize;
            }
            (0xC, _, _, _) => {
                self.loaded_ram.V[x as usize] = rand::random::<u8>() & nn;
            }
            (0xD, _, _, _) => {
                let sprite_start_idx = self.loaded_ram.I as usize;
                let sprite_end_idx = sprite_start_idx + (n + 1) as usize;

                let sprites = &self.loaded_ram.mem[sprite_start_idx..sprite_end_idx]
                    .iter()
                    .copied()
                    .collect::<Vec<u8>>();

                let coords = (self.loaded_ram.V[x as usize], self.loaded_ram.V[y as usize]);

                let flipped_bit_callback = |did_flip: bool| {
                    self.loaded_ram.update_vf_register(did_flip);
                };

                let draw_info = DrawInfo {
                    coords,
                    sprites: sprites.as_slice(),
                    row_count: n,
                };

                self.display.draw(draw_info, flipped_bit_callback).unwrap();
            }
            (0xE, _, _, _) => match n {
                0xE => {
                    println!("IN E::{}", self.loaded_ram.V[x as usize]);
                    if self.is_pressed(self.loaded_ram.V[x as usize]) {
                        self.loaded_ram.PC += 2;
                    }
                }
                0x1 => {
                    if !self.is_pressed(self.loaded_ram.V[x as usize]) {
                        self.loaded_ram.PC += 2;
                    }
                }
                _ => {
                    println!(
                        "Recieved unexpected 0xE operation - O::{} X::{} Y::{} N::{}",
                        first_nibble, x, y, n
                    );
                }
            },
            (0xF, _, _, _) => match n {
                3 => {
                    let (hundred, ten, one) = Self::get_nums(&self.loaded_ram.V[x as usize]);
                    let current_idx = self.loaded_ram.I;
                    self.loaded_ram.mem[current_idx as usize] = hundred;
                    self.loaded_ram.mem[(current_idx + 1) as usize] = ten;
                    self.loaded_ram.mem[(current_idx + 2) as usize] = one;
                }
                5 => match y {
                    1 => self
                        .loaded_ram
                        .set_timer_register(Timer::Delay, self.loaded_ram.V[x as usize]),
                    5 => {
                        let current_index = self.loaded_ram.I as usize;
                        self.loaded_ram.V[0..(x + 1) as usize]
                            .iter()
                            .enumerate()
                            .for_each(|(i, val)| {
                                self.loaded_ram.mem[current_index + i] = *val;
                            });
                    }
                    6 => {
                        (self.loaded_ram.I..self.loaded_ram.I + 1 + x as u16)
                            .enumerate()
                            .for_each(|(i, addr)| {
                                println!("{}::{}", i, addr);
                                self.loaded_ram.V[i] = self.loaded_ram.mem[addr as usize];
                            });
                    }
                    _ => (),
                },
                7 => {
                    let current_delay_value = self.loaded_ram.get_timer_register(Timer::Delay);
                    self.loaded_ram
                        .set_register(x as usize, current_delay_value);
                }
                8 => self
                    .loaded_ram
                    .set_timer_register(Timer::Sound, self.loaded_ram.V[x as usize]),
                9 => {
                    let char = self.loaded_ram.V[x as usize];
                    self.loaded_ram.I = (80 + (char * 5)) as u16;
                }
                0xA => {
                    if let event::Event::KeyDown {
                        scancode: Some(code),
                        ..
                    } = self.event_pump.wait_event()
                    {
                        self.keyboard.scancode_to_hex.get(&code).and_then(|val| {
                            self.loaded_ram.V[x as usize] = *val;
                            Some(())
                        });
                    }
                }
                0xE => {
                    let (update, did_overflow) = self
                        .loaded_ram
                        .I
                        .overflowing_add(self.loaded_ram.V[x as usize] as u16);
                    self.loaded_ram.set_i_register(update);
                    self.loaded_ram.update_vf_register(did_overflow);
                }
                _ => {
                    println!(
                        "Recieved unexpected 0xF operation - O::{} X::{} Y::{} N::{}",
                        first_nibble, x, y, n
                    );
                }
            },
            _ => {
                println!("MISS::{:?}", instruction_bytes);
            }
        }
    }
}
