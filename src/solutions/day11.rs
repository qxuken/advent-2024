use std::io;

use cached::proc_macro::cached;
use itertools::Itertools;

use tracing::{instrument, trace, Level};

use crate::error::{AppError, Result};

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        task_hard(line_reader)?;
    } else {
        task_simple(line_reader)?;
    }
    Ok(())
}

#[instrument(ret(level = Level::TRACE))]
#[cached]
fn stone_count_after_steps(stone: usize, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }
    if stone == 0 {
        return stone_count_after_steps(1, steps - 1);
    }
    let digits = stone.to_string();
    if digits.len() % 2 == 0 {
        let (first, second) = digits.split_at(digits.len() / 2);
        return stone_count_after_steps(first.parse().unwrap(), steps - 1)
            + stone_count_after_steps(second.parse().unwrap(), steps - 1);
    }
    stone_count_after_steps(stone * 2024, steps - 1)
}

#[instrument(ret(level = Level::TRACE))]
fn stones_count_after_steps(stones: Vec<usize>, steps: usize) -> usize {
    stones
        .into_iter()
        .map(|stone| stone_count_after_steps(stone, steps))
        .sum()
}

#[instrument(skip_all, ret)]
fn task_simple(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
    let stones: Vec<usize> = line_reader
        .map_ok(|s| {
            s.split(char::is_whitespace)
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .flatten()
        .collect();
    trace!(input = ?stones);
    let stones = stones_count_after_steps(stones, 25);

    Ok(stones)
}

#[instrument(skip_all, ret)]
fn task_hard(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
    let stones: Vec<usize> = line_reader
        .map_ok(|s| {
            s.split(char::is_whitespace)
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .flatten()
        .collect();
    trace!(input = ?stones);
    let stones = stones_count_after_steps(stones, 75);

    Ok(stones)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"125 17"#;
        let res = task_simple(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 55312);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"125 17"#;
        let res = task_hard(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 65601038650482);
    }
}
