use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io,
};

use tracing::{instrument, trace, Level};

use crate::{
    error::{AppError, Result},
    solutions::utils::Coord,
};

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        try_combine_numbers_with_concat(line_reader)?;
    } else {
        count_antinodes(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn count_antinodes(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    let lines = line_reader
        .map_ok(|s| s.chars().collect())
        .collect::<io::Result<Vec<Vec<char>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;

    let stations = lines.iter().enumerate().fold(
        HashMap::<char, HashSet<Coord>>::new(),
        |mut acc, (row_i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &ch)| ch != '.')
                .for_each(|(col_i, ch)| {
                    acc.entry(*ch)
                        .and_modify(|s| {
                            s.insert((row_i, col_i));
                        })
                        .or_insert(HashSet::from([(col_i, row_i)]));
                });
            acc
        },
    );
    trace!(stations = ?stations, "Parsed");

    Ok(0)
}

#[instrument(skip_all, ret)]
fn try_combine_numbers_with_concat(
    _line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"............
                            ........0...
                            .....0......
                            .......0....
                            ....0.......
                            ......A.....
                            ............
                            ............
                            ........A...
                            .........A..
                            ............
                            ............"#;
        let res = count_antinodes(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 14);
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
