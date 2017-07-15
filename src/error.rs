use image::ImageError;

use gfx::PipelineStateError;
use gfx::CombinedError;

use hyper;

use serde_json;

use std::error;
use std::io;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::result;

pub type Result<T> = result::Result<T, ShadertoyError>;

// Custom error for failing to load shaders
#[derive(Debug)]
pub struct LoadShaderError {
    msg: String
}
impl LoadShaderError {
    pub fn new(msg: String) -> LoadShaderError {
        LoadShaderError {
            msg: msg
        }
    }
}
impl Display for LoadShaderError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Fragment shader error: {}", self.msg)
    }
}
impl error::Error for LoadShaderError {
    fn description(&self) -> &str {
        "Failed to load shader"
    }
}

// Custom error for specifying invalid shader id
#[derive(Debug)]
pub struct InvalidShaderIdError {
    id: String
}
impl InvalidShaderIdError {
    pub fn new(id: &str) -> InvalidShaderIdError {
        InvalidShaderIdError {
            id: id.to_string()
        }
    }
}
impl Display for InvalidShaderIdError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
impl error::Error for InvalidShaderIdError {
    fn description(&self) -> &str {
        "Invalid shader ID specified"
    }
}
impl<'a> From<&'a str> for InvalidShaderIdError {
    fn from(s: &'a str) -> Self {
        InvalidShaderIdError::new(s)
    }
}

// Custom error for failing to save downloaded shader
#[derive(Debug)]
pub struct SaveShaderError {
    shadername: String,
    error: io::Error,
}
impl SaveShaderError {
    pub fn new(shadername: &str, error: io::Error) -> SaveShaderError {
        SaveShaderError {
            shadername: shadername.to_string(),
            error: error,
        }
    }
}
impl Display for SaveShaderError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}): {}", self.shadername, self.error)
    }
}
impl error::Error for SaveShaderError {
    fn description(&self) -> &str {
        "Failed to save shader"
    }
}

#[derive(Debug)]
pub enum ShadertoyError {
    Parse(ParseFloatError),
    Image(ImageError),
    Texture(CombinedError),
    Pipeline(PipelineStateError<String>),
    LoadShader(LoadShaderError),
    DownloadShader(hyper::error::Error),
    Json(serde_json::Error),
    InvalidShaderId(InvalidShaderIdError),
    SaveShader(SaveShaderError),
}

impl Display for ShadertoyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ShadertoyError::Parse(ref err)           => write!(f, "Parse error: {}", err),
            ShadertoyError::Image(ref err)           => write!(f, "Image error: {}", err),
            ShadertoyError::Texture(ref err)         => write!(f, "Texture error: {}", err),
            ShadertoyError::Pipeline(ref err)        => write!(f, "Pipeline error: {}", err),
            ShadertoyError::LoadShader(ref err)      => write!(f, "Shader loading error: {}", err),
            ShadertoyError::DownloadShader(ref err)  => write!(f, "Shader download error: {}", err),
            ShadertoyError::Json(ref err)            => write!(f, "JSON error: {}", err),
            ShadertoyError::InvalidShaderId(ref err) => write!(f, "Invalid shader ID error: {}", err),
            ShadertoyError::SaveShader(ref err)      => write!(f, "Shader saving error: {}", err),
        }
    }
}

impl error::Error for ShadertoyError {
    fn description(&self) -> &str {
        match *self {
            ShadertoyError::Parse(ref err)           => error::Error::description(err),
            ShadertoyError::Image(ref err)           => err.description(),
            ShadertoyError::Texture(ref err)         => err.description(),
            ShadertoyError::Pipeline(ref err)        => err.description(),
            ShadertoyError::LoadShader(ref err)      => err.description(),
            ShadertoyError::DownloadShader(ref err)  => err.description(),
            ShadertoyError::Json(ref err)            => err.description(),
            ShadertoyError::InvalidShaderId(ref err) => err.description(),
            ShadertoyError::SaveShader(ref err)      => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ShadertoyError::Parse(ref err)           => Some(err),
            ShadertoyError::Image(ref err)           => Some(err),
            ShadertoyError::Texture(ref err)         => Some(err),
            ShadertoyError::Pipeline(ref err)        => Some(err),
            ShadertoyError::LoadShader(ref err)      => Some(err),
            ShadertoyError::DownloadShader(ref err)  => Some(err),
            ShadertoyError::Json(ref err)            => Some(err),
            ShadertoyError::InvalidShaderId(ref err) => Some(err),
            ShadertoyError::SaveShader(ref err)      => Some(err),
        }
    }
}

// Automatic error conversions
impl From<ParseFloatError> for ShadertoyError {
    fn from(pfe: ParseFloatError) -> Self {
        ShadertoyError::Parse(pfe)
    }
}
impl From<ImageError> for ShadertoyError {
    fn from(ie: ImageError) -> Self {
        ShadertoyError::Image(ie)
    }
}
impl From<CombinedError> for ShadertoyError {
    fn from(ce: CombinedError) -> Self {
        ShadertoyError::Texture(ce)
    }
}
impl From<PipelineStateError<String>> for ShadertoyError {
    fn from(pse: PipelineStateError<String>) -> Self {
        ShadertoyError::Pipeline(pse)
    }
}
impl From<hyper::error::Error> for ShadertoyError {
    fn from(he: hyper::error::Error) -> Self {
        ShadertoyError::DownloadShader(he)
    }
}
impl From<serde_json::Error> for ShadertoyError {
    fn from(je: serde_json::Error) -> Self {
        ShadertoyError::Json(je)
    }
}
