//! Provides the Direction enum and utilities for working with it

/// Represents the four cardinal directions.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Return the angle represented by the direction, in degrees, starting from Up, going
    /// clockwise
    pub fn to_angle(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 180,
            Direction::Left => 270,
            Direction::Right => 90,
        }
    }

    /// Return the inverse of the current direction, Down->Up, Left->Right, etc.
    pub fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
