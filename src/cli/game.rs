use reqwest;

use std::{fs, io};

#[derive(Debug)]
pub struct LocalGame();

pub struct RemoteGame();

pub trait Loadable {
    type Res;
    fn load(path: &str) -> Self::Res;
}

impl Loadable for LocalGame {
    type Res = io::Result<Vec<u8>>;

    fn load(path: &str) -> Self::Res {
        fs::read(path)
    }
}

impl Loadable for RemoteGame {
    type Res = std::result::Result<Vec<u8>, reqwest::Error>;

    fn load(path: &str) -> Self::Res {
        let resp = reqwest::blocking::get(path)?;
        resp.bytes().map(|bytes| bytes.to_vec())
    }
}
