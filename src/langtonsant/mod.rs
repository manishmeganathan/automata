pub mod cellgrid;

use crate::simulation::SimGrid;
use crate::commons::navigation::Turmite;

/// A struct that represents the automaton 
/// for Langton's Ant on a generic grid.
pub struct LangtonsAnt<T> where T: SimGrid {
    /// Represents the automaton grid.
    grid: T,    
    /// Represents the size of an individual cell.
    cellsize: f32,
    /// Represents the intial state of the automaton.
    initialstate: String,
    /// Represents the number of times the grid has been updated
    generation: u32,
    /// Represents langton's ant on the grid
    ant: Option<Turmite<T>>,
}
