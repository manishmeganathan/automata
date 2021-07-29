# automata
### A 2D Simulator in Rust for cellular automata such as Conway's Game of Life and Langton's Ant among others. Powered by the ggez framework.

## Disclaimer
This project is still under development and not all features are functional. Contributions towards expanding features and improving performance are welcome.

## Installation

### From Release
1. Download the latest release for the target platform.
2. Add the path to the binary to the system PATH environment variable.
3. Application can be invoked using ``automata`` in the shell.

### From Source
1. Install Git, Rust and Cargo on target system.
2. Clone this repository & change directory into it.
```
git clone https://github.com/manishmeganathan/automata.git 
cd automata
```
3. Build the project
```
cargo build --release
```
4. Application can be run using ``cargo run ..`` or the built binary can be invoked directly.

## Usage
The Simulator currently support two automata. **Conway's Game of Life** and **Langton's Ant**. The simulator can be configured with flags that adjust the grid dimension, individual cell dimensions, simulation FPS, etc.

### Example
Run the following to start the **Conway's Game Of Life** automata.
```
automata gameoflife
```

Run the following to start **Langton's Ant** with a grid of 1000x1000 pixels
```
automata langtonsant --grid 1000x1000
```

Supported flags include
- **--grid [-g]** Set dimensions of the grid. Takes the format ``widthXheight``.
- **--cell [-c]** Set the cell size of the grid. Takes an int that represents the size of the cell
- **--fps [-f]** Set the simulation fps.

Currently supported automata
- **Conway's Game Of Life** - ``gameoflife``
- **Langton's Ant** - ``langtonsant``

## Project Structure
### Simulation
This module implements the event handler and simulable trait definitions.
- ``SimCell`` represents a trait implemented by cell structs that can be simulated.
- ``SimGrid`` represents a trait implemented by grid structs that can be simulated. 
- ``Automaton`` represents a trait implemented by any automaton that can be simulated.
- ``Simulation`` represents a struct that contains the simulation runtime handlers.


### Commons
This module implements common tools used on different automata such as different types of cells, grids and turmites.
- ``BinaryCell`` represents a cell state enum for cells that are either on or off.
- ``CellGrid`` represents a grid of square cells.
- ``GridCell`` represents an arbitrary cell on a grid.
- ``GridScanner`` represents an iterator that scans over the entire grid.
- ``Orient`` represents a trait implemented by directional structures.
- ``Direction4`` represents a enum that has variation for different cardinal directions.
- ``Turmite`` represents a turmite on a grid that crawl/travel around.

Notes:
- All Cells implement the ``SimCell`` trait.
- All Grids implement the ``SimGrid`` trait.
- Future grid implementations can include hexagonal and octagonal grids which will use 6 directional and 8 directional orientations.

### GameOfLife
This module implements a struct of the same name generic over different types of grids.
- Currently the initial state of the automata is a randomly generated balance grid. Future implementation can include states such as Gosper's Glider Gun.
- Currently only implemented for square grids.

### LangtonsAnt
This module implements a struct of the same name generic over different types of grids.
- Currently the inital state of the automata is an empty grid. Future implementations can include randomized grids.
- Currently only implemented for square grids.
- Currently only supports 1 randomly placed ant. Future implementations can support predetermined ant positions and multiple ants.

