use rand::Rng;
use ggez::graphics;

use crate::commons::navigation::Direction4;
use crate::simulation::{SimCell, SimGrid};
use crate::commons::grids::{GridCell, GridScanner};

/// A struct that represents a grid of generic cells.
/// The generic cell type must implement the SimCell trait.
#[derive(Debug)]
pub struct CellGrid<T> where T: SimCell {
    /// Represents the size of a single cell
    cellsize: f32,
    /// Represents the 2D vector of grid cells
    pub vector: Option<Vec<Vec<T>>>,
    /// Represents the 2D bounds of the grid
    pub dimensions: Option<graphics::Rect>,
}

/// Implementation of the SimGrid trait for CellGrid
impl<T: SimCell> SimGrid for CellGrid<T> {
    /// Define the type of cell in the grid
    type Cell = T;

    /// Define the type of compass
    type Orientation = Direction4;

    /// A constructor method that creates a null grid.
    /// Set the given cell size into the struct.
    fn new(cellsize: f32) -> Self {
        Self {
            cellsize,
            vector: None,
            dimensions: None,
        }
    }

    /// A setter method that sets the given grid vector to the struct.
    fn setgrid(&mut self, other: Vec<Vec<Self::Cell>>) {
        self.vector = Some(other);
    }

    /// A setter method that sets the given grid dimensions to the struct.
    fn setdimensions(&mut self, other: graphics::Rect) {
        self.dimensions = Some(other);
    }

    /// A getter method that returns the height of the grid (number of rows)
    /// Returns 0 if the grid is null.
    fn getheight(&self) -> usize {
        match &self.vector {
            None => 0,
            Some(vec) => vec.len(),
        }
    }

    /// A getter method that returns the width of the grid (number of columns).
    /// Returns 0 if the grid is null.
    fn getwidth(&self) -> usize {
        match &self.vector {
            None => 0,
            Some(vec) => vec[0].len(),
        }
    }

    /// A method that returns a random cell from the grid.
    /// Returns the x,y position of the cell along with cell state as a GridCell
    fn randomcell(&self) -> GridCell<Self::Cell> {
        // Check if grid exists
        if let Some(grid) = &self.vector {
            // Get a random column and row from the grid
            let col = rand::thread_rng().gen_range(0..grid.len());
            let row = rand::thread_rng().gen_range(0..grid[0].len());
            // Build the GridCell and return it
            return (col, row, grid[row][col])
        
        // If the grid is null, panic
        } else {
            panic!("random grid cell selection failed. grid is empty!")
        }
    } 
}

/// Implementation of the Clone trait for Grid
impl<T: SimCell> Clone for CellGrid<T> {
    /// A method that returns a new CellGrid with the same  
    /// vector and properties as the current CellGrid.
    fn clone(&self) -> Self {
        Self {
            cellsize: self.cellsize,
            vector: self.vector.clone(),
            dimensions: self.dimensions.clone(),
        }
    }
}

/// Implementation of the IntoIterator trait for Grid
impl<T: SimCell> IntoIterator for CellGrid<T> {
    /// Define the iteration item type
    type Item = GridCell<T>;
    /// Define the iterator type
    type IntoIter = GridScanner<CellGrid<T>>;

    /// A method that creates an iterator for the CellGrid.
    fn into_iter(self) -> Self::IntoIter {
        // Check the option value
        match self.vector {
            // Panic if GetOption::None is returned
            None => panic!("could not create grid iterator!"),
            // Create GridIterator with the grid
            Some(gridvector) => {
                GridScanner::<CellGrid<T>> {
                    gridvector,
                    current_column: 0,
                    current_row: 0,
                }
            }
        }
    }
}

