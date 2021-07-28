pub mod cellgrid;
pub mod scanner;

pub use cellgrid::CellGrid;
pub use scanner::GridScanner;

/// A type alias for a cell on the grid and its position in that grid
/// Represents the x, y position and the cell at that position on the grid
pub type GridCell<C> = (usize, usize, C);
