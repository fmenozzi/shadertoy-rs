extern crate shadertoy_rs;

use shadertoy_rs::runner;

fn main() {
    let (w, h, shaderpath) = (600.0, 400.0, "examples/elemental-ring/elemental-ring.frag");
    if let Err(e) = runner::run(w, h, &shaderpath) {
        println!("{}", e);
    }
}
