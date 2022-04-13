use anyhow::Result;
use clap::{command, Arg};
use std::fs::File;
use std::path::PathBuf;
use vespa::book::*;
use vespa::construct_pdf::*;
use vespa::main_util::*;

fn main() -> Result<()> {
    let arg = command!()
        .arg(
            Arg::new("path to book")
                .short('d')
                .value_name("DIR")
                .default_value("./")
                .help("path to book directory"),
        )
        .get_matches();

    let book_dir = PathBuf::from(arg.value_of("path to book").unwrap());
    let mut config = read_book_toml(book_dir.clone())?;
    canonicalize_path(&mut config, book_dir.as_path())?;
    validate_path(&config)?;

    let book = pagenation(&config)?;

    let mut pdf_path = book_dir;
    pdf_path.push("book.pdf");
    let doc = construct_view_pdf(&book, config.size.clone())?;
    doc.save(&mut std::io::BufWriter::new(File::create(&pdf_path)?))?;
    pdf_path.pop();

    pdf_path.push("cover.pdf");
    let doc = construct_cover(&book, config.size.clone())?;
    doc.save(&mut std::io::BufWriter::new(File::create(&pdf_path)?))?;
    pdf_path.pop();

    pdf_path.push("body.pdf");
    let doc = construct_body(&book, config.size.clone())?;
    doc.save(&mut std::io::BufWriter::new(File::create(&pdf_path)?))?;
    pdf_path.pop();

    // concat(cover, body)
    // save("print.pdf")

    Ok(())
}
