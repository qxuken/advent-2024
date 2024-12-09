use itertools::{repeat_n, Itertools};
use rayon::prelude::*;
use std::io;

use tracing::{instrument, Level};

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Sum,
    Multiply,
    Concat,
}

impl Operator {
    fn calc(&self, left: usize, right: usize) -> usize {
        match self {
            Self::Sum => left + right,
            Self::Multiply => left * right,
            Self::Concat => left * 10usize.pow((right as f64).log10().ceil() as u32) + right,
        }
    }
}

struct PermutationsCalculator;

impl PermutationsCalculator {
    #[instrument(ret(level = Level::TRACE))]
    fn generate_permutations(operations: &[Operator], n: usize) -> Vec<Vec<Operator>> {
        repeat_n(operations.iter().copied(), n)
            .multi_cartesian_product()
            .collect()
    }

    #[instrument(ret(level = Level::DEBUG))]
    fn can_be_solved(target: &usize, nums: &[usize], operations: &[Operator]) -> bool {
        let perm = PermutationsCalculator::generate_permutations(operations, nums.len() - 1);
        perm.into_par_iter().any(|p| {
            p.into_iter()
                .enumerate()
                .fold(nums[0], |acc, (i, o)| o.calc(acc, nums[i + 1]))
                == *target
        })
    }
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        try_combine_numbers_with_concat(line_reader)?;
    } else {
        try_combine_numbers(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn try_combine_numbers(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    let Some(lines) = line_reader
        .filter_map(Result::ok)
        .filter_map(|s| {
            s.split_once(':')
                .map(|(t, nums)| (t.to_owned(), nums.trim().to_owned()))
        })
        .map(|(t, nums)| {
            t.parse::<usize>().ok().zip(
                nums.split(char::is_whitespace)
                    .map(|n| n.parse().ok())
                    .collect::<Option<Vec<usize>>>(),
            )
        })
        .collect::<Option<Vec<(usize, Vec<usize>)>>>()
    else {
        return Err(AppError::DataParse("data corrupted".to_string()));
    };
    let operations = vec![Operator::Sum, Operator::Multiply];
    Ok(lines
        .into_par_iter()
        .filter(|(target, nums)| PermutationsCalculator::can_be_solved(target, nums, &operations))
        .map(|(t, _)| t)
        .sum())
}

#[instrument(skip_all, ret)]
fn try_combine_numbers_with_concat(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    let Some(lines) = line_reader
        .filter_map(Result::ok)
        .filter_map(|s| {
            s.split_once(':')
                .map(|(t, nums)| (t.to_owned(), nums.trim().to_owned()))
        })
        .map(|(t, nums)| {
            t.parse::<usize>().ok().zip(
                nums.split(char::is_whitespace)
                    .map(|n| n.parse().ok())
                    .collect::<Option<Vec<usize>>>(),
            )
        })
        .collect::<Option<Vec<(usize, Vec<usize>)>>>()
    else {
        return Err(AppError::DataParse("data corrupted".to_string()));
    };
    let operations = vec![Operator::Sum, Operator::Multiply, Operator::Concat];
    Ok(lines
        .into_par_iter()
        .filter(|(target, nums)| PermutationsCalculator::can_be_solved(target, nums, &operations))
        .map(|(t, _)| t)
        .sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"190: 10 19
                      3267: 81 40 27
                      83: 17 5
                      156: 15 6
                      7290: 6 8 6 15
                      161011: 16 10 13
                      192: 17 8 14
                      21037: 9 7 18 13
                      292: 11 6 16 20"#;
        let res = try_combine_numbers(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 3749);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"190: 10 19
                      3267: 81 40 27
                      83: 17 5
                      156: 15 6
                      7290: 6 8 6 15
                      161011: 16 10 13
                      192: 17 8 14
                      21037: 9 7 18 13
                      292: 11 6 16 20"#;
        let res =
            try_combine_numbers_with_concat(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 11387);
    }
}
