use anyhow::{Context, Result};
use printpdf::image_crate::io::Reader as ImageReader;
use printpdf::image_crate::DynamicImage;
use printpdf::scale::Mm;
use printpdf::Image;
use printpdf::ImageTransform;
use printpdf::PdfLayerReference;
use std::fs::File;
use std::io::BufReader;

pub fn add_image(
    img: &File,
    layer: PdfLayerReference,
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
