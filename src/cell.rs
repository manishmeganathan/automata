use rand::Rng;

// An enum that represents the possible states
// of a cell in Conway's Game of Life
#[derive(Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// Constructor for a Cell
impl Cell {
    // A function that generates a new Cell with a random cell state
    pub fn new() -> Self {
        // Randomly generate a cell state by generating a random number
        // between 0 and 10. If the random number is divisible by 3, the 
        // cell is alive. Otherwise, the cell is dead.
        rand::thread_rng().gen_range(0..10).into()
    }
}

// Implementation of the From<i32> trait for the Cell
impl From<i32> for Cell {
    // Create a cell variant based on an int value.
    fn from(i: i32) -> Self {
        match i%3 {
            // If value is divisible by 3, the cell is alive.
            0 => Cell::Alive,
            // Otherwise, the cell is dead.
            _ => Cell::Dead,
        }
    }
}

// Implementation of the Clone trait for Cell
impl Clone for Cell {
    fn clone(&self) -> Self {
        match self {
            // Cloning a dead cell
            Cell::Dead => Cell::Dead,
            // Cloning a live cell
            Cell::Alive => Cell::Alive,
        }
    }
}
