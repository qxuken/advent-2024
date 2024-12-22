use std::io;

use itertools::Itertools;
// use rayon::prelude::*;

use tracing::{instrument, trace};

use crate::error::{AppError, Result};

#[derive(Debug)]
struct Machine {
    a_diff: (usize, usize),
    b_diff: (usize, usize),
    target: (usize, usize),
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        task_hard(line_reader)?;
    } else {
        task_simple(line_reader)?;
    }
    Ok(())
}

fn parse_line_into_usize_tuple(line: &str) -> Option<(usize, usize)> {
    line.split_once(',').and_then(|(left, right)| {
        left.chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .ok()
            .zip(
                right
                    .chars()
                    .filter(char::is_ascii_digit)
                    .collect::<String>()
                    .parse()
                    .ok(),
            )
    })
}

#[instrument(skip_all, ret)]
fn task_simple(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
    let machines = &line_reader
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            chunk
                .filter_ok(|s| !s.is_empty())
                .collect::<io::Result<Vec<String>>>()
        })
        .map_ok(|lines| {
            assert_eq!(lines.len(), 3);
            let a_diff = parse_line_into_usize_tuple(&lines[0]).unwrap();
            let b_diff = parse_line_into_usize_tuple(&lines[1]).unwrap();
            let target = parse_line_into_usize_tuple(&lines[2]).unwrap();
            Machine {
                a_diff,
                b_diff,
                target,
            }
        })
        .collect::<io::Result<Vec<Machine>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;
    trace!(machines = ?machines, count = machines.len());

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
