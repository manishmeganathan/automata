use rand::Rng;
use crate::commons::simulables::SimCell;

/// An enum that represents the states of a binary cell.
///
/// Possibile states:
/// - ``BinaryCell::Active`` <- represents the ON state
/// - ``BinaryCell::Passive`` <- represents the OFF state
#[derive(Debug, PartialEq)]
pub enum BinaryCell {
    // Represent the off state
    Passive = 0,
    // Represent the on state
    Active = 1,
}

/// Implementation of constructors for BinaryCell
impl SimCell for BinaryCell {
    /// A constructor function that generates a new BinaryCell with a balanced random state.
    /// Balanced generation means that there is an equal probability for a cell to be passive or active.
    fn balanced() -> Self {
        // Randomly generate a number between 0 and 1.
        let val = rand::thread_rng().gen_range(0..=1);

        // Check the value of generated number.
        match val {
            // If generated number is 0, return a Passive cell.
            0 => BinaryCell::Passive,
            // If generated number is 1, return a Active cell.
            _ => BinaryCell::Active,
        }
    }

    /// A constructor function that generates a new BinaryCell with a skewed random state.
    /// Skewed generation means that there is a higher probability for a cell to be a particular state.
    ///
    /// - @param *skew* is a bool that determines whether the skew state is passive or active.
    /// - @param *bias* is an i8 that represents the degree of skew towards the skew state. 
    ///
    /// A bias of 1 is equivalent to a balanced state generation regardless of the skew value 
    /// and a bias of 100 with a skew state of true, means the chance of a Passive cell is 1 in 100.
    ///
    /// Valid values for skew are "active" and "passive".
    fn skewed(skew: &str, bias: i8) -> Self {
        // Randomly generate a number between 0 and the given bias
        let val = rand::thread_rng().gen_range(0..=bias);

        // Check the skew value
        match skew {
            // Skew towards the active state
            "active" => match val {
                // If generated number is 0, return a Passive cell.
                0 => BinaryCell::Passive,
                // Otherwise return an Active cell.
                _ => BinaryCell::Active,
            },
            // Skew towards the passive state
            "passive" => match val {
                // If generated number is 0, return an Active cell.
                0 => BinaryCell::Active,
                // Otherwise return a Passive cell.
                _ => BinaryCell::Passive,
            }
            // Invalid skew value
            _ => panic!("invalid skew value")
        }
    }
}

/// Implementation of the Clone trait for BinaryCell
impl Clone for BinaryCell {
    /// A method that returns a new BinaryCell with the same state as the current BinaryCell.
    fn clone(&self) -> Self {
        match self {
            // Cloning a passive cell
            BinaryCell::Passive => BinaryCell::Passive,
            // Cloning an active cell
            BinaryCell::Active => BinaryCell::Active,
        }
    }
}