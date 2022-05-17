use crate::reqwest;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::{fs, io};
#[derive(Debug)]
pub struct LocalGame {
    pub path: String,
}

pub struct RemoteGame {
    pub path: String,
}

pub trait Playable {
    type Res;
    fn boot(&self) -> Self::Res;
}

impl Playable for LocalGame {
    type Res = io::Result<Vec<u8>>;

    fn boot(&self) -> Self::Res {
        let contents = fs::read(self.path.as_str()).unwrap();
        Ok(contents)
    }
}

impl Playable for RemoteGame {
    type Res = std::result::Result<Vec<u8>, reqwest::Error>;

    fn boot(&self) -> Self::Res {
        let resp = reqwest::blocking::get(self.path.as_str())?;
        resp.bytes().map(|bytes| bytes.to_vec())
    }
}

#[derive(Debug)]
pub struct Question();

impl Question {
    pub fn select(
        options: &[&str],
        prompt: Option<&str>,
        default: Option<&usize>,
    ) -> std::io::Result<Option<usize>> {
        let default_idx = if let Some(&default_idx) = default {
            default_idx
        } else {
            0
        };

        let prompt = if let Some(prompt) = prompt {
            prompt
        } else {
            "Make a selection"
        };

        Select::with_theme(&ColorfulTheme::default())
            .items(options)
            .with_prompt(prompt)
            .default(default_idx)
            .interact_on_opt(&Term::stderr())
    }

    pub fn input<'a>(
        (prompt, initial_text, default): (Option<&'a str>, Option<&'a str>, Option<&'a str>),
    ) -> std::io::Result<String> {
        let mut input = Input::<String>::new();

        if let Some(prompt) = prompt {
            input.with_prompt(prompt);
        }

        if let Some(initial_text) = initial_text {
            input.with_initial_text(initial_text);
        }

        if let Some(default) = default {
            input.default(default.to_string());
        }

        input.interact()
    }
}

pub trait Read {
    fn read(buffer: String) -> std::io::Result<String>;
}
