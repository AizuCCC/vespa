use anyhow::Result;
use std::fs::File;
use vespa::book::*;
use vespa::construct_pdf::*;
use vespa::main_util::*;

fn main() -> Result<()> {
    let (book_path, _out_dir) = parse_arg()?;
    let config = read_book_toml(book_path)?;
    validate_path(&config)?;

    let book = pagenation(&config)?;
    let doc = construct_view_pdf(&book, config.size.clone())?;
    doc.save(&mut std::io::BufWriter::new(File::create("edit.pdf")?))?;

    let book = pagenation(&config)?;
    let doc = construct_cover(&book, config.size.clone())?;
    doc.save(&mut std::io::BufWriter::new(File::create("cover.pdf")?))?;

    let book = pagenation(&config)?;
    let doc = construct_body(&book, config.size.clone())?;
    doc.save(&mut std::io::BufWriter::new(File::create("body.pdf")?))?;

    // write_toc(&book);
    Ok(())
}
