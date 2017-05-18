extern crate shadertoy_rs;

use shadertoy_rs::argvalues::ArgValues;
use shadertoy_rs::runner;

fn main() {
    match ArgValues::from_cli() {
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
