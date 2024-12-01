use std::iter::zip;

use skia_safe::{
    surfaces, Canvas, Color, Color4f, Data, Font, FontMgr, FontStyle, Image, ImageInfo, Paint,
    Point, Shaper, Surface,
};

use crate::{argparse::Args, error::ProgramError, resource::get_res_path};

pub fn create_surface(width: i32, height: i32) -> Result<Surface, ProgramError> {
    surfaces::raster(
        &ImageInfo::new(
            (width, height),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Opaque,
            None,
        ),
        None,
        None,
    )
    .ok_or(ProgramError::SkiaNoSurface)
}

pub fn render_image(width: i32, height: i32, args: &Args) -> Result<Image, ProgramError> {
    let mut surface = create_surface(width, height)?;
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::BLACK);

    let mut debug_painter = Paint::default();
    debug_painter.set_color(Color::BLUE);
    debug_painter.set_style(skia_safe::PaintStyle::Stroke);
    debug_painter.set_stroke_width(4.0);

    let font_mgr = FontMgr::new();
    let default_typeface = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();

    let bg_img = Image::from_encoded(Data::from_filename(get_res_path("xb_bg.png")?).unwrap())
        .ok_or(ProgramError::SkiaNoImage)?;

    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));
    canvas.draw_image(&bg_img, (0, 0), Some(&paint));

    // Render text on the image
    let font = &Font::from_typeface(default_typeface, args.size as f32);
    render_lines(
        &canvas,
        &paint,
        &font,
        &args
            .text
            .lines()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>(),
        (width as f32, height as f32),
    );

    Ok(surface.image_snapshot())
}

fn render_lines(
    canvas: &Canvas,
    paint: &Paint,
    font: &Font,
    lines: &[&str],
    window_wh: (f32, f32),
) {
    let shaper = Shaper::new(FontMgr::new());
    let mut blobs_to_render = Vec::new();

    // Shape all the text for rendering.
    for line in lines {
        if let Some((blob, _)) = shaper.shape_text_blob(line, font, true, 99999.0, Point::default())
        {
            blobs_to_render.push(blob);
        } else {
            println!("[WARNING] failed to shape text {:?}", line);
        }
    }

    // Calculate positions to render.
    let total_height = blobs_to_render.iter().fold(0.0, |acc, blob| {
        let bounds = blob.bounds();
        acc + (bounds.bottom - bounds.top)
    });
    let mut height_acc = 0.0f32;

    let positions_to_render = blobs_to_render
        .iter()
        // Calculate horizontal positions first. Every line will be centered.
        .map(|blob| {
            let bounds = blob.bounds();
            let text_width = bounds.right - bounds.left;
            let text_height = bounds.bottom - bounds.top;
            let text_x = (window_wh.0 - text_width) / 2.0 - bounds.left;
            let offset_y = -bounds.top;
            (text_x, f32::NAN, text_height, offset_y)
        })
        // Then calculate vertical positions.
        .map(|(x, y, text_height, offset_y)| {
            let y_start = (window_wh.1 - total_height) / 2.0;
            (x, y, text_height, offset_y, y_start)
        })
        .map(|(x, _, height, offset_y, y_start)| {
            let actual_y = y_start + height_acc + offset_y;
            height_acc += height;
            (x, actual_y)
        })
        .collect::<Vec<_>>();

    for (blob, pos) in zip(blobs_to_render, positions_to_render) {
        canvas.draw_text_blob(blob, pos, paint);
    }
}
