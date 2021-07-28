#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::{Arg, App};

use automata::simulation::Automaton;
use automata::simulation::Simulation;

fn main() -> ggez::GameResult {
    // Set the version information
    let version = "0.5.0";
    // Set the author information
    let author = "Manish Meganathan <meganthan.manish@gmail.com>";

    // Create a clap App and parse the arguments
    let matches = App::new("Automata")
        .version(version)
        .author(author)
        .about("Cellular Automata Simulator")
        // Argument for grid size
        .arg(Arg::with_name("GRID")
            .short("g")    
            .long("grid")
            .help("Set the grid size").default_value("600x600"))
        // Argument for cell size
        .arg(Arg::with_name("CELL")
            .short("c")
            .long("cell")
            .help("Set the cell size").default_value("10"))
        // Argument for simulation FPS
        .arg(Arg::with_name("FPS")
            .short("f")
            .long("fps")
            .help("Set the simulation FPS").default_value("0"))
        .arg(Arg::with_name("VERSION")
            .short("v")
            .long("version")
            .help("Prints application version"))
        // Argument for automaton to run
        .arg(Arg::with_name("AUTOMATON")
            .help("Automaton to Run. Ex. 'gameoflife', 'langtonsant', etc. Refer to README for all options.")
            .required(true)
            .index(1))
        // Retrieve the argument matches
        .get_matches();

    // Check if the user has requested version information
    match matches.occurrences_of("VERSION") {
        // No occurrence of version flag
        0 => {},
        // Any arbitrary number of occurrences
        _ => {
            // Print the version information and exit
            println!("Automata v{}", version);
            std::process::exit(0);
        }
    }

    // Declare a bunch of variables
    let grid_w;
    let grid_h;
    let cell_size;
    let fps;

    // Check for grid size argument
    match matches.value_of("GRID") {
        // If grid dimensions are set
        Some(grid) => {
            // Split dimensions string
            let dimensions = grid.split("x").collect::<Vec<&str>>();
            // Check number of dimension values
            match dimensions.len() {
                // Only 2 values expected (width and height)
                2 => {
                    // Parse the width into a float
                    match dimensions[0].parse::<f32>() {
                        // If the parse fails, print an error and exit
                        Err(_) => {
                            eprintln!("[error] invalid grid dimensions. width must be a float");
                            std::process::exit(0);
                        },
                        // If it parses, set the width
                        Ok(w) => grid_w = w
                    }
                    
                    // Parse the height into a float
                    match dimensions[1].parse::<f32>() {
                        // If the parse fails, print an error and exit
                        Err(_) => {
                            eprintln!("[error] invalid grid dimensions. height must be a float");
                            std::process::exit(0);
                        },
                        // If it parses, set the height
                        Ok(h) => grid_h = h
                    }
                },
                // Invalid dimension format
                _ => {
                    // Print an error and exit
                    eprintln!("[error] invalid grid dimensions. must be in WIDTHxHEIGHT format.");
                    std::process::exit(0);
                }
            }
        },
        // If grid dimensions are not set
        None => {
            // Print an error and exit
            eprintln!("[error] missing grid dimensions.");
            std::process::exit(0);
        }
    }

    // Check for cell size argument
    match matches.value_of("CELL") {
        // If cell size is set
        Some(cell) => {
            // Parse the cell size into a float
            match cell.parse::<f32>() {
                // If the parse fails, print an error and exit
                Err(_) => {
                    eprintln!("[error] invalid cell dimensions. cell size must be a float");
                    std::process::exit(0);
                },
                // If it parses, set the cell size
                Ok(c) => cell_size = c
            }
        },
        // If cell size is not set
        None => {
            // Print an error and exit
            eprintln!("[error] missing cell dimensions.");
            std::process::exit(0);
        }
    }

    // Check for simulation FPS argument
    match matches.value_of("FPS") {
        // If FPS is set
        Some(cell) => {
            // Parse the FPS into an integer
            match cell.parse::<u32>() {
                // If the parse fails, print an error and exit
                Err(_) => {
                    eprintln!("[error] invalid simulation FPS. fps must be an int");
                    std::process::exit(0);
                },
                // If it parses, set the FPS
                Ok(f) => fps = f
            }
        },
        // If FPS is not set
        None => {
            // Print an error and exit
            eprintln!("[error] missing cell dimensions.");
            std::process::exit(0);
        }
    }
    
    // Automaton Entity Imports
    use automata::commons::grids::CellGrid;
    use automata::commons::cells::BinaryCell;

    // Check if an automaton has been specified and create the simulator grid for it
    match matches.value_of("AUTOMATON") {
        // No automaton specified
        None => {
            // Print an error and exit
            eprintln!("[error] no automaton specified.");
            std::process::exit(0);
        },
        // Some automaton specified. Check the value
        Some(name) => match name {
            // Conway's Game of Life
            "gameoflife" => {
                let sim = &mut Simulation::<automata::gameoflife::GameOfLife<CellGrid<BinaryCell>>>::new("default", cell_size, fps);
                rendersim(sim, grid_w, grid_h, cell_size, fps, &author)
            }, 
            // Langton's Ant
            "langtonsant" => {
                let sim = &mut Simulation::<automata::langtonsant::LangtonsAnt<CellGrid<BinaryCell>>>::new("default", cell_size, fps);
                rendersim(sim, grid_w, grid_h, cell_size, fps, &author)
            },  

            // Unsupported Automaton - Print an error and exit
            _ => {
                eprintln!("[error] invalid automaton specified.");
                std::process::exit(0);
            },
        },
    }
}

// A function that renders the simulation in a window
fn rendersim<T: Automaton>(automaton: &mut Result<Simulation<T>, ggez::GameError>, grid_w: f32, grid_h: f32, cell_size: f32, fps: u32, author: &str) -> ggez::GameResult {
    use ggez::{conf, event};

    // Check if the automaton has any errors
    match automaton {
        // Render the automaton
        Ok(simulation) => {
            // Get the name of the automaton
            let simname = simulation.automaton.name();
            // Create ggez WindowMode.
            let w_mode: conf::WindowMode = conf::WindowMode::default().dimensions(grid_w, grid_h + 60.0);
            // Create ggez Window with the automaton name
            let w_setup = conf::WindowSetup::default().title(simname.as_str());
            // Create a ggez context with the window mode and window setup
            let cb = ggez::ContextBuilder::new(simname.as_str(), author)
                .window_mode(w_mode)
                .window_setup(w_setup);

            // Print the simulation config
            println!("Running {} | {}x{} | {}px @ {} FPS", simname, grid_w, grid_h, cell_size, fps);
            // Build the context and event loop
            let (ctx, event_loop) = &mut cb.build()?;
            // Start the simulation event loop
            event::run(ctx, event_loop, simulation)
        },

        // Print an error and exit
        Err(err) => {
            eprintln!("[error] could not render simulation. {}", err);
            std::process::exit(0);
        }
    }        
}
