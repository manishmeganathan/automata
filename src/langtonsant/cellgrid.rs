use ggez::graphics;

use crate::langtonsant::LangtonsAnt;
use crate::commons::grids::CellGrid;
use crate::commons::cells::BinaryCell;
use crate::simulation::{SimGrid, Automaton};
use crate::commons::navigation::{Turmite, Orient, Direction4};


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

    /// A method that initializes the automaton for the given dimensions.
    fn initialize(&mut self, dimensions: graphics::Rect) {
        // Create a new dimensions object for the grid of cells (60 px removed for the banner)
        let griddimensions = graphics::Rect::new(0.0, 0.0, dimensions.w, dimensions.h - 60.0);
        // Set the grid dimensions to the grid 
        self.grid.setdimensions(griddimensions);
        
        // Check the value of the initial state field
        match self.initialstate.as_str() {
            // Default initial state (random-balanced)
            "default" => {
                // Set the initial state string of the automaton
                self.initialstate = "Empty".to_string();

                // Create a grid of random cells with a balanced ratio of dead and alive cells
                let randomgrid = CellGrid::<BinaryCell>::generate_empty_grid(self.cellsize, griddimensions);
                // Set the generated grid to the automaton grid
                self.grid.setgrid(randomgrid);

                self.ant = Some(Turmite::new(&self.grid));
            },

            // Invalid initial state
            _ => {
                // Print an error and exit
                eprintln!("[error] invalid initial state for 'gameoflife'");
                std::process::exit(0);
            }
        }
    }

    /// A method that advances the ant to the next generation.
    fn advance(&mut self) {
        // Check if the cell grid exists
        if self.grid.vector.is_some() {
            // Clone the ant and check if it is active
            let mut newant = self.ant.clone().unwrap();
            if !newant.active {
                // Return if the ant is inactive
                return;
            }

            // Check if the ant step and automaton generation are in sync
            if self.generation == newant.step {
                // Get the cell state of the current cell that the ant is on
                let (_, _, cell) = newant.position;
                // Rotate the ant based on the automaton rules
                let newdir = match cell {
                    // If cell is active, turn right
                    BinaryCell::Active => newant.orientation.turn_right(),
                    // If cell is inactive, turn left
                    BinaryCell::Passive => newant.orientation.turn_left(),
                };

                // Set the new orientation of the ant
                newant.orientation = newdir;
                // Increase the step count of the ant
                newant.step += 1;
                // Update the automaton ant with the new ant state
                self.ant = Some(newant);

            // If ant and automaton are not in sync
            } else {
                // Create a clone of the cell grid
                let mut newgrid = self.grid.vector.clone().unwrap();
                // Flip the current cell of the ant
                newant.flipcell();

                // Get the cell state and position of the ant
                let (x, y, cell) = newant.position;
                // Update the grid clone for that position with the new cell state
                newgrid[x][y] = cell;
                // Move the ant forward by one unit (kill turmite if at grid edge)
                newant.move_forward(&newgrid);

                // Update the automaton ant with the new ant state
                self.ant = Some(newant);
                // Update the automaton generation and sync with ant step count
                self.generation += 1;
                // Update the automaton grid with the new grid state
                self.grid.setgrid(newgrid);
            }
        }
    }

    /// A method that returns the state of the automaton as a string.
    /// Format: "Generation: {} | Alive: {} | Dead: {}"
    fn state(&self) -> String {
        format!("Generation: {}", self.generation)
    }

    /// A method that returns the name of the automaton as a string.
    /// Format: "Langton's Ant"
    fn name(&self) -> String {
        "Langton's Ant".to_string()
    }

    /// A method that returns the name of the automaton as a string 
    /// along with its initial state and grid type.
    /// Format: "Langton's Ant | Grid | {}"
    fn fullname(&self) -> String {
        format!("Langton's Ant | Grid | {}", self.initialstate)
    }
}
