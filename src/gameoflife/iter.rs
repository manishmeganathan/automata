use crate::gameoflife::cell::Cell;

// A type alias for a Cell and its position in the grid
pub type GridIteratorItem = (usize, usize, Cell);

// A struct that represents an iterator over a Grid
pub struct GridIterator {
    // The Grid being iterated over
    pub grid: Vec<Vec<Cell>>,
    // The current column position in the Grid
    pub current_column: usize,
    // The current row position in the Grid
    pub current_row: usize,
}

// Implementation of the Iterator trait for GridIterator
impl Iterator for GridIterator {
    // Defines the iteration item
    type Item = GridIteratorItem;

    // A method that advances the iterator to the next position in the Grid
    fn next(&mut self) -> Option<Self::Item> {
        // Increment the row position
        let new_row = self.current_row + 1;

        // Check if row position is out of bounds
        if new_row != self.grid[self.current_column].len() {
            // If within bounds, update current row
            self.current_row = new_row;
        } else {
            // Otherwise, reset the current row
            self.current_row = 0;

            // Increment the column position
            let new_column = self.current_column + 1;

            // Check if column position is out of bounds
            if new_column != self.grid.len() {
                // If within bounds, update current column
                self.current_column = new_column;
            } else {
                // Otherwise, return None (end of iteration)
                return None;
            }
        }

        // Return the current position in the Grid and the Cell in that position
        Some((
            self.current_column, self.current_row,
            self.grid[self.current_column][self.current_row].clone(),
        ))
    }
}