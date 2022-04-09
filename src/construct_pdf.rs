use crate::book::Book;
use crate::pdf_util::*;
use crate::size::Size;
use anyhow::Result;
use printpdf::scale::Mm;
use printpdf::PdfDocument;
use printpdf::PdfDocumentReference;
use std::io::Cursor;
use std::mem::swap;

fn get_font_reader() -> Cursor<&'static [u8]> {
    Cursor::new(include_bytes!("../assets/MyricaM.TTC").as_ref())
}

pub fn construct_view_pdf(book: &Book, sz: Size) -> Result<PdfDocumentReference> {
    let mut layer_idx = 0;
    let (width, height) = sz.into_mm();
    let (doc, p, l) = PdfDocument::new("for viewing", width, height, format!("layer{}", layer_idx));

    let font = doc.add_external_font(&mut get_font_reader())?;

    let layer = doc.get_page(p).get_layer(l);
    // front cover
    rendering_page(layer, &font, book, 0, Mm(0.0), width, height)?;
    for idx in 1..book.page.len() {
        layer_idx += 1;
        let (p, l) = doc.add_page(width, height, format!("layer{}", layer_idx));
        let layer = doc.get_page(p).get_layer(l);
        rendering_page(layer, &font, book, idx, Mm(0.0), width, height)?;
    }

    Ok(doc)
}

pub fn construct_cover(book: &Book, sz: Size) -> Result<PdfDocumentReference> {
    let (width, height) = sz.into_mm();
    let (doc, p, l) = PdfDocument::new("cover", width * 2.0, height, "layer1");

    let font = doc.add_external_font(&mut get_font_reader())?;

    let layer = doc.get_page(p).get_layer(l);
    rendering_page(layer.clone(), &font, book, 0, Mm(0.0), width, height)?;
    rendering_page(layer.clone(), &font, book, book.page.len() - 1, width, width, height)?;

    let (p, l) = doc.add_page(width * 2.0, height, "layer2");
    let layer = doc.get_page(p).get_layer(l);

    rendering_page(layer.clone(), &font, book, 1, width, width, height)?;
    rendering_page(layer.clone(), &font, book, book.page.len() - 2, Mm(0.0), width, height)?;

    Ok(doc)
}

pub fn construct_body(book: &Book, sz: Size) -> Result<PdfDocumentReference> {
    let mut layer_idx = 0;
    let (width, height) = sz.into_mm();
    let (doc, p, l) = PdfDocument::new("body", width * 2.0, height, format!("layer{}", layer_idx));

    let font = doc.add_external_font(&mut get_font_reader())?;

    let mut front_idx = 2;
    let mut back_idx = book.page.len() - 3;
    let mut front_shift = Mm(0.0);
    let mut back_shift = width;
    let layer = doc.get_page(p).get_layer(l);
    rendering_page(layer.clone(), &font, book, front_idx, front_shift, width, height)?;
    rendering_page(layer.clone(), &font, book, back_idx, back_shift, width, height)?;
    front_idx += 1;
    back_idx -= 1;
    swap(&mut front_shift, &mut back_shift);

    while back_idx > front_idx {
        layer_idx += 1;
        let (p, l) = doc.add_page(width * 2.0, height, format!("layer {}", layer_idx));
        let layer = doc.get_page(p).get_layer(l);
        rendering_page(layer.clone(), &font, book, front_idx, front_shift, width, height)?;
        rendering_page(layer.clone(), &font, book, back_idx, back_shift, width, height)?;
        front_idx += 1;
        back_idx -= 1;
        swap(&mut front_shift, &mut back_shift);
    }

    Ok(doc)
}
