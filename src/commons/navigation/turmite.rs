use crate::simulation::SimGrid;
use crate::commons::grids::GridCell;
use crate::commons::navigation::Orient;
use crate::commons::grids::CellGrid;
use crate::commons::cells::BinaryCell;
use crate::commons::navigation::Direction4;

/// A struct that represents an automaton turmite.
/// i.e an agent that crawls the simulation grid
pub struct Turmite<T> where T: SimGrid {
    /// Represents the current orientation.
    pub orientation: T::Orientation,
    /// Represents the current location on the grid
    pub position: GridCell<T::Cell>, 
    /// Represents whether the turmite is alive
    pub active: bool,
    /// Represents the number of steps the turmite has been alive
    pub step: u32,
}

/// Implementation of Turmite constructor
impl<T: SimGrid> Turmite<T> {
    /// A constructor that initializes the turmite 
    /// with a random orientation and location.
    pub fn new(grid: &T) -> Self {
        Self{
            orientation: T::Orientation::random(),
            position: grid.randomcell(),
            active: true,
            step: 0,
        }
    }
}

/// Implementation of the Clone trait for Turmite
impl<T:SimGrid> Clone for Turmite<T> {
    /// A method that returns a new BinaryCell with the same state as the current BinaryCell.
    fn clone(&self) -> Self {
        Self{
            orientation: self.orientation.clone(),
            position: self.position,
            active: self.active,
            step: self.step,
        }
    }
}

/// Implementation of helper methods for a Turmite
/// on CellGrid with Binary cells.
impl Turmite<CellGrid<BinaryCell>> {
    /// A method that returns the flipped cell state
    /// of the current cell the turmite is on.
    pub fn flipcell(&mut self) -> BinaryCell {
        // Retrieve the current cell from the position of the turmite
        let (x, y, cell) = self.position;

        // Check the cell state the flip it
        let newcell = match cell {
            BinaryCell::Active => BinaryCell::Passive,
            BinaryCell::Passive => BinaryCell::Active
        };

        // Update the turmite's position with the new cell state
        self.position = (x,y, newcell);
        // Return the new cell state
        return newcell
    }

    // A method that moves the turmite forward one step in the current direction
    // Makes no change if the turmite is at the edge of the grid and kills the turmite.
    pub fn move_forward(&mut self, grid: &Vec<Vec<BinaryCell>>) {
        // Get the cell state and position of the ant
        let (x, y, _) = self.position;

        // Check the orientation of the ant and move it one unit forward
        match self.orientation {
            // If the ant is facing north
            Direction4::North => {
                // Check that incremented value is within the grid bounds
                if y+1 < grid[0].len() {
                    // Increment the y position of the ant
                    self.position = (x, y+1, grid[x][y+1]);
                } else {
                    // If the y position is out of bounds, set the ant to inactive
                    self.active = false;
                }
            },

            // If the ant is facing east
            Direction4::East => {
                // Check that incremented value is within the grid bounds
                if x+1 < grid.len() {
                    // Increment the x position of the ant
                    self.position = (x+1, y, grid[x+1][y]);
                } else {
                    // If the x position is out of bounds, set the ant to inactive
                    self.active = false;
                }
            },

            // If the ant is facing south
            Direction4::South => {
                // Check if y is 0, decrement will result in out of bounds and a stack overflow
                if y == 0 {
                    // Set the ant to inactive
                    self.active = false;
                } else {
                    // Decrement the y position of the ant
                    self.position = (x, y-1, grid[x][y-1]);
                }
            },

            // If the ant is facing west
            Direction4::West => {
                // Check if x is 0, decrement will result in out of bounds and a stack overflow
                if x == 0 {
                    // Set the ant to inactive
                    self.active = false;
                } else {
                    // Decrement the x position of the ant
                    self.position = (x-1, y, grid[x-1][y]);
                }
            },
        };
    }
}