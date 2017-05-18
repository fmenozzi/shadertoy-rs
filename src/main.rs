extern crate shadertoy_rs;

use shadertoy_rs::{arg_values, runner};

fn main() {
    match arg_values::ArgValues::from_cli() {
        Ok(av) => {
            let arg_values::ArgValues{width, height, shaderpath} = av;
            if let Err(e) = runner::run(width, height, &shaderpath) {
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
