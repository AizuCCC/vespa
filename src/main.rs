use anyhow::Result;
use clap::{command, Arg};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
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

    let config = Arc::new(config);
    let book = Arc::new(pagenation(&config)?);
    let mut v = vec![];

    {
        let mut pdf_path = book_dir.clone();
        let config = Arc::clone(&config);
        let book = Arc::clone(&book);
        v.push(thread::spawn(move || -> Result<()> {
            pdf_path.push("book.pdf");
            let doc = construct_view_pdf(&book, config.size.clone())?;
            doc.save(&mut std::io::BufWriter::new(File::create(pdf_path.as_path())?))?;
            Ok(())
        }));
    }

    {
        let mut pdf_path = book_dir.clone();
        let config = Arc::clone(&config);
        let book = Arc::clone(&book);
        v.push(thread::spawn(move || -> Result<()> {
            pdf_path.push("cover.pdf");
            let doc = construct_cover(&book, config.size.clone())?;
            doc.save(&mut std::io::BufWriter::new(File::create(pdf_path.as_path())?))?;
            Ok(())
        }));
    }

    {
        let mut pdf_path = book_dir.clone();
        let config = Arc::clone(&config);
        let book = Arc::clone(&book);
        v.push(thread::spawn(move || -> Result<()> {
            pdf_path.push("body.pdf");
            let doc = construct_body(&book, config.size.clone())?;
            doc.save(&mut std::io::BufWriter::new(File::create(pdf_path.as_path())?))?;
            Ok(())
        }));
    }

    for h in v {
        if let Err(e) = h.join().unwrap() {
            println!("{:?}", e);
        }
    }

    Ok(())
}
