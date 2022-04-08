use crate::book::Book;
use crate::book::Page::*;
use crate::pdf_util::*;
use crate::size::Size;
use anyhow::Result;
use printpdf::scale::Mm;
use printpdf::PdfDocument;
use printpdf::PdfDocumentReference;

pub fn construct_browse_pdf(book: &Book, sz: Size) -> Result<PdfDocumentReference> {
    let mut layer_idx = 0;
    let (width, height) = sz.into_mm();
    let (doc, p, l) =
        PdfDocument::new("for browsing", width, height, format!("layer{}", layer_idx));

    let layer = doc.get_page(p).get_layer(l);
    add_image(
        &book.front_cover,
        layer.clone(),
        Mm(0.0),
        Mm(0.0),
        width,
        height,
    )?;

    for page in &book.page[1..] {
        layer_idx += 1;
        let (p, l) = doc.add_page(width, height, format!("layer{}", layer_idx));
        let layer = doc.get_page(p).get_layer(l);
        match page {
            BackCover => {
                add_image(
                    &book.back_cover,
                    layer.clone(),
                    Mm(0.0),
                    Mm(0.0),
                    width,
                    height,
                )?;
            }
            ToC => {}
            Colophon => {}
            BodyImg(f) => {
                add_image(f, layer.clone(), Mm(0.0), Mm(0.0), width, height)?;
            }
            Blank => {}
            _ => panic!("unreachable pattern!"),
        }
    }

    Ok(doc)
}
