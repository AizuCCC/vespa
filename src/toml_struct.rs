use crate::size::Size;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, PartialEq)]
pub enum StartPage {
    Left,
    Right,
    Auto,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Colophon {
    pub publisher: String,
    pub date_of_issue: toml::value::Datetime,
    pub print: String,
    pub contact: String,
}

#[derive(Deserialize, Debug)]
pub struct Cover {
    pub author: String,
    pub path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Body {
    pub author: String,
    pub files: Vec<PathBuf>,
    pub start: StartPage,
    pub title: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub size: Size,
    pub editor: String,
    pub colophon: Colophon,
    pub front: Cover,
    pub back: Cover,
    pub body: Vec<Body>,
}
