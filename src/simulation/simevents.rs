use ggez::timer;
use ggez::event;
use ggez::graphics;
use ggez::GameResult;
use ggez::nalgebra as na;

use crate::simulation::Automaton;
use crate::simulation::Simulation;

/// Implementation of the EventHandler trait for Simulation
impl<T: Automaton> event::EventHandler for Simulation<T> {

    /// A method that is called when the simulation update is triggered by the event loop
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        // If FPS is set to 0, then no rate-limiting
        if self.fps == 0 {
            // Advance the automaton state
            self.automaton.advance();
        
        // Otherwise refresh the graphics with the set FPS rate
        } else {
            // Wait for the FPS time to elapse
            while timer::check_update_time(ctx, self.fps) {
                // Advance the automaton state
                self.automaton.advance();
            }
        }

        // Return GameResult::Ok
        Ok(())
    }

    /// A method that is called when a render event is to be triggered by the event loop
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Clear the graphics window
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // Check if the simulation has been initialized
        if !self.initialized {
            // Retrieve the size of the graphics window
            let width = graphics::screen_coordinates(ctx).w;
            let height = graphics::screen_coordinates(ctx).h;
            // Create rectangle dimensions
            let dimensions = graphics::Rect::new(0.0, 0.0, width, height);

            // Initialize the automaton with the screen dimensions
            self.automaton.initialize(dimensions);
            // Set the initialized flag to true
            self.initialized = true;
        }

        // Render the automaton state
        self.automaton.draw(ctx, (na::Point2::new(0.0, 0.0),).into())?;
        // Present the graphics on the window
        graphics::present(ctx)?;
        
        // Return an GameResult::Ok
        Ok(())
    }
}