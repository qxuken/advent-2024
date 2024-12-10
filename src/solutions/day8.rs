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

#[instrument(ret(level = Level::TRACE))]
fn calc_antinodes_coords(
    (f_row, f_col): &Coord,
    (s_row, s_col): &Coord,
    bounds: isize,
) -> Vec<Coord> {
    let mut res = vec![];
    if f_row == s_row && f_col == s_col {
        return res;
    }
    let v_row = *s_row as isize - *f_row as isize;
    let v_col = *s_col as isize - *f_col as isize;

    trace!(v = ?(v_row, v_col));
    let mut n_row = *s_row as isize + v_row;
    let mut n_col = *s_col as isize + v_col;
    while 0 <= n_row.min(n_col) && n_row.max(n_col) < bounds {
        trace!(n = ?(n_row, n_col));
        res.push((n_row as usize, n_col as usize));
        n_row += v_row;
        n_col += v_col;
    }
    res
}

#[instrument(ret(level = Level::TRACE))]
fn calc_antinodes_coord((f_row, f_col): &Coord, (s_row, s_col): &Coord) -> Option<Coord> {
    if f_row == s_row && f_col == s_col {
        return None;
    }

    let v_row = *s_row as isize - *f_row as isize;
    let v_col = *s_col as isize - *f_col as isize;

    let n_row = *s_row as isize + v_row;
    let n_col = *s_col as isize + v_col;

    trace!(v = ?(v_row, v_col));

    usize::try_from(n_row).ok().zip(usize::try_from(n_col).ok())
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        count_antinodes_rec(line_reader)?;
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
                            .or_insert(HashSet::from([(row_i, col_i)]));
                    });
                acc
            });

    trace!(char_map = ?char_map, stations = ?stations, "Parsed");

    let antinodes = stations
        .par_iter()
        .flat_map(|(_l, coords)| {
            coords
                .par_iter()
                .flat_map(|first_coord| {
                    coords
                        .par_iter()
                        .filter_map(|second_coord| calc_antinodes_coord(first_coord, second_coord))
                        .collect::<Vec<Coord>>()
                })
                .collect::<Vec<Coord>>()
        })
        .filter(|(row, col)| row.max(col) < &char_map.len())
        .collect::<Vec<Coord>>()
        .into_iter()
        .unique()
        .collect_vec();

    for (row, col) in antinodes.iter() {
        if char_map[*row][*col] == '.' {
            char_map[*row][*col] = '#';
        }
    }

    let mut map = String::with_capacity(char_map.len().pow(2) + char_map.len());
    for row in char_map.iter() {
        map.push_str(&row.iter().join(""));
        map.push('\n');
    }

    trace!(antinodes = ?antinodes, "Antinodes calculated");
    trace!("New map\n{map}");

    Ok(antinodes.len())
}

#[instrument(skip_all, ret)]
fn count_antinodes_rec(
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
                            .or_insert(HashSet::from([(row_i, col_i)]));
                    });
                acc
            });

    trace!(char_map = ?char_map, coords = ?coords, stations = ?stations, "Parsed");

    let bounds = char_map.len() as isize;
    let antinodes = stations
        .par_iter()
        .flat_map(|(_l, coords)| {
            coords
                .par_iter()
                .flat_map(|first_coord| {
                    coords
                        .par_iter()
                        .flat_map(|second_coord| {
                            calc_antinodes_coords(first_coord, second_coord, bounds)
                        })
                        .collect::<Vec<Coord>>()
                })
                .collect::<Vec<Coord>>()
        })
        .filter(|coord| !coords.contains(coord))
        .collect::<Vec<Coord>>()
        .into_iter()
        .unique()
        .collect_vec();

    for (row, col) in antinodes.iter() {
        if char_map[*row][*col] == '.' {
            char_map[*row][*col] = '#';
        }
    }

    let mut map = String::with_capacity(char_map.len().pow(2) + char_map.len());
    for row in char_map.iter() {
        map.push_str(&row.iter().join(""));
        map.push('\n');
    }

    trace!(antinodes = ?antinodes, "Antinodes calculated");
    trace!("New map\n{map}");

    Ok(antinodes.len() + coords.len())
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

        let res = count_antinodes_rec(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 34);
    }
}
