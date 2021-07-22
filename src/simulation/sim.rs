use ggez::GameResult;
use ggez::graphics;

// A trait for simulation grids.
// The must be clone-able, iterable and drawable.
pub trait SimGrid: Clone + IntoIterator + graphics::Drawable {
    fn new(size: f32) -> Self;
    fn initialize(&mut self, dimensions: graphics::Rect);
}

// A generic struct that represents a simulation
pub struct Simulation<T> where T: SimGrid {
    // Represents the simulation grid
    pub grid: T,
    // Represents the refresh rate of the simulation
    pub refresh: u32,
    // Represents whether the simulation has been initialized
    pub initialized: bool,
}

// Constructor implementation for Simulation
impl<T: SimGrid> Simulation<T> {
    // A constructor function that creates a new simulation with
    // the given cell size (pixels) and refresh rate (seconds).
    pub fn new(cellsize: f32, refresh: u32) -> GameResult<Self> {
        Ok(Self {
            grid: T::new(cellsize),
            initialized: false,
            refresh,
        })
    }
}
