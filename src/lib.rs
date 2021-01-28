use glyph_brush_layout::{
    rusttype::{Font, Scale},
    *,
};
use image::{DynamicImage, Rgba};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub fn generate_text(
    title: &str,
    author: &str,
    title_font_size: i32,
    subtitle_font_size: i32,
    rgb_value: (u8, u8, u8),
    font_style: &str,
    font_file: Vec<u8>,
) -> Vec<u8> {
    const WIDTH: f32 = 1200.0;
    const HEIGHT: f32 = 630.0;
    const PADDING: f32 = 50.0;

    let font = if font_style == "monospace" {
        Font::from_bytes(&include_bytes!("../fonts/Inconsolata-Medium.ttf")[..])
            .expect("Error constructing Font")
    } else if font_style == "sans-serif" {
        Font::from_bytes(&include_bytes!("../fonts/OpenSans-Regular.ttf")[..])
            .expect("Error constructing Font")
    } else {
        Font::from_bytes(font_file).expect("Error constructing Font")
    };

    let fonts = vec![font];
    let bounds = (WIDTH - (PADDING * 2.0), HEIGHT - (PADDING * 2.0));

    let title_glyphs: Vec<_> = Layout::default().calculate_glyphs(
        &fonts,
        &SectionGeometry {
            screen_position: (PADDING, PADDING),
            bounds: bounds,
        },
        &[SectionText {
            text: title,
            scale: Scale::uniform(title_font_size as f32),
            font_id: FontId(0),
            color: [0.0, 1.0, 0.0, 1.0],
        }],
    );

    let author_glyphs: Vec<_> = Layout::default().calculate_glyphs(
        &fonts,
        &SectionGeometry {
            screen_position: (PADDING, HEIGHT - PADDING - subtitle_font_size as f32),
            bounds: bounds,
        },
        &[SectionText {
            text: author,
            scale: Scale::uniform(subtitle_font_size as f32),
            font_id: FontId(0),
            color: [0.0, 1.0, 0.0, 1.0],
        }],
    );

    // Create a new rgba image
    let mut image = DynamicImage::new_rgba8(WIDTH as u32, HEIGHT as u32).to_rgba();

    for glyph in title_glyphs {
        if let Some(bounding_box) = glyph.0.pixel_bounding_box() {
            glyph.0.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    for glyph in author_glyphs {
        if let Some(bounding_box) = glyph.0.pixel_bounding_box() {
            glyph.0.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    return image.to_vec();
}
