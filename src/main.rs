use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

    let data_file = match File::open(args.data_file) {
        Ok(file) => file,
        Err(err) => return Err(error::AppError::DataOpen(err.to_string()).into()),
    };
    let data_reader = BufReader::new(&data_file);
    trace!(file = ?data_file, reader = ?data_reader, "Data file reader initialized");

    match args.command {
        Command::Day1 => {
            debug!("Day1 solution requested");
            solutions::day1::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day2 => {
            debug!("Day2 solution requested");
            solutions::day2::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day3 => {
            debug!("Day3 solution requested");
            solutions::day3::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day4 => {
            debug!("Day4 solution requested");
            solutions::day4::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day5 => {
            debug!("Day5 solution requested");
            solutions::day5::solve(args.second_star, data_reader.lines())?;
        }
    }

    Ok(())
}
