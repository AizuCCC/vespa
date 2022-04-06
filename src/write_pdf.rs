use crate::book::Book;
use crate::size::Size;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn write_pdf(book: &Book, sz: Size, output: &Path) -> Result<()> {
    todo!();
}
