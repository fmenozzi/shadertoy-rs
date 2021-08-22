use argvalues::ArgValues;
use error::{
    self, FindExampleShaderError, LoadShaderError, UnsupportedUniformError, UNSUPPORTED_UNIFORMS,
};
use runner::TextureId;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use gfx;
use image;

// Default shaders
pub static DEFAULT_VERT_SRC_BUF: &'static [u8] = include_bytes!("../shaders/default.vert");
pub static DEFAULT_FRAG_SRC_STR: &'static str = include_str!("../shaders/default.frag");

// Default textures
pub static DEFAULT_TEXTURE0_BUF: &'static [u8] = include_bytes!("../textures/01-brickwall.jpg");
pub static DEFAULT_TEXTURE1_BUF: &'static [u8] = include_bytes!("../textures/02-landscape.jpg");
pub static DEFAULT_TEXTURE2_BUF: &'static [u8] = include_bytes!("../textures/03-whitenoise.jpg");
pub static DEFAULT_TEXTURE3_BUF: &'static [u8] = include_bytes!("../textures/04-woodgrain.jpg");

// Example shaders
pub static EXAMPLE_SEASCAPE_STR: &'static str = include_str!("../examples/seascape.frag");
pub static EXAMPLE_ELEMENTAL_RING_STR: &'static str =
    include_str!("../examples/elemental-ring.frag");

// Fragment shader prefix
const PREFIX: &str = "
    #version 150 core

    uniform float     iGlobalTime;
    uniform float     iTime;
    uniform vec3      iResolution;
    uniform vec4      iMouse;
    uniform int       iFrame;
    uniform sampler2D iChannel0;
    uniform sampler2D iChannel1;
    uniform sampler2D iChannel2;
    uniform sampler2D iChannel3;

    in vec2 fragCoord;
    out vec4 fragColor;
";

// Fragment shader suffix
const SUFFIX: &str = "
    void main() {
        mainImage(fragColor, fragCoord);
    }
";

fn return_load_shader_error<E>(shaderpath: &str, err: io::Error) -> error::Result<E> {
    Err(LoadShaderError::new(shaderpath, err).into())
}

pub fn format_shader_src(src: &str) -> Vec<u8> {
    format!("{}\n{}\n{}", PREFIX, src, SUFFIX).into_bytes()
}

pub fn load_fragment_shader(av: &ArgValues) -> error::Result<Vec<u8>> {
    let frag_src_str = if let Some(ref example) = av.examplename {
        match example.as_ref() {
            "seascape" => EXAMPLE_SEASCAPE_STR.to_string(),
            "elemental-ring" => EXAMPLE_ELEMENTAL_RING_STR.to_string(),
            _ => return Err(FindExampleShaderError::new(example.as_str()).into()),
        }
    } else {
        // Read fragment shader from file into String buffer
        match av.shaderpath {
            Some(ref shaderpath) => {
                let mut frag_src_str = String::new();

                File::open(&Path::new(&shaderpath))
                    .or_else(|err| return_load_shader_error(shaderpath, err))?
                    .read_to_string(&mut frag_src_str)
                    .or_else(|err| return_load_shader_error(shaderpath, err))?;

                frag_src_str
            }
            None => String::from(DEFAULT_FRAG_SRC_STR),
        }
    };

    let unsupported_uniforms: Vec<String> = UNSUPPORTED_UNIFORMS
        .iter()
        .map(|s| s.to_string())
        .filter(|uu| frag_src_str.find(uu).is_some())
        .collect();

    if unsupported_uniforms.is_empty() {
        Ok(format_shader_src(&frag_src_str))
    } else {
        Err(UnsupportedUniformError::new(unsupported_uniforms).into())
    }
}

pub fn load_vertex_shader() -> Vec<u8> {
    DEFAULT_VERT_SRC_BUF.to_vec()
}

pub fn load_texture<F, R>(
    id: &TextureId,
    texpath: &Option<String>,
    factory: &mut F,
) -> error::Result<gfx::handle::ShaderResourceView<R, [f32; 4]>>
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    use gfx::format::Rgba8;
    use gfx::texture::Mipmap;

    let default_buf = if texpath.is_some() {
        None
    } else {
        match *id {
            TextureId::ZERO => Some(DEFAULT_TEXTURE0_BUF),
            TextureId::ONE => Some(DEFAULT_TEXTURE1_BUF),
            TextureId::TWO => Some(DEFAULT_TEXTURE2_BUF),
            TextureId::THREE => Some(DEFAULT_TEXTURE3_BUF),
        }
    };
    let img = if let Some(default_buf) = default_buf {
        image::load_from_memory(default_buf)?.flipv().to_rgba()
    } else {
        image::open(&texpath.clone().unwrap())?.flipv().to_rgba()
    };

    let (w, h) = img.dimensions();
    let kind = gfx::texture::Kind::D2(w as u16, h as u16, gfx::texture::AaMode::Single);
    let (_, view) =
        factory.create_texture_immutable_u8::<Rgba8>(kind, Mipmap::Allocated, &[&img])?;
    Ok(view)
}
