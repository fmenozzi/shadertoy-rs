extern crate shadertoy_rs;

use shadertoy_rs::argvalues::ArgValues;
use shadertoy_rs::runner;

fn main() {
    let av = ArgValues::from_values(600.0, 400.0, "examples/elemental-ring/elemental-ring.frag");
    if let Err(e) = runner::run(&av) {
        println!("{}", e);
    }
}
