mod display;
mod instruction;
mod ram;
mod sys_handles;

use std::{error, fs, path, result, thread, time::Duration};
use sys_handles::{keyboard::Keyboard, video};

use crate::display::DrawInfo;
use crate::instruction::Instruction;
use crate::ram::{Ram, Timer};

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

extern crate rand;
extern crate sdl2;

use sdl2::{event, keyboard, pixels};
fn main() -> Result<()> {
    let program =
        fs::read(path::PathBuf::from("../../Downloads/test_opcode.ch8")).expect("couldnt find");
    let pg_len = program.len();
    println!("{}", pg_len);

    sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1");
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut display = display::Display::new(&sdl_context);
    let kb = Keyboard::new();
    let mut loaded_ram = Ram::new(program.as_slice());

    display.video.set_draw_color(pixels::Color::RGB(5, 110, 5));

    'running: loop {
        event_pump.pump_events();
        for event in event_pump.poll_iter() {
            match event {
                event::Event::Quit { .. } => {}
                _ => {
                    println!("in some event");
                }
            }
        }
        'inner: loop {
            // TODO - move everything out of loaded_ram
            if loaded_ram.delay_timer > 0 {
                loaded_ram.delay_timer -= 1;
            }

            if loaded_ram.sound_timer > 0 {
                loaded_ram.sound_timer -= 1;
            }

            let instruction_bytes = loaded_ram.get_next_instruction();

            // thread::sleep(Duration::from_millis(1000 / 60));

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
            // println!("instructions::{:?}", [hi_byte, lo_byte]);
            // println!(
            //     "O::{}::X::{}::Y::{}::N::{}::NN::{}::NNN::{}",
            //     first_nibble, x, y, n, nn, nnn
            // );
            match (first_nibble, x, y, n) {
                (0x0, 0x0, 0xE, 0x0) => {
                    display.clear();
                }
                (0x0, 0x0, 0xE, 0xE) => {
                    loaded_ram.remove_addr().and_then(|return_addr| {
                        loaded_ram.PC = return_addr as usize;
                        Some(())
                    });
                }
                (0x1, _, _, _) => {
                    loaded_ram.PC = nnn as usize;
                }
                (0x2, _, _, _) => {
                    loaded_ram.store_addr(loaded_ram.PC);
                    loaded_ram.PC = nnn as usize;
                }
                (0x3, _, _, _) => {
                    if loaded_ram.V[x as usize] == nn {
                        loaded_ram.PC += 2;
                    }
                }
                (0x4, _, _, _) => {
                    if loaded_ram.V[x as usize] != nn {
                        loaded_ram.PC += 2;
                    }
                }
                (0x5, _, _, 0) => {
                    if loaded_ram.V[x as usize] == loaded_ram.V[y as usize] {
                        loaded_ram.PC += 2;
                    }
                }
                (0x6, _, _, _) => loaded_ram.set_register(x as usize, nn),
                (0x7, _, _, _) => {
                    let (updated, _) = loaded_ram.V[x as usize].overflowing_add(nn);
                    loaded_ram.V[x as usize] = updated;
                }
                (0x8, _, _, _) => match n {
                    0 => loaded_ram.V[x as usize] = loaded_ram.V[y as usize],
                    1 => loaded_ram.V[x as usize] |= loaded_ram.V[y as usize],
                    2 => loaded_ram.V[x as usize] &= loaded_ram.V[y as usize],
                    3 => loaded_ram.V[x as usize] ^= loaded_ram.V[y as usize],
                    4 => {
                        let (updated, did_overflow) =
                            loaded_ram.V[x as usize].overflowing_add(loaded_ram.V[y as usize]);
                        loaded_ram.V[x as usize] = updated;
                        loaded_ram.update_vf_register(did_overflow);
                    }
                    5 => {
                        loaded_ram.update_vf_register(
                            loaded_ram.V[x as usize] > loaded_ram.V[y as usize],
                        );
                        let (updated, _) =
                            loaded_ram.V[x as usize].overflowing_sub(loaded_ram.V[y as usize]);
                        loaded_ram.V[x as usize] = updated;
                    }
                    6 => {
                        // value that will be shifted out - first big end bit
                        let is_one = loaded_ram.V[x as usize] & 1 == 1;
                        loaded_ram.update_vf_register(is_one);
                        loaded_ram.V[x as usize] = loaded_ram.V[x as usize] >> 1;
                    }
                    7 => {
                        loaded_ram.update_vf_register(
                            loaded_ram.V[y as usize] > loaded_ram.V[x as usize],
                        );
                        let (updated, _) =
                            loaded_ram.V[y as usize].overflowing_sub(loaded_ram.V[x as usize]);
                        loaded_ram.V[x as usize] = updated;
                    }
                    14 => {
                        let is_one = loaded_ram.V[x as usize] >> 7 == 1;
                        // value that will be shifted out - final big end bit
                        loaded_ram.update_vf_register(is_one);
                        loaded_ram.V[x as usize] = loaded_ram.V[x as usize] << 1;
                    }
                    _ => println!("MISS::{}", n),
                },
                (0x9, _, _, _) => {
                    if loaded_ram.V[x as usize] != loaded_ram.V[y as usize] {
                        loaded_ram.PC += 2;
                    }
                }
                (0xA, _, _, _) => {
                    loaded_ram.set_i_register(nnn);
                }
                (0xB, _, _, _) => {
                    loaded_ram.PC = (loaded_ram.V[0] as u16 + nnn) as usize;
                }
                (0xC, _, _, _) => {
                    loaded_ram.V[x as usize] = rand::random::<u8>() & nn;
                }
                (0xD, _, _, _) => {
                    let sprite_start_idx = loaded_ram.I as usize;
                    let sprite_end_idx = sprite_start_idx + n as usize;

                    let sprites = &loaded_ram.mem[sprite_start_idx..sprite_end_idx]
                        .iter()
                        .copied()
                        .collect::<Vec<u8>>();

                    let coords = (loaded_ram.V[x as usize], loaded_ram.V[y as usize]);

                    let flipped_bit_callback = || {
                        loaded_ram.update_vf_register(true);
                    };

                    let draw_info = DrawInfo {
                        coords,
                        sprites: sprites.as_slice(),
                        row_count: n,
                    };

                    display.draw(draw_info, flipped_bit_callback).unwrap();
                }
                (0xE, _, _, _) => match n {
                    0xE => {
                        if is_pressed(&event_pump, loaded_ram.V[x as usize] as i32) {
                            loaded_ram.PC += 2;
                        }
                    }
                    0x1 => {
                        if !is_pressed(&event_pump, loaded_ram.V[x as usize] as i32) {
                            loaded_ram.PC += 2;
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
                        let (hundred, ten, one) = get_nums(&loaded_ram.V[x as usize]);
                        let current_idx = loaded_ram.I;
                        loaded_ram.mem[current_idx as usize] = hundred;
                        loaded_ram.mem[(current_idx + 1) as usize] = ten;
                        loaded_ram.mem[(current_idx + 2) as usize] = one;
                    }
                    5 => match y {
                        1 => loaded_ram.set_timer_register(Timer::Delay, loaded_ram.V[x as usize]),
                        5 => {
                            let current_index = loaded_ram.I as usize;
                            loaded_ram.V[0..(x + 1) as usize]
                                .iter()
                                .enumerate()
                                .for_each(|(i, val)| {
                                    loaded_ram.mem[current_index + i] = *val;
                                });
                        }
                        6 => {
                            (loaded_ram.I..loaded_ram.I + 1 + x as u16)
                                .enumerate()
                                .for_each(|(i, addr)| {
                                    println!("{}::{}", i, addr);
                                    loaded_ram.V[i] = loaded_ram.mem[addr as usize];
                                });
                        }
                        _ => (),
                    },
                    7 => {
                        let current_delay_value = loaded_ram.get_timer_register(Timer::Delay);
                        loaded_ram.set_register(x as usize, current_delay_value);
                    }
                    8 => loaded_ram.set_timer_register(Timer::Sound, loaded_ram.V[x as usize]),
                    9 => {
                        let char = loaded_ram.V[x as usize];
                        loaded_ram.I = (80 + (char * 5)) as u16;
                    }
                    0xA => {
                        if let event::Event::KeyDown {
                            scancode: Some(code),
                            ..
                        } = event_pump.wait_event()
                        {
                            kb.scancode_to_hex.get(&code).and_then(|val| {
                                loaded_ram.V[x as usize] = *val;
                                Some(())
                            });
                        }
                    }
                    0xE => {
                        let (update, did_overflow) = loaded_ram
                            .I
                            .overflowing_add(loaded_ram.V[x as usize] as u16);
                        loaded_ram.set_i_register(update);
                        loaded_ram.update_vf_register(did_overflow);
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

    Ok(())
}

fn get_nums(value: &u8) -> (u8, u8, u8) {
    let hundreds_digit = value / 100;
    let tens_digit = (value / 10) % 10;
    let ones_digit = value % 10;

    println!("HUNDREDS::{hundreds_digit}");

    (hundreds_digit, tens_digit, ones_digit)
}

fn is_pressed(e: &sdl2::EventPump, key_code: i32) -> bool {
    if let Some(scan_code) = keyboard::Scancode::from_i32(key_code) {
        return e.keyboard_state().is_scancode_pressed(scan_code);
    }

    false
}
