use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_2024::{
    cli::{Args, Command},
    constants, error, solutions, utils,
};
use clap::Parser;
use tracing::{debug, trace};

fn main() -> error::Result<()> {
    utils::color_eyre::setup()?;

    let args = Args::parse();
    args.instrumentation.setup(&[constants::PACKAGE_NAME])?;
    trace!(args = ?args, "Arguments parsed, Instrumentation setup");

    let data_file = match File::open(args.data_file) {
        Ok(file) => file,
        Err(err) => return Err(advent_2024::error::AppError::DataOpen(err.to_string()).into()),
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
        Command::Day6 => {
            debug!("Day6 solution requested");
            solutions::day6::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day7 => {
            debug!("Day7 solution requested");
            solutions::day7::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day8 => {
            debug!("Day8 solution requested");
            solutions::day8::solve(args.second_star, data_reader.lines())?;
        }
    }

    Ok(())
}
