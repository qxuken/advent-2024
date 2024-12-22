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
        Command::Day9 => {
            debug!("Day9 solution requested");
            solutions::day9::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day10 => {
            debug!("Day10 solution requested");
            solutions::day10::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day11 => {
            debug!("Day11 solution requested");
            solutions::day11::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day12 => {
            debug!("Day12 solution requested");
            solutions::day12::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day13 => {
            debug!("Day13 solution requested");
            solutions::day13::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day14 => {
            debug!("Day14 solution requested");
            solutions::day14::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day15 => {
            debug!("Day15 solution requested");
            solutions::day15::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day16 => {
            debug!("Day16 solution requested");
            solutions::day16::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day17 => {
            debug!("Day17 solution requested");
            solutions::day17::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day18 => {
            debug!("Day18 solution requested");
            solutions::day18::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day19 => {
            debug!("Day19 solution requested");
            solutions::day19::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day20 => {
            debug!("Day20 solution requested");
            solutions::day20::solve(args.second_star, data_reader.lines())?;
        }
        Command::Day21 => {
            debug!("Day21 solution requested");
            solutions::day21::solve(args.second_star, data_reader.lines())?;
        }
    }

    Ok(())
}
