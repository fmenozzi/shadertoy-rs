#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate clap;
#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate serde_json;
extern crate reqwest;

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
