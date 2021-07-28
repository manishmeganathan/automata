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
