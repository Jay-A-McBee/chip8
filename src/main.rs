mod cli;
mod display;
mod emulator;
mod instruction;
mod ram;
mod render;
mod sys_handles;

use cli::answer::{self, Answer};
use std::io;
use std::{
    error, fs,
    io::{IoSlice, Write},
    path, result,
};
use sys_handles::keyboard::Keyboard;

use crate::cli::question::{Question, QuestionFormat};
use crate::emulator::Emulator;
use crate::ram::Ram;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

extern crate rand;
extern crate sdl2;

fn main() -> Result<()> {
    let program = fs::read(path::PathBuf::from("../../Downloads/Astro.ch8")).expect("couldnt find");
    let pg_len = program.len();
    println!("{}", pg_len);

    sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1");
    let sdl_ctx = sdl2::init().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut display = display::Display::from(&sdl_ctx);
    let mut kb = Keyboard::new();
    let mut loaded_ram = Ram::load(program.as_slice());

    let mut emu = Emulator::new(&mut loaded_ram, &mut display, &mut event_pump, &mut kb);

    emu.start();

    // let mut q1 = Question::question(
    //     "Select One",
    //     Some(QuestionFormat::Menu(vec!["Black", "Blue"])),
    // );

    // let _ = q1.ask();

    // let mut answer_q1 = Answer::response_to(&mut q1);

    // let resp = answer_q1.get_response().unwrap();

    // println!("{resp}");

    Ok(())
}
