use anyhow;

use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = anyhow::Result<T>;

// All unsupported uniforms. Attempting to use any of these in a shader will result in an error.
pub static UNSUPPORTED_UNIFORMS: [&str; 5] = [
    "iTimeDelta",
    "iChannelTime",
    "iChannelResolution",
    "iDate",
    "iSampleRate",
];

// Custom error for failing to load shaders.
#[derive(Debug)]
pub struct LoadShaderError {
    shadername: String,
    error: io::Error,
}
impl LoadShaderError {
    pub fn new(shadername: &str, error: io::Error) -> LoadShaderError {
        LoadShaderError {
            shadername: shadername.to_string(),
            error,
        }
    }
}
impl fmt::Display for LoadShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error loading shader {}: {}",
            self.shadername, self.error
        )
    }
}
impl Error for LoadShaderError {}

// Custom error for failing to find example shaders.
#[derive(Debug)]
pub struct FindExampleShaderError {
    example: String,
}
impl FindExampleShaderError {
    pub fn new(example: &str) -> FindExampleShaderError {
        FindExampleShaderError {
            example: example.to_string(),
        }
    }
}
impl Error for FindExampleShaderError {}
impl fmt::Display for FindExampleShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to find example shader {}", self.example)
    }
}

// Custom error for specifying invalid shader id.
#[derive(Debug)]
pub struct InvalidShaderIdError {
    id: String,
}
impl InvalidShaderIdError {
    pub fn new(id: &str) -> InvalidShaderIdError {
        InvalidShaderIdError { id: id.to_string() }
    }
}
impl Error for InvalidShaderIdError {}
impl fmt::Display for InvalidShaderIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid shader ID: {}", self.id)
    }
}

// Custom error for failing to save downloaded shader.
#[derive(Debug)]
pub struct SaveShaderError {
    shadername: String,
    error: io::Error,
}
impl SaveShaderError {
    pub fn new(shadername: &str, error: io::Error) -> SaveShaderError {
        SaveShaderError {
            shadername: shadername.to_string(),
            error,
        }
    }
}
impl Error for SaveShaderError {}
impl fmt::Display for SaveShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error saving shader {}: {}", self.shadername, self.error)
    }
}

// Custom error for attempting to run a shader with unsupported uniforms.
#[derive(Debug)]
pub struct UnsupportedUniformError {
    unsupported_uniforms: Vec<String>,
}
impl UnsupportedUniformError {
    pub fn new(unsupported_uniforms: Vec<String>) -> UnsupportedUniformError {
        UnsupportedUniformError {
            unsupported_uniforms,
        }
    }
}
impl Error for UnsupportedUniformError {}
impl fmt::Display for UnsupportedUniformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The following uniforms are not supported: {:?}",
            self.unsupported_uniforms
        )
    }
}
