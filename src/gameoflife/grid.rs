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
    size: f32,
}

// Constructor implemntations for Grid
impl Grid {
    // A constructor function that creates a  
    // null grid for a given cell size in pixels
    pub fn new(size: f32) -> Self {
        Self {
            cellgrid: None,
            dimensions: None,
            size,
        }
    }

    // A method of Grid that initializes the grid for a given rectangle dimensions.
    pub fn initialize(&mut self, dimensions: graphics::Rect) {
        // Calculate the number of rows and columns in the grid
        let rows = dimensions.w / self.size;
        let cols = dimensions.h / self.size;

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
}

// Implementation of the Clone trait for Grid
impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
            cellgrid: self.cellgrid.clone(),
            dimensions: self.dimensions.clone(),
            size: self.size,
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
                    (x as f32) * self.size,
                    (y as f32) * self.size,
                    self.size,
                    self.size,
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