use ggez::graphics;
use ggez::GameResult;
use ggez::nalgebra as na;

use crate::simulation::simulables::{SimGrid, Automaton};
use crate::commons::cells::binarycell::BinaryCell;
use crate::commons::grids::cellgrid::CellGrid;
use crate::gameoflife::GameOfLife;

/// Implementation of the Automaton trait for GameOfLife with a CellGrid grid,
impl Automaton for GameOfLife<CellGrid<BinaryCell>> {
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
            alive: 0,
            dead: 0,
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
                self.initialstate = "Random [1:1]".to_string();

                // Create a grid of random cells with a balanced ratio of dead and alive cells
                let randomgrid = CellGrid::<BinaryCell>::generate_randomgrid_balanced(self.cellsize, griddimensions);
                // Set the generated grid to the automaton grid
                self.grid.setgrid(randomgrid);
            },
            // Balanced Random initial state
            "random-balanced" => {
                // Set the initial state string of the automaton
                self.initialstate = "Random [1:1]".to_string();

                // Create a grid of random cells with a balanced ratio of dead and alive cells
                let randomgrid = CellGrid::<BinaryCell>::generate_randomgrid_balanced(self.cellsize, griddimensions);
                // Set the generated grid to the automaton grid
                self.grid.setgrid(randomgrid);
            },

            // Invalid initial state
            _ => {
                // Print an error and exit
                eprintln!("[error] invalid initial state for 'gameoflife'");
                std::process::exit(0);
            }
        }
    }

    /// A method that advances the game of life to the next generation.
    fn advance(&mut self) {
        // Declare counter variables for the number of alive and dead cells
        let mut alive: u32 = 0;
        let mut dead: u32 = 0;

        // Check if the cell grid exists
        if self.grid.vector.is_some() {
            // Create a clone of the cell grid
            let mut newgrid = self.grid.vector.clone().unwrap();

            // Iterate over the grid
            for (x, y, cell) in self.grid.clone() {

                // Check the vicinity of the cell
                let cell = match (cell, self.scan_vicinity(x, y)) {
                    // If a cell is alive, and there are either too many live  
                    // neighbors or not enough live neighbors, kill it.
                    (BinaryCell::Active, n) if n < 2 || n > 3 => BinaryCell::Passive,

                    // If a cell is alive and has either 2 
                    // or 3 live neighbors, keep it alive
                    (BinaryCell::Active, n) if n == 3 || n == 2 => BinaryCell::Active,

                    // If a cell is dead and has exactly 3 live neighbors, revive it
                    (BinaryCell::Passive, 3) => BinaryCell::Active,

                    // Otherwise, keep the cell state
                    (c, _) => c,
                };

                // Add the new cell to the new grid
                newgrid[x][y] = cell.clone();

                // Increment the alive or dead counter
                match cell {
                    BinaryCell::Passive => dead += 1,
                    BinaryCell::Active => alive += 1
                }
            }

            // Assign the new grid to the grid struct
            self.grid.setgrid(newgrid);
        }

        // Update the alive and dead cell value in the grid struct
        self.alive = alive;
        self.dead = dead;
        // Increment the generation value in the grid struct
        self.generation += 1;
    }   

    /// A method that returns the state of the automaton as a string.
    /// Format: "Generation: {} | Alive: {} | Dead: {}"
    fn state(&self) -> String {
        format!("Generation: {} | Alive: {} | Dead: {}", self.generation, self.alive, self.dead)
    }

    /// A method that returns the name of the automaton as a string.
    /// Format: "Conway's Game of Life | Grid | {}"
    fn name(&self) -> String {
        format!("Conway's Game of Life | Grid | {}", self.initialstate)
    }
}

// Implementation of helper methods for GameOfLife with a CellGrid grid,
impl GameOfLife<CellGrid<BinaryCell>> {
    // A function that retrieves the number of alive cells in 
    // the neighbouring vicity of a given cell (x, y)
    fn scan_vicinity(&mut self, x: usize, y: usize) -> i32 {
        // Declare a counter
        let mut count = 0;
        
        // Check if the cell grid exists
        if let Some(grid) = &self.grid.vector {

            // Iterate over the cells in the vicinity of the cell at (x, y).
            // The [-1,0,1] vectors represent the vicinity offsets for the x and y axis each.
            for x_off in vec![-1, 0, 1] {
                for y_off in vec![-1, 0, 1] {
                    // Create the position of the cell in the 
                    // grid based on the vicinity offsets
                    let nx = x as i32 + x_off;
                    let ny = y as i32 + y_off;
                    
                    // Check if position is out of grid bounds (x axis)
                    if nx < 0 || nx >= grid.len() as i32 {
                        continue;
                    }

                    // Check if position is out of grid bounds (y axis)
                    if ny < 0 || ny >= grid[nx as usize].len() as i32 {
                        continue;
                    }

                    // Check if position points to the cell itself i.e (0,0) offsets
                    if nx == x as i32 && ny == y as i32 {
                        continue;
                    }

                    // Check if the cell if alive
                    match grid[nx as usize][ny as usize].clone() {
                        // Increment the counter if the cell is alive
                        BinaryCell::Active => count = count+1,
                        _ => continue,
                    }
                }
            }
        }

        // Return the counter value
        return count
    }
}


// Implementation of the Drawable trait for GameOfLife with a CellGrid grid,
impl graphics::Drawable for GameOfLife<CellGrid<BinaryCell>> {

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
    fn draw(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> GameResult<()> {
        
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

        // Build and Draw the mesh
        mb.build(ctx)?.draw(ctx, param)?;

        // Declare a variable for the font size
        let font_size = 18.0;

        // Create the text graphics for the banner
        let mut name_text = graphics::Text::new(self.name());
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