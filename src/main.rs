#[macro_use] extern crate clap;
#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate url;
extern crate serde_json;

mod argvalues;
mod runner;
mod loader;
mod error;
mod download;

use argvalues::ArgValues;

fn main() {
    env_logger::init().expect("Unable to initialize logger");

    if let Err(e) = ArgValues::from_cli().and_then(|av| runner::run(&av)) {
        error!("{}", e);
    }
}
