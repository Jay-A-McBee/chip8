use reqwest;

use std::{fs, io};

#[derive(Debug)]
pub struct LocalGame {
    pub path: String,
}

pub struct RemoteGame {
    pub path: String,
}

pub trait Bootable {
    type Res;
    fn boot(&self) -> Self::Res;
}

impl Bootable for LocalGame {
    type Res = io::Result<Vec<u8>>;

    fn boot(&self) -> Self::Res {
        fs::read(self.path.as_str())
    }
}

impl Bootable for RemoteGame {
    type Res = std::result::Result<Vec<u8>, reqwest::Error>;

    fn boot(&self) -> Self::Res {
        let resp = reqwest::blocking::get(self.path.as_str())?;
        resp.bytes().map(|bytes| bytes.to_vec())
    }
}
