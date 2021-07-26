use ggez::graphics;
//use ggez::nalgebra as na;
use crate::simulation::simulables::{SimCell, SimGrid};
use crate::commons::grids::scanner::{GridCell, GridScanner};

// A struct that represents a grid of cells
#[derive(Debug)]
pub struct CellGrid<T> where T:SimCell {
    // Represents the size of a single cell
    pub cellsize: f32,
    // Represents the 2D vector of grid cells
    pub vector: Option<Vec<Vec<T>>>,
    // Represents the bounds of the grid
    pub dimensions: Option<graphics::Rect>,
}

// Implementation of SimGrid<T> trait for Grid
impl<T: SimCell> SimGrid for CellGrid<T> {
    type Cell = T;

    // A constructor function that creates a null grid for a given cell size in pixels.
    // The height of the state banner is set to 50px.
    fn new(cellsize: f32) -> Self {
        Self {
            cellsize,
            vector: None,
            dimensions: None,
        }
    }

    // 
    fn setgrid(&mut self, other: Vec<Vec<Self::Cell>>) {
        self.vector = Some(other);
    }

    fn setdimensions(&mut self, other: graphics::Rect) {
        self.dimensions = Some(other);
    }
}

impl<T: SimCell> CellGrid<T> {
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
                // Create a new cell and push it into the column vector
                column.push(T::balanced());
            }

            // Push the column vector into the row vector (grid)
            gridvector.push(column);
        }  

        // Return the grid vector
        return gridvector
    }
}

// Implementation of the Clone trait for Grid
impl<T: SimCell> Clone for CellGrid<T> {
    fn clone(&self) -> Self {
        Self {
            cellsize: self.cellsize,
            vector: self.vector.clone(),
            dimensions: self.dimensions.clone(),
        }
    }
}

// Implementation of the IntoIterator trait for Grid
impl<T: SimCell> IntoIterator for CellGrid<T> {
    // Defines the iteration item
    type Item = GridCell<T>;
    // Defines the iteration iterator
    type IntoIter = GridScanner<CellGrid<T>>;

    // Defines the iterator's initial state
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

