use ggez::GameResult;
use crate::commons::simulables::SimGrid;

// A generic struct that represents a simulation
pub struct Simulation<T> where T: SimGrid {
    // Represents the simulation grid
    pub grid: T,
    // Represents the simulation refresh rate
    pub fps: u32,
    // Represents whether the simulation has been initialized
    pub initialized: bool,
}

// Constructor implementation for Simulation
impl<T: SimGrid> Simulation<T> {
    // A constructor function that creates a new simulation with
    // the given cell size (pixels) and refresh rate (seconds).
    pub fn new(cellsize: f32, fps: u32) -> GameResult<Self> {
        Ok(Self {
            grid: T::new(cellsize),
            initialized: false,
            fps,
        })
    }
}