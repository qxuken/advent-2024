use std::{
    collections::{HashSet, VecDeque},
    io,
};

use itertools::Itertools;
use rayon::prelude::*;

use tracing::{instrument, trace};

use crate::{
    error::{AppError, Result},
    solutions::utils::{Coord, SIDE_MOVES},
};

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        trailheads_ratting_sum(line_reader)?;
    } else {
        trailheads_score_sum(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn trailheads_score_sum(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    let raw_blocks = line_reader
        .map_ok(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as u8))
                .collect::<Vec<u8>>()
        })
        .collect::<io::Result<Vec<Vec<u8>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;
    trace!(raw_blocks = ?raw_blocks);
    let count = raw_blocks
        .par_iter()
        .enumerate()
        .flat_map(|(row_i, row)| {
            row.par_iter()
                .enumerate()
                .filter(|(_col_i, &n)| n == 0)
                .map(|(col_i, _n)| (row_i, col_i))
                .collect::<Vec<Coord>>()
        })
        .map(|start| {
            let mut moves = VecDeque::from([start]);
            let mut visited = HashSet::new();
            let mut ends = HashSet::new();
            while let Some(coord) = moves.pop_front() {
                let val = raw_blocks[coord.0][coord.1];
                trace!(move = ?coord, val = raw_blocks[coord.0][coord.1]);
                if val == 9 {
                    ends.insert(coord);
                    continue;
                }
                for new_coord in
                    SIDE_MOVES
                        .iter()
                        .filter_map(|d| d.new_coord(coord))
                        .filter(|new_coord| {
                            raw_blocks
                                .get(new_coord.0)
                                .and_then(|r| r.get(new_coord.1))
                                .is_some_and(|&v| v == val + 1)
                        })
                {
                    if visited.contains(&new_coord) {
                        continue;
                    }
                    moves.push_back(new_coord);
                    visited.insert(new_coord);
                }
            }
            ends.len()
        })
        .sum();

    Ok(count)
}

#[instrument(skip_all, ret)]
fn trailheads_ratting_sum(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    let raw_blocks = line_reader
        .map_ok(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as u8))
                .collect::<Vec<u8>>()
        })
        .collect::<io::Result<Vec<Vec<u8>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;
    trace!(raw_blocks = ?raw_blocks);
    let count = raw_blocks
        .par_iter()
        .enumerate()
        .flat_map(|(row_i, row)| {
            row.par_iter()
                .enumerate()
                .filter(|(_col_i, &n)| n == 0)
                .map(|(col_i, _n)| (row_i, col_i))
                .collect::<Vec<Coord>>()
        })
        .map(|start| {
            let mut moves = VecDeque::from([start]);
            let mut count = 0;
            while let Some(coord) = moves.pop_front() {
                let val = raw_blocks[coord.0][coord.1];
                trace!(move = ?coord, val = raw_blocks[coord.0][coord.1]);
                if val == 9 {
                    count += 1;
                    continue;
                }
                moves.extend(SIDE_MOVES.iter().filter_map(|d| d.new_coord(coord)).filter(
                    |new_coord| {
                        raw_blocks
                            .get(new_coord.0)
                            .and_then(|r| r.get(new_coord.1))
                            .is_some_and(|&v| v == val + 1)
                    },
                ));
            }
            count
        })
        .sum();

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"89010123
                      78121874
                      87430965
                      96549874
                      45678903
                      32019012
                      01329801
                      10456732"#;
        let res = trailheads_score_sum(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 36);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"89010123
                      78121874
                      87430965
                      96549874
                      45678903
                      32019012
                      01329801
                      10456732"#;
        let res = trailheads_ratting_sum(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 81);
    }
}
