use argvalues::ArgValues;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use gfx;
use image;

use runner::TextureId;

// Default shaders
pub static DEFAULT_VERT_SRC_BUF: &'static [u8] = include_bytes!("../shaders/default.vert");
pub static DEFAULT_FRAG_SRC_STR: &'static str  = include_str!("../shaders/default.frag");

// Default textures
pub static DEFAULT_TEXTURE0_BUF: &'static [u8] = include_bytes!("../textures/01-brickwall.jpg");
pub static DEFAULT_TEXTURE1_BUF: &'static [u8] = include_bytes!("../textures/02-landscape.jpg");
pub static DEFAULT_TEXTURE2_BUF: &'static [u8] = include_bytes!("../textures/03-whitenoise.jpg");
pub static DEFAULT_TEXTURE3_BUF: &'static [u8] = include_bytes!("../textures/04-woodgrain.jpg");

// Example shaders
pub static EXAMPLE_SEASCAPE_STR: &'static str = include_str!("../examples/seascape.frag");
pub static EXAMPLE_ELEMENTAL_RING_STR: &'static str = include_str!("../examples/elemental-ring.frag");

pub fn load_shaders(av: &ArgValues) -> (Result<Vec<u8>, String>, Result<Vec<u8>, String>) {
    (load_vertex_shader(), load_fragment_shader(&av))
}

pub fn load_fragment_shader(av: &ArgValues) -> Result<Vec<u8>, String> {
    let frag_src_str = if let Some(ref example) = av.examplename {
        let example_str;
        match example.as_ref() {
            "seascape" => example_str = EXAMPLE_SEASCAPE_STR.to_string(),
            "elemental-ring" => example_str = EXAMPLE_ELEMENTAL_RING_STR.to_string(),
            _ => return Err(format!("No example named {}", example)),
        }
        example_str
    } else {
        // Read fragment shader from file into String buffer
        match av.shaderpath {
            Some(ref shaderpath) => {
                let mut frag_src_str = String::new();
                match File::open(&Path::new(&shaderpath)) {
                    Ok(mut file) => {
                        if let Err(e) = file.read_to_string(&mut frag_src_str) {
                            return Err(format!("Error reading from '{}': {}", shaderpath, e));
                        }
                    },
                    Err(e) => {
                        return Err(format!("Error opening file '{}': {}", shaderpath, e));
                    }
                }
                frag_src_str
            },
            None => {
                String::from(DEFAULT_FRAG_SRC_STR)
            }
        }
    };

    // Add prefix/suffix to fragment shader source
    let prefix = "
        #version 150 core

        uniform float     iGlobalTime;
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
    let suffix = "
        void main() {
            mainImage(fragColor, fragCoord);
        }
    ";
    let frag_src_str = format!("{}\n{}\n{}", prefix, frag_src_str, suffix);

    Ok(frag_src_str.into_bytes())
}

pub fn load_vertex_shader() -> Result<Vec<u8>, String> {
    Ok(DEFAULT_VERT_SRC_BUF.to_vec())
}

pub fn load_texture<F, R>(id: TextureId, texpath: &Option<String>, factory: &mut F) ->
        Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>
    where F: gfx::Factory<R>,
          R: gfx::Resources
{
    use gfx::format::Rgba8;

    let default_buf = match *texpath {
        Some(_) => {
            None
        },
        None => {
            match id {
                TextureId::ZERO  => Some(DEFAULT_TEXTURE0_BUF),
                TextureId::ONE   => Some(DEFAULT_TEXTURE1_BUF),
                TextureId::TWO   => Some(DEFAULT_TEXTURE2_BUF),
                TextureId::THREE => Some(DEFAULT_TEXTURE3_BUF),
            }
        }
    };

    let img = match default_buf {
        Some(default_buf) => {
            let img;
            match image::load_from_memory(default_buf) {
                Ok(res) => img = res.flipv().to_rgba(),
                Err(e)  => return Err(format!("Error opening default texture: {}", e)),
            }
            img
        },
        None => {
            let path = texpath.clone().unwrap();
            let img;
            match image::open(&path) {
                Ok(res) => img = res.flipv().to_rgba(),
                Err(e)  => return Err(format!("Error opening texture {}: {}", path, e)),
            }
            img
        }
    };

    let (w, h) = img.dimensions();

    let kind = gfx::texture::Kind::D2(w as u16, h as u16, gfx::texture::AaMode::Single);

    let view;
    match factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]) {
        Ok((_, v)) => view = v,
        Err(e)     => return Err(format!("Error creating texture: {}", e)),
    }

    Ok(view)
}
