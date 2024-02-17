use image::{imageops, ImageBuffer, Rgba};
use qrcode_generator::QrCodeEcc;
use std::io::Cursor;

use crate::Options;

pub(super) fn generate(
    input: String,
    options: Options,
    logo: &[u8],
    image_size: usize,
) -> Result<Vec<u8>, anyhow::Error> {
    let mut qr = image::DynamicImage::ImageLuma8(qrcode_generator::to_image_buffer(
        input,
        QrCodeEcc::Quartile,
        image_size,
    )?)
    .into_rgba8();

    if options.add_transparency == Some(true) {
        make_transparent(&mut qr);
    }

    if options.add_logo {
        add_logo(&mut qr, logo);
    }

    if options.add_gradient {
        add_gradient(&mut qr);
    }

    let mut result = vec![];
    qr.write_to(&mut Cursor::new(&mut result), image::ImageOutputFormat::Png)?;
    Ok(result)
}

fn make_transparent(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (_x, _y, pixel) in qr.enumerate_pixels_mut() {
        if pixel.0 == [255, 255, 255, 255] {
            *pixel = image::Rgba([255, 255, 255, 0]);
        }
    }
}


fn add_logo(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, logo: &[u8]) {
    let image_size = qr.width().min(qr.height()) as usize;
    let element_size = get_qr_element_size(qr);

    let mut logo_size = element_size;

    while logo_size + 2 * element_size <= 5 * image_size / 16 {
        logo_size += 2 * element_size;
    }

    let mut logo = image::io::Reader::new(Cursor::new(logo))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    logo = logo.resize(
        logo_size as u32,
        logo_size as u32,
        imageops::FilterType::Lanczos3,
    );

    imageops::replace(
        qr,
        &logo,
        ((image_size - logo_size) / 2) as i64,
        ((image_size - logo_size) / 2) as i64,
    );
}

fn add_gradient(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let image_size = qr.width().min(qr.height()) as usize;

    let gradient = colorgrad::CustomGradient::new()
        .colors(&[
            colorgrad::Color::from_rgba8(100, 0, 100, 255),
            colorgrad::Color::from_rgba8(30, 5, 60, 255),
        ])
        .build()
        .unwrap();

    let center = (image_size / 2) as u32;
    for (x, y, pixel) in qr.enumerate_pixels_mut() {
        if pixel.0 == [0, 0, 0, 255] {
            let distance = x.abs_diff(center) + y.abs_diff(center);
            let rgba = gradient.at(distance as f64 / image_size as f64).to_rgba8();
            *pixel = image::Rgba(rgba);
        }
    }
}


fn get_qr_element_size(qr: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> usize {
    const BLACK_PIXEL: [u8; 4] = [0, 0, 0, 255];

    let size = qr.width().min(qr.height());

    let mut start = size;
    for i in 0..size {
        if qr.get_pixel(i, i).0 == BLACK_PIXEL {
            start = i;
            break;
        }
    }

    let mut element_size = 1;
    for i in 0..size - start {
        if qr.get_pixel(start + i, start + i).0 != BLACK_PIXEL {
            element_size = i;
            break;
        }
    }

    element_size as usize
}