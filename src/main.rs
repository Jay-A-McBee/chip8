mod cli;
mod display;
mod emulator;
mod instruction;
mod ram;
mod render;
mod sys_handles;

use std::{error, fs, path, result};

use crate::cli::question::Question;
use crate::emulator::Emulator;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

extern crate dialoguer;
extern crate rand;
extern crate sdl2;
fn main() -> Result<()> {
    // let program = fs::read(path::PathBuf::from("../../Downloads/Astro.ch8")).expect("couldnt find");
    // let pg_len = program.len();
    // println!("{}", pg_len);

    const MENU_OPTIONS: [&str; 2] = ["Select Game", "Upload Game"];
    const AVAILABLE_GAMES: [&str; 2] = ["Astro", "Ibm"];

    sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1");

    if let Ok(Some(idx)) = Question::select(
        &Vec::from(MENU_OPTIONS),
        Some("Welcome to Chipwich -> Make a Selection"),
        Some(&0),
    ) {
        match idx {
            0 => {
                if let Ok(Some(idx)) =
                    Question::select(&Vec::from(AVAILABLE_GAMES), Some("Choose a game"), Some(&0))
                {
                    let selected_game = AVAILABLE_GAMES.get(idx).unwrap();
                    let game_path = path::PathBuf::from(format!("games/{}.ch8", selected_game));
                    let game = fs::read(&game_path).unwrap();

                    let mut emu = Emulator::new(game);
                    emu.start();
                }
            }

            _ => (),
        }
    }

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
