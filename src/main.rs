use anyhow::Result;
use vespa::book::*;
use vespa::util::*;
use vespa::write_pdf::*;

fn main() -> Result<()> {
    let (book_path, out_dir) = parse_arg()?;
    let config = read_book_toml(book_path)?;
    validate_path(&config)?;
    let book = pagenation(&config)?;

    println!("{:?}", book);
    write_pdf(&book, config.size, out_dir.as_path())?;
    // write_pagenated_pdf(&book);
    // write_toc(&book);
    Ok(())
}
