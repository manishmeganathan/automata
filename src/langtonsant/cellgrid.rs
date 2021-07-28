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
}
