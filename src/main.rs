extern crate shadertoy_rs;

use shadertoy_rs::{arg_values, runner};

fn main() {
    let w: f32;
    let h: f32;
    let shaderpath: String;
    match arg_values::ArgValues::new() {
        Ok(av) => {
            w = av.width;
            h = av.height;
            shaderpath = av.shaderpath;
            if let Err(e) = runner::run(w, h, &shaderpath) {
                println!("{}", e);
                return;
            }
        },
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
}
