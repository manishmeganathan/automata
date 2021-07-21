use crate::gameoflife::cell::Cell;
use crate::gameoflife::griditer::{GridIteratorItem ,GridIterator};

use ggez::graphics;

// A struct that represents the cell grid
#[derive(Debug)]
pub struct Grid {
    // Represents the 2D vector of grid cells
    cellgrid: Option<Vec<Vec<Cell>>>,
    // Represents the 2D rectangular bounds of the grid
    dimensions: Option<graphics::Rect>,
    // Represents the size of a single cell
    size: f32,
}

// Constructor implemntations for Grid
impl Grid {
    // A constructor function that creates a  
    // null grid for a given cell size in pixels
    pub fn new(size: f32) -> Self {
        Self {
            cellgrid: None,
            dimensions: None,
            size,
        }
    }

    // A method of Grid that initializes the grid for a given rectangle dimensions.
    pub fn initialize(&mut self, dimensions: graphics::Rect) {
        // Calculate the number of rows and columns in the grid
        let rows = dimensions.w / self.size;
        let cols = dimensions.h / self.size;

        // Create a new vector (represents rows)
        let mut grid = Vec::new();

        // Iterate for each row
        for _ in 0..(rows as i32) {
            // Create a new vector (represents columns)
            let mut column = Vec::new();
            // Iterate for each column
            for _ in 0..(cols as i32) {
                // Create a new cell and push it into the column vector
                column.push(Cell::new());
            }

            // Push the column vector into the row vector (grid)
            grid.push(column);
        }

        // Assign the initialized grid vector to the grid struct
        self.cellgrid = Some(grid);
        // Assign the dimensions to the grid struct
        self.dimensions = Some(dimensions);
    }
}

// Implementation of the Clone trait for Grid
impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
            cellgrid: self.cellgrid.clone(),
            dimensions: self.dimensions.clone(),
            size: self.size,
        }
    }
}

// Implementation of the IntoIterator trait for Grid
impl IntoIterator for Grid {
    // Defines the iteration item
    type Item = GridIteratorItem;
    // Defines the iteration iterator
    type IntoIter = GridIterator;

    // Defines the iterator's initial state
    fn into_iter(self) -> Self::IntoIter {
        // Check the option value
        match self.cellgrid {
            // Panic if GetOption::None is returned
            None => panic!("could not create grid iterator!"),
            // Create GridIterator with the grid
            Some(grid) => {
                GridIterator {
                    grid: grid,
                    current_column: 0,
                    current_row: 0,
                }
            }
        }
    }
}