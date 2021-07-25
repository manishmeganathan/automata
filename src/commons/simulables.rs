use ggez::graphics;

// A trait for simulation grids.
// The must be clone-able, iterable and drawable.
pub trait SimGrid: Clone + IntoIterator + graphics::Drawable {
    // A constructor method
    fn new(size: f32) -> Self;
    // A method that sets the initial state of the grid
    fn initialize(&mut self, dimensions: graphics::Rect);
    // A method that updates the state of the grid
    fn update(&mut self);
}

pub trait SimCell: Clone {
    // A constructor method that generates a random 
    // new cell with a balanced generator.
    fn balanced() -> Self;
    // A constructor method that generates a random
    // new cell with a skewed generator.
    fn skewed(skew: &str, bias: i8) -> Self;
}