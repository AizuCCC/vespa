use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub enum StartPage {
    Left,
    Right,
    Auto,
}

#[derive(Deserialize, Debug)]
pub struct Cover {
    author: String,
    path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Body {
    author: String,
    files: Vec<PathBuf>,
    start: StartPage,
    title: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    title: String,
    editor: String,
    front: Cover,
    back: Cover,
    body: Vec<Body>,
}
