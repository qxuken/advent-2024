use std::{io, mem};

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
    fn min_tokens(&self, max_presses: usize) -> Option<usize> {
        let mut cheap = self.b_diff;
        let mut costy = self.a_diff;
        let reversed = (self.a_diff.0 + self.a_diff.1) / (self.b_diff.0 + self.b_diff.1) >= 3;
        if reversed {
            mem::swap(&mut cheap, &mut costy);
        }
        let mut presses = (
            (self.target.0 / cheap.0)
                .min(self.target.1 / cheap.1)
                .min(max_presses),
            0,
        );
        let x_coord = |presses: &(usize, usize)| presses.0 * cheap.0 + presses.1 * costy.0;
        let y_coord = |presses: &(usize, usize)| presses.0 * cheap.1 + presses.1 * costy.1;
        let mut curr_target = (x_coord(&presses), y_coord(&presses));
        trace!(
            reversed,
            ?presses,
            ?curr_target,
            x_coord = x_coord(&presses),
            y_coord = y_coord(&presses)
        );
        presses.1 = ((self.target.0 - curr_target.0) / costy.0)
            .min((self.target.1 - curr_target.1) / costy.1)
            .min(max_presses);
        curr_target.0 += costy.0 * presses.1;
        curr_target.1 += costy.1 * presses.1;
        trace!(
            reversed,
            ?presses,
            ?curr_target,
            x_coord = x_coord(&presses),
            y_coord = y_coord(&presses)
        );
        while presses.0 > 0 && curr_target != self.target {
            // trace!(
            //     ?presses,
            //     ?curr_target,
            //     sub_x = cheap.0 + costy.0 * presses.1,
            //     sub_y = cheap.1 + costy.1 * presses.1,
            // );
            presses.0 -= 1;
            curr_target.0 -= cheap.0 + costy.0 * presses.1;
            curr_target.1 -= cheap.1 + costy.1 * presses.1;

            presses.1 = ((self.target.0 - curr_target.0) / costy.0)
                .min((self.target.1 - curr_target.1) / costy.1)
                .min(max_presses);
            curr_target.0 += costy.0 * presses.1;
            curr_target.1 += costy.1 * presses.1;
        }
        trace!(
            reversed,
            ?presses,
            ?curr_target,
            x_coord = x_coord(&presses),
            y_coord = y_coord(&presses)
        );
        if curr_target == self.target {
            if reversed {
                Some(presses.0 * 3 + presses.1)
            } else {
                Some(presses.0 + presses.1 * 3)
            }
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

    let mut sum = 0;
    for machine in machines {
        sum += machine.min_tokens(100).unwrap_or_default();
    }

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
        .filter_map(|m| m.min_tokens(usize::MAX))
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
        assert_eq!(res.unwrap(), 0);
    }
}
