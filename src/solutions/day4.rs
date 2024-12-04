use std::io;

use strum::{EnumIter, IntoEnumIterator};
use tracing::{instrument, Level};

use crate::error::Result;

/// (row, col)
type Coord = (usize, usize);

#[derive(Debug, EnumIter, Clone, Copy)]
enum Direction {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
    TopLeft = 4,
    BottomLeft = 5,
    TopRight = 6,
    BottomRight = 7,
}

impl Direction {
    fn new_coord(&self, (row, col): Coord) -> Option<Coord> {
        match self {
            Direction::Top => row.checked_sub(1).zip(Some(col)),
            Direction::Right => Some(row).zip(col.checked_add(1)),
            Direction::Left => Some(row).zip(col.checked_sub(1)),
            Direction::Bottom => row.checked_add(1).zip(Some(col)),
            Direction::TopLeft => row.checked_sub(1).zip(col.checked_sub(1)),
            Direction::TopRight => row.checked_sub(1).zip(col.checked_add(1)),
            Direction::BottomLeft => row.checked_add(1).zip(col.checked_sub(1)),
            Direction::BottomRight => row.checked_add(1).zip(col.checked_add(1)),
        }
    }
}

#[derive(Debug)]
struct Scanner {
    matrix: Vec<Vec<char>>,
}

impl Scanner {
    fn new(matrix: Vec<Vec<char>>) -> Self {
        Self { matrix }
    }

    fn relative_char(&self, coord: Coord, dir: Direction) -> Option<(char, Coord)> {
        let new_coord = dir.new_coord(coord)?;
        let ch = self
            .matrix
            .get(new_coord.0)
            .and_then(|r| r.get(new_coord.1))?;
        Some((*ch, new_coord))
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn follow_xmas(&self, mut pref: String, coord: Coord, dir: Direction) -> bool {
        match pref.as_str() {
            "XMAS" => true,
            b if "XMAS".starts_with(b) => {
                let Some((ch, new_coord)) = self.relative_char(coord, dir) else {
                    return false;
                };
                pref.push(ch);
                self.follow_xmas(pref, new_coord, dir)
            }
            _ => false,
        }
    }

    fn count_xmas(&self) -> usize {
        let mut count = 0;
        for (row_i, row) in self.matrix.iter().enumerate() {
            for (col_i, ch) in row.iter().enumerate() {
                if ch == &'X' {
                    for dir in Direction::iter() {
                        count += self.follow_xmas("X".to_owned(), (row_i, col_i), dir) as usize;
                    }
                }
            }
        }
        count
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn collecti_diaonal(&self, coord: Coord) -> Option<(String, String)> {
        let mut left = ['A'; 2];
        let mut right = ['A'; 2];
        let (top_left_ch, _) = self.relative_char(coord, Direction::TopLeft)?;
        left[0] = top_left_ch;
        let (bot_right_ch, _) = self.relative_char(coord, Direction::BottomRight)?;
        left[1] = bot_right_ch;
        let (top_right_ch, _) = self.relative_char(coord, Direction::TopRight)?;
        right[0] = top_right_ch;
        let (bot_left_ch, _) = self.relative_char(coord, Direction::BottomLeft)?;
        right[1] = bot_left_ch;
        left.sort();
        right.sort();
        Some((left.into_iter().collect(), right.into_iter().collect()))
    }

    fn count_mas_x(&self) -> usize {
        let mut count = 0;
        for (row_i, row) in self.matrix.iter().enumerate() {
            for (col_i, ch) in row.iter().enumerate() {
                if ch == &'A' {
                    count += self
                        .collecti_diaonal((row_i, col_i))
                        .is_some_and(|(left, right)| {
                            (left.as_str(), right.as_str()) == ("MS", "MS")
                        }) as usize;
                }
            }
        }
        count
    }
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        scan_mas_x(line_reader)?;
    } else {
        scan_xmas(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn scan_xmas(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in line_reader.filter_map(Result::ok) {
        matrix.push(line.chars().collect());
    }
    let scanner = Scanner::new(matrix);
    Ok(scanner.count_xmas())
}

#[instrument(skip_all, ret)]
fn scan_mas_x(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in line_reader.filter_map(Result::ok) {
        matrix.push(line.chars().collect());
    }
    let scanner = Scanner::new(matrix);
    Ok(scanner.count_mas_x())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"#;
        let res = scan_xmas(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 18);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#".M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            .........."#;
        let res = scan_mas_x(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 9);
    }
}
