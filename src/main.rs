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
    const MENU_OPTIONS: [&str; 2] = ["Select Game", "Upload Game"];
    const INTRO: &str = "__________________________________________________________
      __                                                  
    /    )    /      ,                     ,           /  
---/---------/__------------__------------------__----/__-
  /         /   )  /      /   ) | /| /   /    /   '  /   )
_(____/____/___/__/______/___/__|/_|/___/____(___ __/___/_
                        /                                 
                       /\n A chip8 emulator written in Rust\n\n";

    sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1");

    let games = fs::read_dir("games")
        .unwrap()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    Some(name.to_owned())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let available_games = games.iter().map(|val| val.as_str()).collect::<Vec<&str>>();

    println!("{INTRO}");

    if let Ok(Some(idx)) =
        Question::select(&Vec::from(MENU_OPTIONS), Some("Make a Selection"), Some(&0))
    {
        match idx {
            0 => {
                if let Ok(Some(idx)) =
                    Question::select(&available_games, Some("Choose a game"), Some(&0))
                {
                    let selected_game = available_games.get(idx).unwrap();
                    let game_path = path::PathBuf::from(format!("games/{}", selected_game));
                    let game = fs::read(&game_path).unwrap();

                    let mut emu = Emulator::new(game);
                    emu.start();
                }
            }

            _ => {
                if let Ok(file_path) = Question::input((Some(&"Type in the path to the game\n This can accept an absolute file path or an http/https url"), None, None)) {
                    if file_path.starts_with("http") || file_path.starts_with("https") {
                        todo!()
                    } else {
                        println!("{file_path}");
                        let game = fs::read(file_path).unwrap();
                        let mut emu = Emulator::new(game);
                        emu.start();
                    }
                }
            },
        }
    }

    Ok(())
}
