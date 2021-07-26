use ggez::graphics;

// A trait for a simulation cell
// The cell must be cloneable
pub trait SimCell: Clone {
    // A constructor method that generates a random 
    // new cell with a balanced generator.
    fn balanced() -> Self;
    // A constructor method that generates a random
    // new cell with a skewed generator.
    fn skewed(skew: &str, bias: i8) -> Self;
}

// A trait for simulation grids.
// The grid must be cloneable, iterable and drawable.
pub trait SimGrid: Clone + IntoIterator {
    // Defines the type of cell used in the grid
    type Cell;

    // A constructor method
    fn new(cellsize: f32) -> Self;

    //
    fn setgrid(&mut self, other: Vec<Vec<Self::Cell>>);

    fn setdimensions(&mut self, dimensions: graphics::Rect);
}

pub trait Automaton {
    fn new(initialstate: &str, cellsize: f32) -> Self;
    
    fn initialize(&mut self, initialstate: &str, dimensions: graphics::Rect);

    fn advance(&mut self);
    // fn state() -> str;
}

