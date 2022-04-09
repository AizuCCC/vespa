use crate::book::Book;
use crate::book::Colophon;
use crate::book::Page::*;
use crate::book::ToC;
use anyhow::{Context, Result};
use printpdf::image_crate::io::Reader as ImageReader;
use printpdf::image_crate::DynamicImage;
use printpdf::scale::Mm;
use printpdf::Image;
use printpdf::ImageTransform;
use printpdf::IndirectFontRef;
use printpdf::PdfLayerReference;
use std::fs::File;
use std::io::BufReader;

pub fn rendering_page(
    layer: PdfLayerReference,
    font: &IndirectFontRef,
    book: &Book,
    page_idx: usize,
    offset_width: Mm,
    width: Mm,
    height: Mm,
) -> Result<()> {
    let page = &book.page[page_idx];
    match page {
        FrontCover => {
            add_image(
                layer,
                File::open(book.front_cover.as_path()).with_context(|| format!("cannot open {:?}", book.front_cover))?,
                Mm(0.0),
                Mm(0.0),
                width,
                height,
            )?;
        }
        BackCover => {
            add_image(
                layer,
                File::open(book.back_cover.as_path()).with_context(|| format!("cannot open {:?}", book.back_cover))?,
                offset_width,
                Mm(0.0),
                width,
                height,
            )?;
        }
        ToC => {
            rendering_table_of_contents(&layer, &font, &book.toc, offset_width, width, height);
        }
        Colophon => {
            rendering_colophon(&layer, &font, &book.colophon, offset_width, width, height);
        }
        BodyImg(p) => {
            add_image(layer.clone(), File::open(p)?, offset_width, Mm(0.0), width, height)?;
            rendering_page_index(&layer, &font, page_idx + 1, offset_width, width, height);
        }
        Blank => {
            rendering_page_index(&layer, &font, page_idx + 1, offset_width, width, height);
        }
    }
    Ok(())
}

pub fn rendering_page_index(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    page_idx: usize,
    offset_width: Mm,
    page_width: Mm,
    _page_height: Mm,
) {
    layer.begin_text_section();
    layer.set_font(font, 12.0);
    match page_idx % 4 {
        // 紙の表裏と見開きの左右で印刷箇所場合分け
        1 | 3 => {
            layer.set_text_cursor(Mm(5.0), Mm(5.0));
            layer.write_text(format!("{}", page_idx), &font);
        }
        0 | 2 => {
            layer.set_text_cursor(offset_width + page_width - Mm(8.0), Mm(5.0));
            layer.write_text(format!("{}", page_idx), &font);
        }
        _ => panic!("unreachable path"),
    }
}

pub fn rendering_colophon(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    colophon: &Colophon,
    offset_width: Mm,
    _page_width: Mm,
    page_height: Mm,
) {
    layer.begin_text_section();

    layer.set_font(font, 24.0);
    layer.set_line_height(30.0);
    layer.set_text_cursor(offset_width + Mm(10.0), page_height / 4.0);

    layer.write_text("奥付", &font);
    layer.add_line_break();
    layer.set_font(font, 20.0);
    layer.write_text(&colophon.title, &font);
    layer.add_line_break();

    layer.set_font(font, 14.0);
    layer.set_line_height(22.0);

    layer.write_text(format!("発行者  | {}", colophon.publisher), &font);
    layer.add_line_break();
    layer.write_text(format!("発行日  | {}", colophon.date_of_issue), &font);
    layer.add_line_break();
    layer.write_text(format!("印刷    | {}", colophon.print), &font);
    layer.add_line_break();
    layer.write_text(format!("連絡先  | {}", colophon.contact), &font);

    layer.end_text_section();
}

pub fn rendering_table_of_contents(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    toc: &ToC,
    offset_width: Mm,
    page_width: Mm,
    page_height: Mm,
) {
    layer.begin_text_section();

    layer.set_font(font, 24.0);
    layer.set_line_height(30.0);
    layer.set_text_cursor(offset_width + page_width / 5.0, page_height / 8.0 * 7.0);

    layer.write_text(&toc.title, &font);
    layer.add_line_break();
    layer.write_text("目次", &font);
    layer.add_line_break();

    layer.set_font(font, 20.0);
    layer.write_text(format!("  表紙   :   {}", toc.front_author), &font);
    layer.add_line_break();
    layer.write_text(format!("  裏表紙 : {}", toc.back_author), &font);
    layer.add_line_break();

    for (range, title, author) in &toc.body {
        if range.len() <= 1 {
            layer.write_text(format!("  P{}    : ", range.start), &font);
        } else {
            layer.write_text(format!("  P{}-{} : ", range.start, range.end - 1), &font);
        }

        if let Some(title) = title {
            layer.write_text(format!("\"{}\"", title), &font);
        }

        layer.write_text(format!("{}", author), &font);

        layer.add_line_break();
    }
    layer.write_text(format!("  編集   : {}", toc.editor), &font);

    layer.end_text_section();
}

pub fn add_image(layer: PdfLayerReference, img: File, pos_x: Mm, pos_y: Mm, print_width: Mm, print_height: Mm) -> Result<()> {
    let reader = ImageReader::new(BufReader::new(img.try_clone()?));
    let image = Image::from_dynamic_image(&DynamicImage::from(DynamicImage::ImageRgb8(
        reader
            .with_guessed_format()
            .with_context(|| format!("{:?}", img))?
            .decode()
            .with_context(|| format!("{:?}", img))?
            .into_rgb8(),
    )));

    let print_width = (print_width * 300.0 / 25.4).0; // dpi = 300
    let print_height = (print_height * 300.0 / 25.4).0;
    let img_width = image.image.width.0 as f64;
    let img_height = image.image.height.0 as f64;
    let scale = (print_width / img_width).min(print_height / img_height);

    image.add_to_layer(
        layer,
        ImageTransform {
            scale_x: Some(scale),
            scale_y: Some(scale),
            translate_x: Some(pos_x),
            translate_y: Some(pos_y),
            ..Default::default()
        },
    );
    Ok(())
}
