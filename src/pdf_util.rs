use crate::book::ToC;
use crate::toml_struct::Colophon;
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

    layer.set_font(font, 14.0);
    layer.set_line_height(22.0);

    layer.write_text(format!("発行者      | {}", colophon.publisher), &font);
    layer.add_line_break();
    layer.write_text(format!("発行日      | {}", colophon.date_of_issue), &font);
    layer.add_line_break();
    layer.write_text(format!("印刷        | {}", colophon.print), &font);
    layer.add_line_break();
    layer.write_text(format!("連絡先      | {}", colophon.contact), &font);

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
    layer.write_text(format!("  表紙:   {}", toc.front_author), &font);
    layer.add_line_break();
    layer.write_text(format!("  裏表紙: {}", toc.back_author), &font);
    layer.add_line_break();

    for (range, title, author) in &toc.body {
        if range.len() <= 1 {
            layer.write_text(format!("  P{}: ", range.start), &font);
        } else {
            layer.write_text(format!("  P{}-{}: ", range.start, range.end - 1), &font);
        }

        if let Some(title) = title {
            layer.write_text(format!("「{}」", title), &font);
        }

        layer.write_text(format!("{}", author), &font);

        layer.add_line_break();
    }

    layer.end_text_section();
}

pub fn add_image(
    layer: PdfLayerReference,
    img: &File,
    pos_x: Mm,
    pos_y: Mm,
    print_width: Mm,
    print_height: Mm,
) -> Result<()> {
    let reader = ImageReader::new(BufReader::new(img));
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

    image.add_to_layer(
        layer,
        ImageTransform {
            scale_x: Some(print_width / img_width),
            scale_y: Some(print_height / img_height),
            translate_x: Some(pos_x),
            translate_y: Some(pos_y),
            ..Default::default()
        },
    );
    Ok(())
}
