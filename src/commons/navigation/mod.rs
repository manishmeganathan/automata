mod turmite;
mod direction4;

pub use turmite::Turmite;
pub use direction4::Direction4;

/// A trait for grid orientation.
/// The orientation must be cloneable
pub trait Orient: Clone {
    /// A constructor method that generates a random orientation
    fn random() -> Self;

    /// A method that rotates the orientation clockwise
    fn turn_right(&self) -> Self;

    /// A method that rotates the orientation counterclockwise
    fn turn_left(&self) -> Self;
    
    /// A method that flips the orientation
    fn turn_around(&self) -> Self;
}