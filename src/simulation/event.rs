use ggez::{event, graphics, nalgebra as na};
use ggez::GameResult;

use crate::simulation::sim::{SimGrid, Simulation};

// Implementation of the EventHandler trait for Simulation
impl<T: SimGrid> event::EventHandler for Simulation<T> {
    // A method that is called when the simulation state is to be updated
    fn update(&mut self, _: &mut ggez::Context) -> GameResult<()> {
        Ok(())
    }

    // A method that is called when the simulation the simulation is to be rendered
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Clear the graphics window
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());

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