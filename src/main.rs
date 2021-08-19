#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate failure;
extern crate notify;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate failure_derive;
extern crate old_school_gfx_glutin_ext;

mod argvalues;
mod download;
mod error;
mod loader;
mod runner;

use argvalues::ArgValues;

fn main() {
    env_logger::init().expect("Unable to initialize logger");

    if let Err(e) = ArgValues::from_cli().and_then(|av| runner::run(av)) {
        error!("{}", e);
    }
}
