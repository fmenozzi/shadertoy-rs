#[macro_use] extern crate clap;
#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;

mod argvalues;
mod runner;
mod loader;

use argvalues::ArgValues;

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
