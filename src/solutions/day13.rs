use std::io;

use itertools::Itertools;
use rayon::prelude::*;

use tracing::{instrument, trace, Level};

use crate::error::{AppError, Result};

#[derive(Debug)]
struct Machine {
    /// 3 tokens
    a_diff: (usize, usize),
    /// 1 token
    b_diff: (usize, usize),
    target: (usize, usize),
}

impl Machine {
    #[instrument(ret(level = Level::DEBUG))]
    fn min_tokens(&self) -> Option<usize> {
        // px = i*ax + j*bx
        // py = i*ay + j*by
        //
        // A = ⌈ax bx⌉  X = ⌈i⌉  AX = C = ⌈px⌉
        //     ⌊ay by⌋      ⌊j⌋           ⌊py⌋
        //
        // D = |A| = ax*by - ay*bx
        // Di = px*by - py*bx
        // Dj = py*ax - px*ay
        //
        // i = Di / D
        // j = Dj / D
        // answer = 3*i + j
        // c: https://github.com/ndunnett/aoc/blob/d669668c310971c675e650528f0d747571a2de23/rust/2024/src/bin/day13.rs#L41
        let d = (self.a_diff.0 * self.b_diff.1) as i64 - (self.a_diff.1 * self.b_diff.0) as i64;
        let di = (self.target.0 * self.b_diff.1) as i64 - (self.target.1 * self.b_diff.0) as i64;
        let dj = (self.target.1 * self.a_diff.0) as i64 - (self.target.0 * self.a_diff.1) as i64;

        if di % d == 0 && dj % d == 0 {
            Some((3 * di / d + dj / d) as usize)
        } else {
            None
        }
    }

    #[instrument]
    fn fix_target_coord(mut self, val: usize) -> Self {
        self.target.0 += val;
        self.target.1 += val;
        self
    }
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

    let sum = machines
        .into_par_iter()
        .filter_map(Machine::min_tokens)
        .sum();

    Ok(sum)
}

#[instrument(skip_all, ret)]
fn task_hard(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
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
        .map_ok(|m| m.fix_target_coord(10000000000000))
        .collect::<io::Result<Vec<Machine>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;
    trace!(machines = ?machines, count = machines.len());

    let sum = machines
        .into_par_iter()
        .filter_map(Machine::min_tokens)
        .sum();

    Ok(sum)
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
    fn validate_failed_case_1() {
        let data = r#"Button A: X+21, Y+30
                      Button B: X+87, Y+34
                      Prize: X=4413, Y=1790"#;
        let res = task_simple(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 59);
    }

    #[test]
    fn validate_second_star_example() {
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
        let res = task_hard(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 875318608908);
    }
}
