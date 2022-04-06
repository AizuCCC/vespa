use crate::toml_struct::Config;
use anyhow::{anyhow, bail, Context, Result};
use std::io::prelude::Read;
use std::path::{Path, PathBuf};

pub fn parse_arg() -> Result<(PathBuf, PathBuf)> {
    Ok((PathBuf::from("."), PathBuf::from(".")))
    /*
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
    */
}

pub fn read_book_toml(mut path: PathBuf) -> Result<Config> {
    path.push("book.toml");
    let mut config =
        std::fs::File::open(path.as_path()).context("cannot open or not found book.toml")?;
    let mut buf = String::new();
    config
        .read_to_string(&mut buf)
        .context("cannot read book.toml")?;
    let config: Config = toml::from_str(&buf).context("parse error in book.toml")?;
    Ok(config)
}

pub fn check_extension(p: &Path) -> Result<()> {
    if p.extension().is_none() {
        bail!("invalid file-type {:?}", p)
    }
    let ext = ["png", "jpg", "jpeg"]; // enum使うべきか？
    for e in ext {
        if p.extension().unwrap() == e {
            return Ok(());
        }
    }
    bail!("invalid file-type {:?}", p)
}

fn validate_path_(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("not found {:?}", path);
    } else if let Err(e) = check_extension(path) {
        bail!("{:?}", e);
    }
    Ok(())
}

pub fn validate_path(config: &Config) -> Result<()> {
    let mut err = String::new();

    if let Err(e) = validate_path_(config.front.path.as_path()) {
        err += &format!("front cover: {:?}\n", e);
    }
    if let Err(e) = validate_path_(config.back.path.as_path()) {
        err += &format!("back cover: {:?}\n", e);
    }

    for body in &config.body {
        for path in &body.files {
            if let Err(e) = validate_path_(path.as_path()) {
                err += &format!("body: {:?}\n", e);
            }
        }
    }

    if err.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(err))
    }
}
