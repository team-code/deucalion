//! Provides the Direction enum and utilities for working with it

/// Represents any of the four cardinal directions.
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
            Up => 0,
            Down => 180,
            Left => 270,
            Right => 90,
        }
    }
}
