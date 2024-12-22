use strum::EnumIter;

/// (row, col)
pub type Coord = (usize, usize);

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
    TopLeft = 4,
    BottomLeft = 5,
    TopRight = 6,
    BottomRight = 7,
}

pub const SIDE_MOVES: [Direction; 4] = [
    Direction::Left,
    Direction::Top,
    Direction::Right,
    Direction::Bottom,
];

impl Direction {
    pub fn new_coord(&self, (row, col): Coord) -> Option<Coord> {
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
