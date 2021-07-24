use rand::Rng;

/// An enum that represents the cardinal directions.
///
/// Possibile states:
/// - ``Direction::North`` <- represents the north direction
/// - ``Direction::South`` <- represents the south direction
/// - ``Direction::East`` <- represents the east direction
/// - ``Direction::West`` <- represents the west direction
#[derive(Debug, PartialEq)]
pub enum Direction {
    // Represents the north direction.
    North = 0,
    // Represents the east direction.
    East = 1,
    // Represents the south direction.
    South = 2,
    // Represents the west direction.
    West = 3,
}

/// Implementation of constructors for Direction.
impl Direction {
    /// A constructor function that generates a new Direction with a random direction
    pub fn random() -> Self {
        // Randomly generate a number between 0 and 3 (inclusive)
        // and return the corresponding Direction
        rand::thread_rng().gen_range(0..=3).into()
    }   
}

/// Implementation of rotation methods for Direction
impl Direction {
    /// A method that rotates the Direction 90 degrees clockwise and returns a new Direction.
    pub fn turn_right(&self) -> Self {
        // Check the current Direction and return the corresponding rotated Direction
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// A method that rotates the Direction 90 degrees counter-clockwise and returns a new Direction.
    pub fn turn_left(&self) -> Self {
        // Check the current Direction and return the corresponding rotated Direction
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    /// A method that rotates the Direction 180 degrees and returns a new Direction.
    pub fn turn_around(&self) -> Self {
        // Check the current Direction and return the corresponding rotated Direction
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

// Implementation of the From<i32> trait for Direction
impl From<i32> for Direction {
    /// A method that converts an i32 into a Direction
    ///
    /// - @param *i* is an i32 to that is converted into a Direction
    ///
    /// Direction-Int Mapping
    /// - 0 -> Direction::North 
    /// - 1 -> Direction::East
    /// - 2 -> Direction::South
    /// - 3 -> Direction::West
    fn from(i: i32) -> Self {
        match i {
            // 0 = North
            0 => Direction::North,
            // 1 = East
            1 => Direction::East,
            // 2 = South
            2 => Direction::South,
            // 3 = West
            3 => Direction::West,
            // Invalid Direction
            _ => panic!("Invalid direction: {:?}", i)
        }
    }
}

// Implementation of the Clone trait for Direction
impl Clone for Direction {
    /// A method that returns a copy of the current Direction
    fn clone(&self) -> Self {
        match self {
            Direction::North => Direction::North,
            Direction::East => Direction::East,
            Direction::South => Direction::South,
            Direction::West => Direction::West,
        }
    }
}
