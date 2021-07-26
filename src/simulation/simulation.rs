use ggez::GameResult;
use crate::simulation::simulables::{Automaton};

// A generic struct that represents a simulation
pub struct Simulation<T> where T: Automaton {
    // Represents the automaton to simulate
    pub automaton: T,
    // Represents the refresh rate of the simulation 
    pub fps: u32,
    pub cellsize: f32,
    // Represents whether the automaton has been initialized
    pub initialized: bool,
}

// Constructor implementation for Simulation
impl<T: Automaton> Simulation<T> {
    // A constructor function that creates a new simulation with
    // the given cell size (pixels) and refresh rate (seconds).
    pub fn new(initialstate: &str, cellsize: f32, fps: u32) -> GameResult<Self> {
        Ok(Self {
            fps,
            cellsize,
            automaton: T::new(initialstate, cellsize), 
            initialized: false,
        })
    }

}