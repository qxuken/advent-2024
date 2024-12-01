#![feature(file_buffered)]
use std::{fs::File, io::BufRead};

use crate::cli::{Args, Command};
use clap::Parser;
use tracing::{debug, trace};

mod cli;
mod constants;
mod error;
mod instrument;
mod solutions;
mod utils;

fn main() -> error::Result<()> {
    utils::color_eyre::setup()?;

    let args = Args::parse();
    args.instrumentation.setup(&[constants::PACKAGE_NAME])?;
    trace!(args = ?args, "Arguments parsed, Instrumentation setup");

    let data_file_reader = match File::open_buffered(args.data_file) {
        Ok(file) => file,
        Err(err) => return Err(error::AppError::DataOpen(err.to_string()).into()),
    };
    trace!(reader = ?data_file_reader, "Data file reader initialized");

    match args.command {
        Command::Day1 => {
            debug!("Day1 solution requested");
            solutions::day1::solve(args.second_star, data_file_reader.lines())?;
        }
    }

    Ok(())
}
