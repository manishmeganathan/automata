use ggez::graphics;

use crate::simulation::SimCell;
use crate::commons::cells::BinaryCell;
use crate::commons::grids::CellGrid;

/// Implementation of builder methods for CellGrid<T>.
/// A collection of functions that build various intial states of the grid.
impl<T: SimCell> CellGrid<T> {
    /// A function that creates a randomized grid of cells for the given cell size and grid dimensions. 
    /// All cell states have an equal probability of occuring on the grid.
    pub fn generate_randomgrid_balanced(cellsize: f32, dimensions:graphics::Rect) -> Vec<Vec<T>> {
        // Calculate the number of rows and columns in the grid
        let rows = dimensions.w / cellsize;
        let cols = dimensions.h / cellsize;

        // Create a new vector (represents rows)
        let mut gridvector = Vec::new();

        // Iterate for each row
        for _ in 0..(rows as i32) {
            // Create a new vector (represents columns)
            let mut column = Vec::new();
            // Iterate for each column
            for _ in 0..(cols as i32) {
                // Create a new cell with the balanced randomizer 
                // and push it into the column vector
                column.push(T::balanced());
            }

            // Push the column vector into the row vector (grid)
            gridvector.push(column);
        }  

        // Return the grid vector
        return gridvector
    }
}

/// Implementation of builder methods for CellGrid<BinaryCell>.
/// A collection of functions that build various intial states of the grid with binary cells.
impl CellGrid<BinaryCell> {
    /// A function that creates an empty grid of cells for the given cell size and grid dimensions.
    /// All cell states are set to BinaryCell::Passive.
    pub fn generate_empty_grid(cellsize: f32, dimensions:graphics::Rect) -> Vec<Vec<BinaryCell>> {
        // Calculate the number of rows and columns in the grid
        let rows = dimensions.w / cellsize;
        let cols = dimensions.h / cellsize;

        // Create a new vector (represents rows)
        let mut gridvector = Vec::new();

        // Iterate for each row
        for _ in 0..(rows as i32) {
            // Create a new vector (represents columns)
            let mut column = Vec::new();
            // Iterate for each column
            for _ in 0..(cols as i32) {
                // Create a new cell with the balanced randomizer 
                // and push it into the column vector
                column.push(BinaryCell::Passive);
            }

            // Push the column vector into the row vector (grid)
            gridvector.push(column);
        }  

        // Return the grid vector
        return gridvector
    }
}