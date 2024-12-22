use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

use itertools::Itertools;
// use rayon::prelude::*;

use tracing::{instrument, trace, Level};

use crate::{
    error::{AppError, Result},
    solutions::utils::{Direction, SIDE_MOVES},
};

use super::utils::Coord;

type Sides = (Direction, (usize, usize));

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        task_hard(line_reader)?;
    } else {
        task_simple(line_reader)?;
    }
    Ok(())
}

#[instrument(skip(map), ret(level = Level::TRACE))]
fn region_data(map: &[Vec<char>], ch: char, coord: Coord) -> (HashSet<Sides>, HashSet<Coord>) {
    let mut moves = VecDeque::from([coord]);
    let mut loc_visited = HashSet::from([coord]);
    let mut sides = HashSet::new();
    while let Some(coord) = moves.pop_front() {
        for (dir, next_coord) in SIDE_MOVES.iter().map(|d| (d, d.new_coord(coord))) {
            match next_coord {
                Some(c) if c.0.max(c.1) >= map.len() => {
                    sides.insert((*dir, coord));
                }
                Some(c) if loc_visited.contains(&c) => {
                    continue;
                }
                Some(c) if map[c.0][c.1] != ch => {
                    sides.insert((*dir, coord));
                }
                Some(c) => {
                    loc_visited.insert(c);
                    moves.push_back(c);
                }
                None => {
                    sides.insert((*dir, coord));
                }
            }
        }
    }
    trace!(
        sides_len = sides.len(),
        visited_len = loc_visited.len(),
        mult = sides.len() * loc_visited.len(),
    );
    (sides, loc_visited)
}

#[instrument(ret(level = Level::TRACE))]
fn count_sides(sides: &HashSet<Sides>) -> usize {
    let mut side_combined: HashMap<(Direction, usize), Vec<usize>> = HashMap::new();
    for &(dir, (row, col)) in sides.iter() {
        match dir {
            Direction::Top | Direction::Bottom => {
                side_combined
                    .entry((dir, row))
                    .and_modify(|cols| (*cols).push(col))
                    .or_insert(vec![col]);
            }
            Direction::Right | Direction::Left => {
                side_combined
                    .entry((dir, col))
                    .and_modify(|rows| (*rows).push(row))
                    .or_insert(vec![row]);
            }
            _ => {
                unreachable!("Data corruption, got wrong side")
            }
        }
    }
    trace!(side_combined = ?side_combined);
    side_combined
        .into_values()
        .map(|mut side| {
            side.sort();
            side.into_iter()
                .tuple_windows()
                .fold(1, |acc, (a, b)| acc + ((a + 1 != b) as usize))
        })
        .sum()
}

#[instrument(skip_all, ret)]
fn task_simple(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
    let raw = line_reader
        .map_ok(|s| s.chars().collect::<Vec<char>>())
        .collect::<io::Result<Vec<Vec<char>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;
    trace!(raw_input = ?raw);

    let mut visited = HashSet::new();
    let mut sum = 0;
    for (row_i, row) in raw.iter().enumerate() {
        for (col_i, ch) in row.iter().enumerate() {
            let coord = (row_i, col_i);
            if visited.contains(&coord) {
                continue;
            }
            let (sides, visits) = region_data(&raw, *ch, coord);
            sum += sides.len() * visits.len();
            visited.extend(visits);
        }
    }

    Ok(sum)
}

#[instrument(skip_all, ret)]
fn task_hard(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize, AppError> {
    let raw = line_reader
        .map_ok(|s| s.chars().collect::<Vec<char>>())
        .collect::<io::Result<Vec<Vec<char>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;
    trace!(raw_input = ?raw);

    let mut visited = HashSet::new();
    let mut sum = 0;
    for (row_i, row) in raw.iter().enumerate() {
        for (col_i, ch) in row.iter().enumerate() {
            let coord = (row_i, col_i);
            if visited.contains(&coord) {
                continue;
            }
            let (sides, visits) = region_data(&raw, *ch, coord);
            sum += count_sides(&sides) * visits.len();
            visited.extend(visits);
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"RRRRIICCFF
                      RRRRIICCCF
                      VVRRRCCFFF
                      VVRCCCJFFF
                      VVVVCJJCFE
                      VVIVCCJJEE
                      VVIIICJJEE
                      MIIIIIJJEE
                      MIIISIJEEE
                      MMMISSJEEE"#;
        let res = task_simple(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1930);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"RRRRIICCFF
                      RRRRIICCCF
                      VVRRRCCFFF
                      VVRCCCJFFF
                      VVVVCJJCFE
                      VVIVCCJJEE
                      VVIIICJJEE
                      MIIIIIJJEE
                      MIIISIJEEE
                      MMMISSJEEE"#;
        let res = task_hard(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1206);
    }
}
