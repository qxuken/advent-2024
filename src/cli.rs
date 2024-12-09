use crate::instrument::instrumentation::Instrumentation;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Program to solve 2024 advent of code
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    pub instrumentation: Instrumentation,

    /// Sets a data file
    #[arg(value_name = "DATA_FILE")]
    pub data_file: PathBuf,

    /// Should solve second star problem
    #[arg(long, short, global = true)]
    pub second_star: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Day 1 solution [https://adventofcode.com/2024/day/1]
    Day1,
    /// Day 2 solution [https://adventofcode.com/2024/day/2]
    Day2,
    /// Day 3 solution [https://adventofcode.com/2024/day/3]
    Day3,
    /// Day 4 solution [https://adventofcode.com/2024/day/4]
    Day4,
    /// Day 5 solution [https://adventofcode.com/2024/day/5]
    Day5,
    /// Day 6 solution [https://adventofcode.com/2024/day/6]
    Day6,
    /// Day 7 solution [https://adventofcode.com/2024/day/7]
    Day7,
    /// Day 8 solution [https://adventofcode.com/2024/day/8]
    Day8,
}
