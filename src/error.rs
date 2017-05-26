use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;

use image::ImageError;

use gfx::PipelineStateError;
use gfx::CombinedError;

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

#[derive(Debug)]
pub enum ShadertoyError {
    Parse(ParseFloatError),
    Image(ImageError),
    Texture(CombinedError),
    Pipeline(PipelineStateError<String>),
    LoadShader(LoadShaderError),
}

impl Display for ShadertoyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ShadertoyError::Parse(ref err)      => write!(f, "Parse error: {}", err),
            ShadertoyError::Image(ref err)      => write!(f, "Image error: {}", err),
            ShadertoyError::Texture(ref err)    => write!(f, "Texture error: {}", err),
            ShadertoyError::Pipeline(ref err)   => write!(f, "Pipeline error: {}", err),
            ShadertoyError::LoadShader(ref err) => write!(f, "Shader loading error: {}", err),
        }
    }
}

impl error::Error for ShadertoyError {
    fn description(&self) -> &str {
        match *self {
            ShadertoyError::Parse(ref err)      => error::Error::description(err),
            ShadertoyError::Image(ref err)      => err.description(),
            ShadertoyError::Texture(ref err)    => err.description(),
            ShadertoyError::Pipeline(ref err)   => err.description(),
            ShadertoyError::LoadShader(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ShadertoyError::Parse(ref err)      => Some(err),
            ShadertoyError::Image(ref err)      => Some(err),
            ShadertoyError::Texture(ref err)    => Some(err),
            ShadertoyError::Pipeline(ref err)   => Some(err),
            ShadertoyError::LoadShader(ref err) => Some(err),
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
