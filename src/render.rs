
use skia_safe::{
    surfaces,
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    Color, Color4f, Data, FontMgr, FontStyle, Image, ImageInfo, Paint, Surface,
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

pub fn render_image(args: &Args) -> Result<Image, ProgramError> {
    let bg_img = Image::from_encoded(
        Data::from_filename(args.custom_bg.clone().unwrap_or(get_res_path("xb_bg.png")?)).unwrap(),
    )
    .ok_or(ProgramError::SkiaNoImage)?;

    let width = bg_img.width();
    let height = bg_img.height();

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
    let typeface;
    if let Some(ref name) = args.font {
        if let None = font_mgr.family_names().find(|x| x == name) {
            return Err(ProgramError::NoFontFamilyName(name.clone()));
        }
        typeface = font_mgr
            .match_family(name)
            .match_style(FontStyle::default())
            .unwrap();
    } else {
        typeface = font_mgr
            .legacy_make_typeface(None, FontStyle::default())
            .unwrap();
    }
    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));
    canvas.draw_image(&bg_img, (0, 0), Some(&paint));

    let mut text_style = TextStyle::new();
    text_style.set_font_families(&[&typeface.family_name()]);
    text_style.set_font_size(args.size as f32);
    text_style.set_color(Color::BLACK);

    let mut para_style = ParagraphStyle::new();
    para_style.set_text_style(&text_style);
    para_style.set_text_align(skia_safe::textlayout::TextAlign::Center);
    para_style.set_height(height as f32);

    let mut font_col = FontCollection::new();
    font_col.set_default_font_manager(font_mgr, None);

    let mut para_builder = ParagraphBuilder::new(&para_style, font_col);
    para_builder.add_text(&args.text);
    let mut para = para_builder.build();
    para.layout(width as f32);
    let text_h = para.height();
    para.paint(canvas, (0., (height as f32 - text_h) / 2.));

    Ok(surface.image_snapshot())
}
