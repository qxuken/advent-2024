use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io,
};

use tracing::{instrument, trace};

use crate::{
    error::{AppError, Result},
    solutions::utils::Coord,
};

fn calc_antinodes_coords((f_row, f_col): &Coord, (s_row, s_col): &Coord) -> Vec<Coord> {
    unimplemented!()
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        count_antinodes_hard(line_reader)?;
    } else {
        count_antinodes(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn count_antinodes(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    let mut char_map = line_reader
        .map_ok(|s| s.chars().collect())
        .collect::<io::Result<Vec<Vec<char>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?;

    let coords: HashSet<Coord> =
        char_map
            .iter()
            .enumerate()
            .fold(HashSet::new(), |mut acc, (row_i, row)| {
                row.iter()
                    .copied()
                    .enumerate()
                    .filter(|(_, ch)| ch != &'.')
                    .for_each(|(col_i, _ch)| {
                        acc.insert((row_i, col_i));
                    });
                acc
            });

    let stations: HashMap<char, HashSet<Coord>> =
        char_map
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (row_i, row)| {
                row.iter()
                    .copied()
                    .enumerate()
                    .filter(|(_, ch)| ch != &'.')
                    .for_each(|(col_i, ch)| {
                        acc.entry(ch)
                            .and_modify(|s| {
                                s.insert((row_i, col_i));
                            })
                            .or_insert(HashSet::from([(col_i, row_i)]));
                    });
                acc
            });

    trace!(char_map = ?char_map, coords = ?coords, stations = ?stations, "Parsed");

    let mut antinodes = stations
        .par_iter()
        .flat_map(|(_l, coords)| {
            coords
                .par_iter()
                .flat_map(|first_coord| {
                    coords
                        .par_iter()
                        .flat_map(|second_coord| calc_antinodes_coords(first_coord, second_coord))
                        .collect::<Vec<Coord>>()
                })
                .collect::<Vec<Coord>>()
        })
        .collect::<Vec<Coord>>()
        .into_iter()
        .unique()
        .filter(|coord| !coords.contains(coord))
        .filter(|(row, col)| row.max(col) < &char_map.len())
        .collect_vec();
    antinodes.sort();

    for (row, col) in antinodes.iter() {
        char_map[*row][*col] = '#';
    }

    let mut map = "\n".to_string();
    for row in char_map.iter() {
        map.push_str(&row.iter().join(""));
        map.push('\n');
    }

    trace!(antinodes = ?antinodes, "Antinodes calculated");
    trace!("New map {map}");

    Ok(antinodes.len())
}

#[instrument(skip_all, ret)]
fn count_antinodes_hard(
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
        let res = count_antinodes_hard(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 11387);
    }
}
