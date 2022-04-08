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
    scale_x: f64,
    scale_y: f64,
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

    image.add_to_layer(
        layer,
        ImageTransform {
            scale_x: Some(scale_x),
            scale_y: Some(scale_y),
            translate_x: Some(pos_x),
            translate_y: Some(pos_y),
            ..Default::default()
        },
    );
    Ok(())
}
