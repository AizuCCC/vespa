use anyhow::Result;
use vespa::util::*;

fn main() -> Result<()> {
    let (book_path, out_dir) = parse_arg()?;

    let config = read_book_toml(book_path)?;

    // book.toml内のファイルパスを精査する
    todo!();
}
