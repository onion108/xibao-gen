use std::num::ParseIntError;

use sdl2::{video::WindowBuildError, IntegerOrSdlError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("cannot find resource file")]
    ResourceNotFound { path: String },

    #[error("failed at utf8 validity check, maybe using some evil non-utf8 encoding 😈")]
    EncodingError,

    #[error(transparent)]
    ArgParseError(#[from] ParseIntError),

    #[error("please specify argument for flag {0}")]
    ArgParseMissingFlagValue(String),

    #[error("please specify content text")]
    ArgParseMissingContent,

    #[error("cannot initialize skia canvas")]
    SkiaNoCanvas,

    #[error("cannot initialize skia surface")]
    SkiaNoSurface,

    #[error("cannot read or generate image data")]
    SkiaNoImage,

    #[error("SDL2 pooped itself: {0}")]
    SDLError(String),

    #[error(transparent)]
    SDLIntegerOrError(#[from] IntegerOrSdlError),

    #[error(transparent)]
    SDLWindowBuildError(#[from] WindowBuildError),
}
