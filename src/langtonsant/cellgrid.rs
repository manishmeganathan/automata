use ggez::graphics;
use ggez::nalgebra as na;

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

// Implementation of the Drawable trait for GameOfLife with a CellGrid grid,
impl graphics::Drawable for LangtonsAnt<CellGrid<BinaryCell>> {

    // A method that returns the dimensions of the automaton
    fn dimensions(&self, _ctx: &mut ggez::Context) -> Option<graphics::Rect> {
        // Get the grid dimesions and add the banner height
        if let Some(dimensions) = &self.grid.dimensions {
            Some(graphics::Rect::new(0.0, 0.0, dimensions.w, dimensions.h + 60.0))
        } else {None}     
    }

    // A method that returns the graphics blending mode of the automaton grid
    fn blend_mode(&self) -> Option<graphics::BlendMode> {
        Some(graphics::BlendMode::Add)
    }

    // A method that set the graphics blend mode of the automaton grid (currently does nothing)
    fn set_blend_mode(&mut self, _: Option<graphics::BlendMode>) {}

    // A method that renders the automaton grid and state and returns a GameResult
    fn draw(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult<()> {
        
        // Create a new graphic mesh builder
        let mut mb = graphics::MeshBuilder::new();

        // Iterate through each cell in the grid
        for (x, y, cell) in self.grid.clone() {

            // Create the bounds of the cell
            let cellbounds = graphics::Rect::new(
                (x as f32) * self.cellsize,
                (y as f32) * self.cellsize,
                self.cellsize,
                self.cellsize,
            );

            // Add the cell fill to the mesh builder
            mb.rectangle(
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                cellbounds,
                // Set the cell color based on cell state
                match cell {
                    BinaryCell::Passive => [0.0, 0.0, 0.0, 1.0].into(),
                    BinaryCell::Active => [1.0, 1.0, 1.0, 1.0].into(),
                },
            )
            // Add the cell boundary to the mesh builder
            .rectangle(
                graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
                cellbounds,
                [1.0, 1.0, 1.0, 0.25].into(),
            );
        }

        // Check if the ant is initialized
        if let Some(ant) = &self.ant {
            // Get the position of the ant
            let (x, y, _) = ant.position;
            // Create the centroid of the ant sprite based on the position
            let centroid = na::Point2::new((x as f32 * self.cellsize) + self.cellsize/2.0, (y as f32 * self.cellsize) + self.cellsize/2.0);
            // Define a metric of distance between the ant sprite vertices
            let unitdist = self.cellsize/4.0;

            // Get the vertices of the ants sprite based on its orientation.
            // The ant sprite is a triangle pointing in the direction of its orientation
            let points = match ant.orientation {
                Direction4::North => {[
                    na::Point2::new(centroid.x, centroid.y + unitdist), 
                    na::Point2::new(centroid.x + unitdist, centroid.y - unitdist), 
                    na::Point2::new(centroid.x - unitdist, centroid.y - unitdist)
                ]},
                Direction4::East => {[
                    na::Point2::new(centroid.x + unitdist, centroid.y),
                    na::Point2::new(centroid.x - unitdist, centroid.y - unitdist),
                    na::Point2::new(centroid.x - unitdist, centroid.y + unitdist)
                ]},
                Direction4::South => {[
                    na::Point2::new(centroid.x, centroid.y - unitdist),
                    na::Point2::new(centroid.x - unitdist, centroid.y + unitdist),
                    na::Point2::new(centroid.x + unitdist, centroid.y + unitdist)
                ]},
                Direction4::West => {[
                    na::Point2::new(centroid.x - unitdist, centroid.y),
                    na::Point2::new(centroid.x + unitdist, centroid.y + unitdist),
                    na::Point2::new(centroid.x + unitdist, centroid.y - unitdist)
                ]},
            };

            // Set the color of the ant based on if its alive
            let color;
            if ant.active {
                // Red color if ant is alive
                color = [1.0, 0.0, 0.0, 1.0];
            } else {
                // Blue color is ant is dead
                color = [0.0, 1.0, 0.0, 1.0];
            }

            // Construct the triangle polygon from the sprite vertices
            mb.polygon(
                graphics::DrawMode::Fill(graphics::FillOptions::default()), 
                &points, 
                color.into()
            )?;
        }
        
        // Build and Draw the mesh
        mb.build(ctx)?.draw(ctx, param)?;

        // Declare a variable for the font size
        let font_size = 18.0;

        // Create the text graphics for the banner
        let mut name_text = graphics::Text::new(self.fullname());
        let mut state_text = graphics::Text::new(self.state());

        // Set the font styling for the text graphics
        state_text.set_font(graphics::Font::default(), graphics::Scale::uniform(font_size));
        name_text.set_font(graphics::Font::default(), graphics::Scale::uniform(font_size));

        // Chekc the grid dimesions
        if let Some(dimensions) = &self.grid.dimensions {
            // Calculate the spacing between banner elements.
            // Assumes 2 units of spacing above the name text and below the state text
            // and 1 unit of spacing between the name and state text.
            let spacing = (60.0 - (font_size * 2.0)) / 5.0;

            // Calculate the position of the name text
            let name_offset = dimensions.h + (spacing * 2.0);
            // Calculate the position of the state text
            let state_offset = dimensions.h + (spacing * 3.0) + font_size;

            // Draw the banner text graphics
            name_text.draw(ctx, (na::Point2::new(param.dest.x + 10.0, param.dest.y + name_offset),).into())?;
            state_text.draw(ctx, (na::Point2::new(param.dest.x + 10.0, param.dest.y + state_offset),).into())?;
        }

        // Return GameResult::Ok
        Ok(())
    }
}
