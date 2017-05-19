use argvalues::ArgValues;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn load_shaders(av: &ArgValues) -> (Result<Vec<u8>, String>, Result<Vec<u8>, String>) {
    (load_vertex_shader(), load_fragment_shader(&av))
}

pub fn load_fragment_shader(av: &ArgValues) -> Result<Vec<u8>, String> {
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

            uniform float iGlobalTime;
            uniform vec3  iResolution;
            uniform vec4  iMouse;
            uniform int   iFrame;

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


