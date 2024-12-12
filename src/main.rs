#![cfg_attr(not(feature = "std"), no_std)]

use axvmconfig::*;

#[cfg(feature = "std")]
mod parser;

fn main() {
    // configure logger and set log level
    #[cfg(feature = "std")]
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    #[cfg(feature = "std")]
    parser::run();
}
