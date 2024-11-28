use std::num::ParseIntError;

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

    #[error("cannot initialize skia canvas")]
    SkiaNoCanvas,

    #[error("cannot initialize skia surface")]
    SkiaNoSurface,

    #[error("cannot read or generate image data")]
    SkiaNoImage,
}

