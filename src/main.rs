#![forbid(unsafe_code)]
#![allow(dead_code)]
#![deny(unused_imports)]

//! Glimbot is a general admin and anti-spam bot for Discord, written in Rust.
//! The primary design goal is to create a robust Discord bot with high performance to
//! manage large servers in the spirit of SweetieBot.

#[macro_use] extern crate log;
#[macro_use] extern crate rusqlite;

use std::env;
use std::path::Path;
use crate::data::{data_folder, AUTHORS, VERSION};
use clap::{App, AppSettings};
use failure::Fallible;

pub mod data;
pub mod db;
pub mod util;

#[cfg(feature = "development")]
pub mod dev;

pub mod args;

fn main() -> Fallible<()> {
    better_panic::install();
    let _ = dotenv::dotenv();
    env_logger::init();
    let _token = std::env::var("GLIMBOT_TOKEN")?;
    let _owner = std::env::var("GLIMBOT_OWNER").unwrap_or_default().parse::<u64>()?;

    // Create our working directory
    let data_dir = data_folder();
    ensure_data_folder(&data_dir);

    let mut subcommands = vec![];

    #[cfg(feature="development")]
    subcommands.push(dev::command_parser());

    subcommands.push(db::args::command_parser());

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(AUTHORS)
        .version(VERSION)
        .subcommands(subcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    #[cfg(feature="development")]
    dev::handle_matches(&matches)?;

    db::args::handle_matches(&matches)?;

    Ok(())
}

fn ensure_data_folder(p: impl AsRef<Path>) {
    std::fs::create_dir_all(p).expect("Couldn't create the path to default directory.");
}