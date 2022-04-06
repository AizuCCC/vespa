use anyhow::Result;
use vespa::book::*;
use vespa::util::*;

fn main() -> Result<()> {
    let (book_path, out_dir) = parse_arg()?;
    let config = read_book_toml(book_path)?;
    validate_path(&config)?;
    let book = pagenation(&config)?;

    println!("{:?}", book);
    // write_pdf(&book);
    // write_pagenated_pdf(&book);
    // write_toc(&book);
    Ok(())
}
