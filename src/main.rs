extern crate shadertoy_rs;

use shadertoy_rs::{arg_values, runner};

fn main() {
    match arg_values::ArgValues::from_cli() {
        Ok(av) => {
            if let Err(e) = runner::run(&av) {
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
