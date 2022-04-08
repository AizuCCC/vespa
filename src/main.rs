use anyhow::Result;
use std::fs::File;
use vespa::book::*;
use vespa::main_util::*;
use vespa::write_pdf::*;

fn main() -> Result<()> {
    let (book_path, _out_dir) = parse_arg()?;
    let config = read_book_toml(book_path)?;
    validate_path(&config)?;
    let book = pagenation(&config)?;

    let doc = construct_view_pdf(&book, config.size)?;
    doc.save(&mut std::io::BufWriter::new(File::create("edit.pdf")?))?;

    // let doc = construct_pagenated_pdf(&book);
    // write_toc(&book);
    Ok(())
}
