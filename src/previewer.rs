use sdl2::{event::Event, image::{InitFlag, LoadTexture}, keyboard::Keycode};
use skia_safe::Data;

use crate::error::ProgramError;

pub fn run_preview(png_image: Data, width: i32, height: i32) -> Result<(), ProgramError> {
    let sdl_ctx = sdl2::init().map_err(|x| ProgramError::SDLError(x))?;
    let _ = sdl2::image::init(InitFlag::PNG).map_err(|x| ProgramError::SDLError(x))?;
    let video_subsystem = sdl_ctx.video().map_err(|x| ProgramError::SDLError(x))?;
    let window = video_subsystem
        .window("Preview", width as u32, height as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(&png_image).map_err(|x| ProgramError::SDLError(x))?;
    
    canvas.copy(&texture, None, None).map_err(|x| ProgramError::SDLError(x))?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_ctx.event_pump().map_err(|x| ProgramError::SDLError(x))?.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => {
                    break 'mainloop;
                }
                _ => {}
            }
        }
    }


    Ok(())
}

