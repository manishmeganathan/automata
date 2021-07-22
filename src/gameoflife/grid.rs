use crate::simulation::sim::SimGrid;
use crate::gameoflife::cell::Cell;
use crate::gameoflife::iter::{GridIteratorItem ,GridIterator};

use ggez::graphics;

// A struct that represents the cell grid
#[derive(Debug)]
pub struct Grid {
    // Represents the 2D vector of grid cells
    cellgrid: Option<Vec<Vec<Cell>>>,
    // Represents the 2D rectangular bounds of the grid
    dimensions: Option<graphics::Rect>,
    // Represents the size of a single cell
    cellsize: f32,
    // Represents the number of times the grid has been updated
    generation: u32,
    // Represents the number of cells that are alive
    alive: u32,
    // Represents the number of cells that are dead
    dead: u32,

}

// Implementation of SimGrid trait for Grid
impl SimGrid for Grid {
    // A constructor function that creates a  
    // null grid for a given cell size in pixels
    fn new(cellsize: f32) -> Self {
        Self {
            cellgrid: None,
            cellsize,
            dimensions: None,
            generation: 0,
            alive: 0,
            dead: 0,
        }
    }

    // A method of Grid that initializes the grid for a given rectangle dimensions.
    fn initialize(&mut self, dimensions: graphics::Rect) {
        // Calculate the number of rows and columns in the grid
        let rows = dimensions.w / self.cellsize;
        let cols = dimensions.h / self.cellsize;

        // Create a new vector (represents rows)
        let mut grid = Vec::new();

        // Iterate for each row
        for _ in 0..(rows as i32) {
            // Create a new vector (represents columns)
            let mut column = Vec::new();
            // Iterate for each column
            for _ in 0..(cols as i32) {
                // Create a new cell and push it into the column vector
                column.push(Cell::new());
            }

            // Push the column vector into the row vector (grid)
            grid.push(column);
        }

        // Assign the initialized grid vector to the grid struct
        self.cellgrid = Some(grid);
        // Assign the dimensions to the grid struct
        self.dimensions = Some(dimensions);
    }

    // A method of Grid that updates the grid state based on the rules of Conway's Game of Life.
    //
    // Automaton Rules:
    // - Any live cell with fewer than two live neighbours dies, as if caused by under-population.
    // - Any live cell with two or three live neighbours lives on to the next generation.
    // - Any live cell with more than three live neighbours dies, as if by overcrowding.
    // - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    // - All other states are propogated to the next generation.
    fn update(&mut self) {
        // Declare counter variables for the number of alive and dead cells
        let mut alive: u32 = 0;
        let mut dead: u32 = 0;

        // Check if the cell grid exists
        if self.cellgrid.is_some() {
            // Create a clone of the cell grid
            let mut newgrid = self.cellgrid.clone().unwrap();

            // Iterate over the grid
            for (x, y, cell) in self.clone() {

                // Check the vicinity of the cell
                let cell = match (cell, self.scan_vicinity(x, y)) {
                    // If a cell is alive, and there are either too many live  
                    // neighbors or not enough live neighbors, kill it.
                    (Cell::Alive, n) if n < 2 || n > 3 => Cell::Dead,

                    // If a cell is alive and has either 2 
                    // or 3 live neighbors, keep it alive
                    (Cell::Alive, n) if n == 3 || n == 2 => Cell::Alive,

                    // If a cell is dead and has exactly 3 live neighbors, revive it
                    (Cell::Dead, 3) => Cell::Alive,

                    // Otherwise, keep the cell state
                    (c, _) => c,
                };

                // Add the new cell to the new grid
                newgrid[x][y] = cell.clone();

                // Increment the alive or dead counter
                match cell {
                    Cell::Dead => dead += 1,
                    Cell::Alive => alive += 1
                }
            }

            // Assign the new grid to the grid struct
            self.cellgrid = Some(newgrid)
        }
        
        // Update the alive and dead cell value in the grid struct
        self.alive = alive;
        self.dead = dead;
        // Increment the generation value in the grid struct
        self.generation += 1;
    }
}

// Implementation of helper methods for Grid
impl Grid {
    // A function that retrieves the number of alive cells in 
    // the neighbouring vicity of a given cell (x, y)
    fn scan_vicinity(&mut self, x: usize, y: usize) -> i32 {
        // Declare a counter
        let mut count = 0;

        // Check if the cell grid exists
        if let Some(grid) = &self.cellgrid {

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
                        Cell::Alive => count = count+1,
                        _ => continue,
                    }
                }
            }
        }

        // Return the counter value
        return count
    }
}

// Implementation of the Clone trait for Grid
impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
            cellgrid: self.cellgrid.clone(),
            dimensions: self.dimensions.clone(),
            cellsize: self.cellsize,
            generation: self.generation,
            alive: self.alive,
            dead: self.dead,
        }
    }
}

// Implementation of the IntoIterator trait for Grid
impl IntoIterator for Grid {
    // Defines the iteration item
    type Item = GridIteratorItem;
    // Defines the iteration iterator
    type IntoIter = GridIterator;

    // Defines the iterator's initial state
    fn into_iter(self) -> Self::IntoIter {
        // Check the option value
        match self.cellgrid {
            // Panic if GetOption::None is returned
            None => panic!("could not create grid iterator!"),
            // Create GridIterator with the grid
            Some(grid) => {
                GridIterator {
                    grid: grid,
                    current_column: 0,
                    current_row: 0,
                }
            }
        }
    }
}

// Implementation of graphics for Grid
impl graphics::Drawable for Grid {

    // A method that returns the dimensions of the grid
    fn dimensions(&self, _ctx: &mut ggez::Context) -> Option<graphics::Rect> {
        self.dimensions
    }

    // A method that returns the graphics blending mode of the grid
    fn blend_mode(&self) -> Option<graphics::BlendMode> {
        Some(graphics::BlendMode::Add)
    }

    // A method that set the graphics blend mode of the grid (currently does nothing)
    fn set_blend_mode(&mut self, _: Option<graphics::BlendMode>) {}

    // A method that draws the grid and returns a GameResult
    fn draw(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult<()> {
        // Check if cell grid exists
        if self.cellgrid.is_some() {
            // Create a new graphic mesh builder
            let mut mb = graphics::MeshBuilder::new();

            // Iterate through each cell in the grid
            for (x, y, cell) in self.clone() {

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
                        Cell::Dead => [0.0, 0.0, 0.0, 1.0].into(),
                        Cell::Alive => [1.0, 1.0, 1.0, 1.0].into(),
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
        }

        // Return GameResult::Ok
        Ok(())
    }
}