use crate::commons::simulables::SimGrid;

use ggez::timer;
use ggez::event;
use ggez::graphics;
use ggez::GameResult;
use ggez::nalgebra as na;

// A generic struct that represents a simulation
pub struct Simulation<T> where T: SimGrid {
    // Represents the simulation grid
    pub grid: T,
    // Represents the refresh rate of the simulation
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

// Implementation of the EventHandler trait for Simulation
impl<T: SimGrid> event::EventHandler for Simulation<T> {
    // A method that is called when the simulation state is to be updated
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        // If FPS is set to 0, then no rate-limiting
        if self.fps == 0 {
            // Update the simulation state
            self.grid.update();
        
        // Otherwise refresh the graphics with the set FPS rate
        } else {
            // Wait for the FPS time to elapse
            while timer::check_update_time(ctx, self.fps) {
                // Update the simulation state
                self.grid.update();
            }
        }

        // Return GameResult::Ok
        Ok(())
    }

    // A method that is called when the simulation the simulation is to be rendered
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Clear the graphics window
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // Retrieve the size of the graphics window
        let width = graphics::screen_coordinates(ctx).w;
        let height = graphics::screen_coordinates(ctx).h;

        // Check if the grid has been initialized
        if !self.initialized {
            // Initialize the grid
            self.grid.initialize(graphics::Rect::new(0.0, 0.0, width, height));
            // Set the initialized flag to true
            self.initialized = true;
        }

        // Render the grid
        self.grid.draw(ctx, (na::Point2::new(0.0, 0.0),).into())?;
        // Present the graphics on the window
        graphics::present(ctx)?;
        // Return an GameResult::Ok
        Ok(())
    }
}