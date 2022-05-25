mod cli;
mod display;
mod emulator;
mod instruction;
mod ram;
mod sys_handles;

extern crate dialoguer;
extern crate rand;
extern crate reqwest;
extern crate sdl2;

use std::{error, fs, result};

use crate::cli::{
    game::{Loadable, LocalGame, RemoteGame},
    question::Question,
};
use crate::emulator::{Emulator, GameMode};

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

fn main() -> Result<()> {
    const MENU_OPTIONS: [&str; 3] = ["Select Game", "Load Local Game", "Download Remote Game"];
    const INTRO: &str = "__________________________________________________________
      __                                                  
    /    )    /      ,                     ,           /  
---/---------/__------------__------------------__----/__-
  /         /   )  /      /   ) | /| /   /    /   '  /   )
_(____/____/___/__/______/___/__|/_|/___/____(___ __/___/_
                        /                                 
                       /\n A rustaceous chip8 emulator\n\n";

    sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1");

    let games = fs::read_dir("games")?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                entry.file_name().to_str().map(|name| name.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let available_games: Vec<&str> = games.iter().map(|val| val.as_str()).collect::<Vec<&str>>();

    println!("{INTRO}");

    if let Ok(Some(idx)) = Question::select(&MENU_OPTIONS, Some("Make a Selection"), Some(&0)) {
        match idx {
            0 => {
                if let Ok(Some(idx)) =
                    Question::select(&available_games, Some("Choose a game"), Some(&0))
                {
                    let selected = *available_games.get(idx).unwrap();
                    let path = format!("games/{}", selected);
                    let program = LocalGame::load(path.as_str())?;

                    let mut emu = Emulator::boot(program, GameMode::Standard);
                    emu.start();
                }
            },
            1 => {
                if let Ok(file_path) = Question::input((Some("Type in the path to the game\n This should be an absolute file path. (Ex. /Users/SomeUser/documents/games/blah.ch8)"), None, None)) {
                    let program = LocalGame::load(file_path.as_str())?;
                    let mut emu = Emulator::boot(program, GameMode::Standard);
                    emu.start();
                }
            },
            2 => {
                if let Ok(url) = Question::input((Some("Type in the url of the game to download."), None, None)) {
                    println!("Downloading -> {}", &url);

                    let program = RemoteGame::load(&url)?;
                    let mut emu = Emulator::boot(program, GameMode::Standard);
                    emu.start();
                }
            },

            _ => ()
        }
    }

    Ok(())
}
