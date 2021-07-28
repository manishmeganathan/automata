use ggez::GameResult;
use crate::simulation::Automaton;

/// A struct that represents the simulator for a generic automaton.
pub struct Simulation<T> where T: Automaton {
    /// Represents the automaton that is being simulated.
    pub automaton: T,
    /// Represents the refresh rate of the simulation.
    pub fps: u32,
    /// Represents whether the simulation has been initialized.
    pub initialized: bool,
}

/// Implementation of constructor for Simulation.
impl<T: Automaton> Simulation<T> {
    /// A constructor function that creates a new simulation automaton with
    /// the given intial state, cell size (pixels) and refresh rate (seconds).
    pub fn new(initialstate: &str, cellsize: f32, fps: u32) -> GameResult<Self> {
        // Create the simulation, wrap it in a GameResult and return it.
        Ok(Self{
            fps, initialized: false,
            automaton: T::new(initialstate, cellsize), 
        })
    }
}