use crate::config::Config;
use anyhow::{anyhow, bail, Context, Result};
use std::io::prelude::Read;
use std::path::{Path, PathBuf};

pub fn read_book_toml(mut path: PathBuf) -> Result<Config> {
    path.push("book.toml");
    let mut config = std::fs::File::open(path.as_path()).context("cannot open or not found book.toml")?;
    let mut buf = String::new();
    config.read_to_string(&mut buf).context("cannot read book.toml")?;
    let config: Config = toml::from_str(&buf).context("parse error in book.toml")?;
    Ok(config)
}

pub fn canonicalize_path(config: &mut Config, book_path: &Path) -> Result<()> {
    let concat = |p1, p2: &Path| {
        let mut ret = PathBuf::from(p1);
        ret.push(p2);
        ret
    };

    config.front.path = concat(book_path, config.front.path.as_path())
        .canonicalize()
        .with_context(|| format!("{:?}", config.front.path))?;
    config.back.path = concat(book_path, config.back.path.as_path())
        .canonicalize()
        .with_context(|| format!("{:?}", config.back.path))?;

    for body in &mut config.body {
        for path in &mut body.files {
            *path = concat(book_path, path.as_path())
                .canonicalize()
                .with_context(|| format!("{:?}", path))?;
        }
    }

    Ok(())
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
        bail!("unsupported format {:?}", e);
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
