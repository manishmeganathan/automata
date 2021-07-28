use crate::simulation::{SimCell, SimGrid};
use crate::commons::grids::CellGrid;
use crate::commons::grids::GridCell;

/// A struct that represents a scanning iterator for a SimGrid
pub struct GridScanner<T> where T: SimGrid {
    // The 2D vector of the grid being scanned
    pub gridvector: Vec<Vec<T::Cell>>,
    // The current column position in the Grid
    pub current_column: usize,
    // The current row position in the Grid
    pub current_row: usize,
}

/// Implementation of the Iterator trait for GridScanner
impl<C: SimCell> Iterator for GridScanner<CellGrid<C>> {
    /// Define the iterator item type
    type Item = GridCell<C>;

    /// A method that advances the iterator to the next position in the grid
    fn next(&mut self) -> Option<Self::Item> {
        // Retrieve the element based on the iterator's current position on the grid
        let element = match (self.current_column, self.current_row) {
            // If the scanner has reached the end of a row
            (x, y) if y == self.gridvector[x].len() => {
                // Rese the row position
                self.current_row = 0; 
                // Increment the column position
                self.current_column += 1;
                // Check if the column has reached the end of the grid
                if self.current_column == self.gridvector.len() {
                    // Return None. End of Iteration
                    return  None
                };

                // Build the grid cell element 
                Some((self.current_column, self.current_row, self.gridvector[self.current_column][self.current_row].clone()))
            },

            // Otherwise, build the grid cell element
            _ => Some((self.current_column, self.current_row, self.gridvector[self.current_column][self.current_row].clone()))
        };

        // Increment the row position
        self.current_row += 1;
        // Return the iteration element
        return element;
    }
}