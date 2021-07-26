use ggez::graphics;
use crate::simulation::simulables::{SimCell, SimGrid};
use crate::commons::grids::scanner::{GridCell, GridScanner};

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
}

/// Implementation of builder methods for CellGrid.
/// A collection of functions that build various intial states of the grid.
impl<T: SimCell> CellGrid<T> {
    /// A function that creates a randomized grid of cells for the
    /// given cell size and grid dimensions. All cell states have
    /// an equal probability of occuring on the grid.
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

