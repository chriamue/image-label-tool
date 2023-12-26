use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use image::ImageOutputFormat;
use image::Rgba;
use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use std::io::Cursor;

use crate::Annotation;

/// converts dynamic image to base64 encoded png image
pub fn image_to_base64_image(img: &DynamicImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = general_purpose::STANDARD.encode(image_data);
    format!("data:image/png;base64,{}", res_base64)
}

/// decodes base64 encoded image to dynamic image
pub fn base64_image_to_image(b64img: &str) -> DynamicImage {
    let b64img = b64img.replace("data:image/png;base64,", "");
    let data = general_purpose::STANDARD.decode(b64img).unwrap();
    image::load_from_memory(&data).unwrap()
}

/// visualizes detections on given image
pub fn visualize_annotations(image: &DynamicImage, annotations: &Vec<Annotation>) -> DynamicImage {
    let mut img_copy = image.to_rgba8();
    for annotation in annotations.iter() {
        let (bbox, class) = annotation;
        let color = Rgba([125u8, 255u8, 0u8, 0u8]);
        draw_hollow_rect_mut(
            &mut img_copy,
            Rect::at(bbox.x as i32, bbox.y as i32).of_size(bbox.w as u32, bbox.h as u32),
            color,
        );

        let font_data = include_bytes!("../res/Arial.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

        const FONT_SCALE: f32 = 10.0;

        draw_text_mut(
            &mut img_copy,
            Rgba([125u8, 255u8, 0u8, 0u8]),
            bbox.x as i32,
            bbox.y as i32,
            Scale::uniform(FONT_SCALE),
            &font,
            &format!("{}", class),
        );
    }
    DynamicImage::ImageRgba8(img_copy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bbox::BBox;

    #[test]
    fn test_image_base64_encoding_and_decoding() {
        let image = DynamicImage::ImageRgb8(image::ImageBuffer::new(100, 100));
        let encoded = image_to_base64_image(&image);
        assert!(encoded.starts_with("data:image/png;base64"));
        let decoded = base64_image_to_image(&encoded).to_rgb8();
        assert_eq!(image.width(), decoded.width());
        assert_eq!(image.height(), decoded.height());
    }

    #[test]
    fn test_visualize_annotations() {
        let det1 = (
            BBox {
                x: 0.5,
                y: 0.5,
                w: 5.0,
                h: 5.0,
            },
            0,
        );
        let det2 = (
            BBox {
                x: 0.6,
                y: 0.6,
                w: 1.0,
                h: 1.0,
            },
            1,
        );
        let annotations: Vec<Annotation> = vec![det1, det2];
        let image = DynamicImage::ImageRgb8(image::ImageBuffer::new(100, 100));
        let image = visualize_annotations(&image, &annotations).to_rgb8();
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }
}
