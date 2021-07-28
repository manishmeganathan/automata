mod cellgrid;

use crate::simulation::SimGrid;

/// A struct that represents the automaton for 
/// Conway's Game of Life on a generic grid.
pub struct GameOfLife<T> where T: SimGrid {
    /// Represents the automaton grid.
    grid: T,    
    /// Represents the size of an individual cell.
    cellsize: f32,
    /// Represents the intial state of the automaton.
    initialstate: String,
    /// Represents the number of times the grid has been updated
    generation: u32,
    /// Represents the number of cells that are alive
    alive: u32,
    /// Represents the number of cells that are dead
    dead: u32,
}
