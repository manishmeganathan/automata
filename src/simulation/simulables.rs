use ggez::graphics;
use crate::commons::grids::GridCell;
use crate::commons::navigation::Orient;

/// A trait for a simulation cell
/// The cell must be cloneable
pub trait SimCell: Clone + Copy {
    /// A constructor method that generates a random 
    /// new cell with a balanced ratio.
    fn balanced() -> Self;

    /// A constructor method that generates a random
    /// new cell with a skewed ratio.
    fn skewed(skew: &str, bias: i8) -> Self;
}

/// A trait for simulation grids.
/// The grid must be cloneable and iterable.
pub trait SimGrid: Clone + IntoIterator {
    /// Defines the type of cell used in the grid.
    type Cell: SimCell;
    
    /// Defines the type of navigation used in the grid.
    type Orientation: Orient; 

    /// A constructor method that create a null grid.
    fn new(cellsize: f32) -> Self;

    /// A setter method that sets the grid vector.
    fn setgrid(&mut self, other: Vec<Vec<Self::Cell>>);

    /// A setter method that sets the grid dimensions.
    fn setdimensions(&mut self, dimensions: graphics::Rect);

    /// A getter method that returns the grid vector's height (no.of rows).
    fn getheight(&self) -> usize;

    /// A getter method that returns the grid vector's width (no.of columns).
    fn getwidth(&self) -> usize;

    /// A method that returns a random cell from the grid.
    fn randomcell(&self) -> GridCell<Self::Cell>;
}

/// A trait for simulation automaton.
/// The automaton must be drawable.
pub trait Automaton: graphics::Drawable {
    /// Defines the type of grid used in the automaton.
    type Grid: SimGrid;

    /// A constructor method that creates a null automaton  
    /// and sets intialization parameters into the object.
    fn new(initialstate: &str, cellsize: f32) -> Self;

    /// A method that initializes the automaton with 
    /// its initial state for the given dimensions.
    fn initialize(&mut self, dimensions: graphics::Rect);

    /// A method that advances the automaton to the next generation.
    fn advance(&mut self);

    /// A method that returns the current state of the 
    /// automaton as a formatted string.
    fn state(&self) -> String;  

    /// A method that returns the name of the automaton as a string.
    /// Ex: "Conway's Game of Life"
    fn name(&self) -> String;

    /// A method that returns the name of the automaton along
    /// with its initial state and grid type as formatted string.
    /// Ex: "Conway's Game of Life | Grid | Random [1:1]"
    fn fullname(&self) -> String;    
}
