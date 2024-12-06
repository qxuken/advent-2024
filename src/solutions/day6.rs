use rayon::prelude::*;
use std::{
    fmt::{Display, Write},
    io,
};

use tracing::{debug, instrument, trace, Level};

use crate::error::{AppError, Result};

use super::utils::{Coord, Direction};

#[derive(Debug, Clone, Copy)]
struct Guard {
    direction: Direction,
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::Top => f.write_char('^'),
            Direction::Left => f.write_char('<'),
            Direction::Right => f.write_char('>'),
            Direction::Bottom => f.write_char('v'),
            _ => Err(std::fmt::Error),
        }
    }
}

impl TryFrom<&char> for Guard {
    type Error = AppError;
    #[instrument(ret(level = Level::TRACE))]
    fn try_from(ch: &char) -> Result<Self, Self::Error> {
        match ch {
            '^' => Ok(Self {
                direction: Direction::Top,
            }),
            '>' => Ok(Self {
                direction: Direction::Right,
            }),
            '<' => Ok(Self {
                direction: Direction::Left,
            }),
            'v' => Ok(Self {
                direction: Direction::Bottom,
            }),
            _ => Err(AppError::DataParse("Incorrect character".to_string())),
        }
    }
}

impl From<Guard> for Direction {
    fn from(value: Guard) -> Self {
        value.direction
    }
}

impl Guard {
    fn should_turn(it: &MapItem) -> bool {
        matches!(it, MapItem::Wall | MapItem::NewWall)
    }

    fn try_new_direction(dir: Direction) -> Result<Direction, AppError> {
        match dir {
            Direction::Top => Ok(Direction::Right),
            Direction::Right => Ok(Direction::Bottom),
            Direction::Bottom => Ok(Direction::Left),
            Direction::Left => Ok(Direction::Top),
            d => Err(AppError::LogicalError(format!(
                "Undefined direction: {d:?}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MapItem {
    Wall,
    NewWall,
    Guard(Guard),
    Floor,
    Visited,
}

impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => f.write_char('#'),
            Self::NewWall => f.write_char('0'),
            Self::Guard(g) => f.write_str(g.to_string().as_str()),
            Self::Floor => f.write_char('.'),
            Self::Visited => f.write_char('X'),
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    map: Vec<Vec<MapItem>>,
    guard: (Coord, Direction),
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for it in row.iter() {
                write!(f, "{it}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<Vec<Vec<char>>> for Scanner {
    type Error = AppError;

    fn try_from(value: Vec<Vec<char>>) -> Result<Self, Self::Error> {
        let mut guard: Option<((usize, usize), Direction)> = None;
        let map = value
            .iter()
            .enumerate()
            .map(|(row_i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_i, ch)| match ch {
                        '^' | '>' | '<' | 'v' => {
                            let g: Guard = ch.try_into()?;
                            let dir: Direction = g.into();
                            guard = Some(((row_i, col_i), dir));
                            Ok(MapItem::Guard(g))
                        }
                        '.' => Ok(MapItem::Floor),
                        '#' => Ok(MapItem::Wall),
                        _ => Err(AppError::DataParse(format!("Unknown character: {ch}"))),
                    })
                    .collect::<Result<Vec<MapItem>, AppError>>()
            })
            .collect::<Result<Vec<Vec<MapItem>>, AppError>>()?;
        let Some(guard) = guard else {
            return Err(AppError::DataParse("Guard not found".to_string()));
        };
        Ok(Self { map, guard })
    }
}

impl Scanner {
    fn try_walk_step(&self) -> Option<(Coord, Direction)> {
        let (cur_coord, cur_dir) = self.guard;
        let new_coord = cur_dir.new_coord(cur_coord)?;
        if new_coord.0 >= self.map.len() || new_coord.1 >= self.map.len() {
            return None;
        }
        let it = self
            .map
            .get(new_coord.0)
            .and_then(|row| row.get(new_coord.1))?;
        if Guard::should_turn(it) {
            Some((cur_coord, Guard::try_new_direction(cur_dir).ok()?))
        } else {
            Some((new_coord, cur_dir))
        }
    }

    fn try_walk_steps(&mut self, n: usize) -> Option<()> {
        for _ in 0..n {
            self.guard = self.try_walk_step()?;
        }
        Some(())
    }

    fn plot_guard_route(&self) -> Self {
        let mut s = self.clone();
        while let Some((coord, dir)) = s.try_walk_step() {
            s.guard = (coord, dir);
            s.map[coord.0][coord.1] = MapItem::Visited;
        }
        s
    }

    #[instrument(skip(self), ret(level = Level::TRACE))]
    fn count_visited(&self) -> usize {
        self.map
            .par_iter()
            .map(|row| {
                row.par_iter()
                    .filter(|it| matches!(it, MapItem::Visited | MapItem::Guard(_)))
                    .count()
            })
            .sum()
    }

    #[instrument(skip(self), ret(level = Level::TRACE))]
    fn count_block_options(&self) -> usize {
        let s = self.plot_guard_route();
        s.map
            .par_iter()
            .enumerate()
            .map(|(row_i, row)| {
                row.par_iter()
                    .enumerate()
                    .filter(move |(col_i, it)| {
                        if matches!(it, MapItem::Visited) {
                            let (mut ss, mut fs) = (self.clone(), self.clone());
                            ss.map[row_i][*col_i] = MapItem::NewWall;
                            fs.map[row_i][*col_i] = MapItem::NewWall;
                            while ss.try_walk_steps(1).is_some() && fs.try_walk_steps(2).is_some() {
                                if ss.guard == fs.guard {
                                    trace!(row_i = row_i, col_i = col_i, "found loop\n{fs}");
                                    return true;
                                }
                            }
                        }
                        false
                    })
                    .count()
            })
            .sum()
    }
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        count_loop_options(line_reader)?;
    } else {
        count_guard_area(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn count_guard_area(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut map = vec![];
    for line in line_reader.filter_map(Result::ok) {
        map.push(line.chars().collect());
    }
    let scanner: Scanner = map.try_into()?;
    let scanner = scanner.plot_guard_route();
    debug!("success walk\n{scanner}");
    Ok(scanner.count_visited())
}

#[instrument(skip_all, ret)]
fn count_loop_options(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut map = vec![];
    for line in line_reader.filter_map(Result::ok) {
        map.push(line.chars().collect());
    }
    let scanner: Scanner = map.try_into()?;
    Ok(scanner.count_block_options())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"....#.....
                      .........#
                      ..........
                      ..#.......
                      .......#..
                      ..........
                      .#..^.....
                      ........#.
                      #.........
                      ......#..."#;
        let res = count_guard_area(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 41);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"....#.....
                      .........#
                      ..........
                      ..#.......
                      .......#..
                      ..........
                      .#..^.....
                      ........#.
                      #.........
                      ......#..."#;
        let res = count_loop_options(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 6);
    }
}
