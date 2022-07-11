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
extern crate anyhow;
extern crate env_logger;
extern crate notify;
extern crate old_school_gfx_glutin_ext;
extern crate reqwest;
extern crate serde_json;

mod argvalues;
mod download;
mod error;
mod loader;
mod runner;

use argvalues::ArgValues;

fn main() {
    env_logger::init().expect("Unable to initialize logger");

    if let Err(e) = ArgValues::from_cli().and_then(runner::run) {
        error!("{}", e);
    }
}
