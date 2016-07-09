//! Tests for the direction module

use direction;

#[test]
fn test_direction_reverse() {
    let initial_direction = direction::Direction::Down;
    // The reverse of Down, should be up.
    assert_eq!(initial_direction.reverse(), direction::Direction::Up);
}

#[test]
fn test_direction_to_angle() {
    let direction = direction::Direction::Down;
    assert_eq!(direction.to_angle(), 180);
}
