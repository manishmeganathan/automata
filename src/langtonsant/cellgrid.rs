use crate::langtonsant::LangtonsAnt;
use crate::commons::grids::CellGrid;
use crate::commons::cells::BinaryCell;
use crate::simulation::{SimGrid, Automaton};


/// Implementation of the Automaton trait for Langton's Ant with a CellGrid grid,
impl Automaton for LangtonsAnt<CellGrid<BinaryCell>> {
    /// Defines the type of grid for the automaton.
    type Grid = CellGrid<BinaryCell>;

    /// A constructor method that creates a null automaton
    /// ands sets the initial state and cell size parameters.
    fn new(initialstate: &str, cellsize: f32) -> Self {
        Self {
            grid: Self::Grid::new(cellsize),
            initialstate: initialstate.to_string(),
            cellsize,
            generation: 0,
            ant: None,
        }
    }

    /// A method that returns the state of the automaton as a string.
    /// Format: "Generation: {} | Alive: {} | Dead: {}"
    fn state(&self) -> String {
        format!("Generation: {}", self.generation)
    }

    /// A method that returns the name of the automaton as a string.
    /// Format: "Langton's Ant"
    fn name(&self) -> String {
        "Langton's Ant".to_string()
    }

    /// A method that returns the name of the automaton as a string 
    /// along with its initial state and grid type.
    /// Format: "Langton's Ant | Grid | {}"
    fn fullname(&self) -> String {
        format!("Langton's Ant | Grid | {}", self.initialstate)
    }
}
