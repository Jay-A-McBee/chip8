use reqwest;

use std::{fs, io};

#[derive(Debug)]
pub struct LocalGame();

pub struct RemoteGame();

pub trait Bootable {
    type Res;
    fn boot(path: &str) -> Self::Res;
}

impl Bootable for LocalGame {
    type Res = io::Result<Vec<u8>>;

    fn boot(path: &str) -> Self::Res {
        fs::read(path)
    }
}

impl Bootable for RemoteGame {
    type Res = std::result::Result<Vec<u8>, reqwest::Error>;

    fn boot(path: &str) -> Self::Res {
        let resp = reqwest::blocking::get(path)?;
        resp.bytes().map(|bytes| bytes.to_vec())
    }
}
