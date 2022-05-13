mod display;
mod emulator;
mod instruction;
mod ram;
mod render;
mod sys_handles;

use std::{error, fs, path, result};
use sys_handles::keyboard::Keyboard;

use crate::emulator::Emulator;
use crate::ram::Ram;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

extern crate rand;
extern crate sdl2;

fn main() -> Result<()> {
    let program = fs::read(path::PathBuf::from("../../Downloads/Blitz.ch8")).expect("couldnt find");
    let pg_len = program.len();
    println!("{}", pg_len);

    sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1");
    let sdl_ctx = sdl2::init().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut display = display::Display::from(&sdl_ctx);
    let mut kb = Keyboard::new();
    let mut loaded_ram = Ram::new(program.as_slice());

    let mut emu = Emulator::new(&mut loaded_ram, &mut display, &mut event_pump, &mut kb);

    emu.start();

    Ok(())
}
