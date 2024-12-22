use std::io;

use itertools::Itertools;
// use rayon::prelude::*;

use tracing::{instrument, trace};

use crate::error::{AppError, Result};

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        task_hard(line_reader)?;
    } else {
        task_simple(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn task_simple(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
    let raw = line_reader
        .map_ok(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as u8))
                .collect::<Vec<u8>>()
        })
        .collect::<io::Result<Vec<Vec<u8>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;
    trace!(raw_input = ?raw);

    Ok(0)
}

#[instrument(skip_all, ret)]
fn task_hard(_line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"Button A: X+94, Y+34
                      Button B: X+22, Y+67
                      Prize: X=8400, Y=5400

                      Button A: X+26, Y+66
                      Button B: X+67, Y+21
                      Prize: X=12748, Y=12176

                      Button A: X+17, Y+86
                      Button B: X+84, Y+37
                      Prize: X=7870, Y=6450

                      Button A: X+69, Y+23
                      Button B: X+27, Y+71
                      Prize: X=18641, Y=10279"#;
        let res = task_simple(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 480);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"multiline
                      datadatad"#;
        let res = task_hard(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 0);
    }
}
