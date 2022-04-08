use crate::book::Book;
use crate::book::Page::*;
use crate::pdf_util::*;
use crate::size::Size;
use anyhow::Result;
use printpdf::scale::Mm;
use printpdf::PdfDocument;
use printpdf::PdfDocumentReference;
use std::io::Cursor;

pub fn construct_view_pdf(book: &Book, sz: Size) -> Result<PdfDocumentReference> {
    let mut layer_idx = 0;
    let (width, height) = sz.into_mm();
    let (doc, p, l) = PdfDocument::new("for viewing", width, height, format!("layer{}", layer_idx));

    let mut font_reader = Cursor::new(include_bytes!("../assets/MyricaM.TTC").as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    let layer = doc.get_page(p).get_layer(l);
    add_image(
        layer.clone(),
        &book.front_cover,
        Mm(0.0),
        Mm(0.0),
        width,
        height,
    )?;
    for (idx, page) in book.page[1..].iter().enumerate() {
        layer_idx += 1;
        let (p, l) = doc.add_page(width, height, format!("layer{}", layer_idx));
        let layer = doc.get_page(p).get_layer(l);
        match page {
            BackCover => {
                add_image(
                    layer.clone(),
                    &book.back_cover,
                    Mm(0.0),
                    Mm(0.0),
                    width,
                    height,
                )?;
            }
            ToC => {
                rendering_table_of_contents(&layer, &font, &book.toc, Mm(0.0), width, height);
                rendering_page_index(&layer, &font, idx + 1, Mm(0.0), width, height);
            }
            Colophon => {
                rendering_colophon(&layer, &font, &book.colophon, Mm(0.0), width, height);
            }
            BodyImg(f) => {
                add_image(layer.clone(), f, Mm(0.0), Mm(0.0), width, height)?;
                rendering_page_index(&layer, &font, idx + 1, Mm(0.0), width, height);
            }
            Blank => {
                rendering_page_index(&layer, &font, idx + 1, Mm(0.0), width, height);
            }
            _ => panic!("unreachable pattern!"),
        }
    }

    Ok(doc)
}
