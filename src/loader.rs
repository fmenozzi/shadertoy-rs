use argvalues::ArgValues;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use gfx;
use image;

pub fn load_shaders(av: &ArgValues) -> (Result<Vec<u8>, String>, Result<Vec<u8>, String>) {
    (load_vertex_shader(), load_fragment_shader(&av))
}

pub fn load_fragment_shader(av: &ArgValues) -> Result<Vec<u8>, String> {
    info!("Loading fragment shader {}", av.shaderpath);

    // Read fragment shader from file into String buffer
    let mut frag_src_str = String::new();
    match File::open(&Path::new(&av.shaderpath)) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_string(&mut frag_src_str) {
                return Err(format!("Error reading from '{}': {}", av.shaderpath, e));
            }
        },
        Err(e) => {
            return Err(format!("Error opening file '{}': {}", av.shaderpath, e));
        }
    }

    // Add prefix/suffix to shader source if appropriate
    let (prefix, suffix) = if av.not_from_shadertoy {
        ("", "")
    } else {
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

        (prefix, suffix)
    };
    let frag_src_str = format!("{}\n{}\n{}", prefix, frag_src_str, suffix);

    Ok(frag_src_str.into_bytes())
}

pub fn load_vertex_shader() -> Result<Vec<u8>, String> {
    let vert_src_buf = include_bytes!("../shaders/default.vert");

    Ok(vert_src_buf.to_vec())
}

pub fn load_texture<F, R>(texpath: &str, factory: &mut F) ->
        Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>
    where F: gfx::Factory<R>,
          R: gfx::Resources
{
    info!("Loading texture from {}", texpath);

    use gfx::format::Rgba8;

    let img;
    match image::open(texpath) {
        Ok(res) => img = res.flipv().to_rgba(),
        Err(e)  => return Err(format!("Error opening texture {}: {}", texpath, e)),
    }

    let (w, h) = img.dimensions();

    let kind = gfx::texture::Kind::D2(w as u16, h as u16, gfx::texture::AaMode::Single);

    let view;
    match factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]) {
        Ok((_, v)) => view = v,
        Err(e)     => return Err(format!("Error creating texture: {}", e)),
    }

    Ok(view)
}
