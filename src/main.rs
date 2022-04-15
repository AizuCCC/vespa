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
    let mut threads = vec![];

    let (tx, rx) = std::sync::mpsc::channel();

    {
        let tx = tx.clone();
        let mut pdf_path = book_dir.clone();
        pdf_path.push("book.pdf");
        let config = Arc::clone(&config);
        let book = Arc::clone(&book);
        threads.push(thread::spawn(move || -> Result<()> {
            tx.send(("book", "generating pdf in memory (uncompressed)"))?;
            let doc = construct_view_pdf(&book, config.size.clone())?;
            tx.send(("book", "compressing and saving pdf"))?;
            doc.save(&mut std::io::BufWriter::new(File::create(pdf_path.as_path())?))?;
            tx.send(("book", "done"))?;
            Ok(())
        }));
    }

    {
        let tx = tx.clone();
        let mut pdf_path = book_dir.clone();
        pdf_path.push("cover.pdf");
        let config = Arc::clone(&config);
        let book = Arc::clone(&book);
        threads.push(thread::spawn(move || -> Result<()> {
            tx.send(("cover", "generating pdf in memory (uncompressed)"))?;
            let doc = construct_cover(&book, config.size.clone())?;
            tx.send(("cover", "compressing and saving pdf"))?;
            doc.save(&mut std::io::BufWriter::new(File::create(pdf_path.as_path())?))?;
            tx.send(("cover", "done"))?;
            Ok(())
        }));
    }

    {
        let tx = tx.clone();
        let mut pdf_path = book_dir.clone();
        pdf_path.push("body.pdf");
        let config = Arc::clone(&config);
        let book = Arc::clone(&book);
        threads.push(thread::spawn(move || -> Result<()> {
            tx.send(("body", "generating pdf in memory (uncompressed)"))?;
            let doc = construct_body(&book, config.size.clone())?;
            tx.send(("body", "compressing and saving pdf"))?;
            doc.save(&mut std::io::BufWriter::new(File::create(pdf_path.as_path())?))?;
            tx.send(("body", "done"))?;
            Ok(())
        }));
    }

    drop(tx);
    for msg in rx {
        println!("{}: {}", msg.0, msg.1);
    }

    println!("all processes completed successfully");

    Ok(())
}
