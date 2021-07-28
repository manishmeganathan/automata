use rand::Rng;
use crate::commons::navigation::Orient;

/// An enum that represents the cardinal directions.
///
/// Possibile states:
/// - ``Direction4::North`` <- represents the north direction
/// - ``Direction4::South`` <- represents the south direction
/// - ``Direction4::East`` <- represents the east direction
/// - ``Direction4::West`` <- represents the west direction
#[derive(Debug, PartialEq)]
pub enum Direction4 {
    /// Represents the north direction.
    North = 0,
    /// Represents the east direction.
    East = 1,
    /// Represents the south direction.
    South = 2,
    /// Represents the west direction.
    West = 3,
}

impl Orient for Direction4 {
    /// A constructor function that generates a new Direction4 with a random direction
    fn random() -> Self {
        // Randomly generate a number between 0 and 3 (inclusive)
        // and return the corresponding Direction
        rand::thread_rng().gen_range(0..=3).into()
    }  

    /// A method that rotates the Direction 90 degrees clockwise and returns a new Direction.
    fn turn_right(&self) -> Self {
        // Check the current Direction and return the corresponding rotated Direction
        match self {
            Direction4::North => Direction4::East,
            Direction4::East => Direction4::South,
            Direction4::South => Direction4::West,
            Direction4::West => Direction4::North,
        }
    }

    /// A method that rotates the Direction 90 degrees counter-clockwise and returns a new Direction.
    fn turn_left(&self) -> Self {
        // Check the current Direction and return the corresponding rotated Direction
        match self {
            Direction4::North => Direction4::West,
            Direction4::East => Direction4::North,
            Direction4::South => Direction4::East,
            Direction4::West => Direction4::South,
        }
    }

    /// A method that rotates the Direction 180 degrees and returns a new Direction.
    fn turn_around(&self) -> Self {
        // Check the current Direction and return the corresponding rotated Direction
        match self {
            Direction4::North => Direction4::South,
            Direction4::East => Direction4::West,
            Direction4::South => Direction4::North,
            Direction4::West => Direction4::East,
        }
    }
}

/// Implementation of the From<i32> trait for Direction4
impl From<i32> for Direction4 {
    /// A method that converts an i32 into a Direction
    ///
    /// - @param *i* is an i32 to that is converted into a Direction
    ///
    /// Direction-Int Mapping
    /// - 0 -> Direction4::North 
    /// - 1 -> Direction4::East
    /// - 2 -> Direction4::South
    /// - 3 -> Direction4::West
    fn from(i: i32) -> Self {
        match i {
            // 0 = North
            0 => Direction4::North,
            // 1 = East
            1 => Direction4::East,
            // 2 = South
            2 => Direction4::South,
            // 3 = West
            3 => Direction4::West,
            // Invalid Direction
            _ => panic!("Invalid direction: {:?}", i)
        }
    }
}

/// Implementation of the Clone trait for Direction4
impl Clone for Direction4 {
    /// A method that returns a copy of the current Direction4
    fn clone(&self) -> Self {
        match self {
            Direction4::North => Direction4::North,
            Direction4::East => Direction4::East,
            Direction4::South => Direction4::South,
            Direction4::West => Direction4::West,
        }
    }
}