#[macro_use] extern crate clap;
#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[macro_use] extern crate log;
extern crate env_logger;

mod argvalues;
mod runner;
mod loader;

use argvalues::ArgValues;

fn main() {
    env_logger::init().expect("Unable to initialize logger");

    match ArgValues::from_cli() {
        Ok(av) => {
            if let Err(e) = runner::run(&av) {
                error!("{}", e);
                return;
            }
        },
        Err(e) => {
            error!("{}", e);
            return;
        }
    }
}
