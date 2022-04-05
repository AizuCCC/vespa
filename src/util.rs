use crate::toml_struct::Config;
use anyhow::{bail, Context, Result};
use std::io::prelude::Read;
use std::path::PathBuf;

pub fn parse_arg() -> Result<(PathBuf, PathBuf)> {
    if std::env::args().count() < 3 {
        bail!("missing argument!");
    }
    let args: Vec<String> = std::env::args().collect();
    let book_path = std::path::PathBuf::from(&args[1]);
    let out_dir = std::path::PathBuf::from(&args[2]);

    if !book_path.is_dir() {
        bail!("book: {:?} is not directory", book_path);
    }
    if !out_dir.is_dir() {
        bail!("output directory: {:?} is not directory", out_dir);
    }

    Ok((book_path, out_dir))
}

pub fn read_book_toml(mut path: PathBuf) -> Result<Config> {
    path.push("book.toml");
    let mut config = std::fs::File::open(path).context("cannot open or not found book.toml")?;
    let mut buf = String::new();
    config
        .read_to_string(&mut buf)
        .context("cannot read book.toml")?;
    let config: Config = toml::from_str(&buf).context("parse error in book.toml")?;
    Ok(config)
}
