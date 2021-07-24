#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ggez::{conf, event};
use clap::{Arg, App};
use automata::simulation::sim::Simulation;

// fn main() -> ggez::GameResult {
//     // Create ggez WindowMode. Height will be 50 pixels great than width (grid state display)
//     let w_mode: conf::WindowMode = conf::WindowMode::default().dimensions(1200.0, 600.0);
//     // Create ggez WindowSetup
//     let w_setup: conf::WindowSetup = conf::WindowSetup::default().title("Conway's Game Of Life");

//     // Create a ggez context with the window mode and window setup
//     let cb = ggez::ContextBuilder::new("game_of_life", "Manish Meganathan")
//         .window_mode(w_mode)
//         .window_setup(w_setup);

//     // Build the context and event loop
//     let (ctx, event_loop) = &mut cb.build()?;

//     // Create the simulation
//     let sim = &mut Simulation::<automata::gameoflife::grid::Grid>::new(10.0, 0)?;

//     // Start the event loop with the simulation
//     event::run(ctx, event_loop, sim)
// }



fn main() -> ggez::GameResult {
    let version = "0.2.0";
    let author = "Manish Meganathan <meganthan.manish@gmail.com>";

    let matches = App::new("Automata")
        .version(version)
        .author(author)
        .about("Cellular Automata Simulator")

        .arg(Arg::with_name("GRID")
            .short("g")    
            .long("grid")
            .help("Set the grid size").default_value("600x600"))

        .arg(Arg::with_name("CELL")
            .short("c")
            .long("cell")
            .help("Set the cell size").default_value("10"))

        .arg(Arg::with_name("FPS")
            .short("f")
            .long("fps")
            .help("Set the simulation FPS").default_value("0"))

        .arg(Arg::with_name("AUTOMATON")
            .help("Cellular Automaton to Run. Ex. 'gameoflife', 'langstonant', etc. Refer to README for all options.")
            .required(true)
            .index(1))

        .get_matches();

    

    let grid_w;
    let grid_h;
    let cell_size;
    let fps;

    match matches.value_of("GRID") {
        Some(grid) => {
            let dimensions = grid.split("x").collect::<Vec<&str>>();

            match dimensions.len() {
                2 => {
                    match dimensions[0].parse::<f32>() {
                        Err(_) => {
                            eprintln!("[error] invalid grid dimensions. width must be a float");
                            std::process::exit(0);
                        },
                        Ok(w) => grid_w = w
                    }
        
                    match dimensions[1].parse::<f32>() {
                        Err(_) => {
                            eprintln!("[error] invalid grid dimensions. height must be a float");
                            std::process::exit(0);
                        },
                        Ok(h) => grid_h = h + 50.0
                    }
                },
                _ => {
                    eprintln!("[error] invalid grid dimensions. must be in WIDTHxHEIGH format.");
                    std::process::exit(0);
                }
            }
        },
        None => {
            eprintln!("[error] missing grid dimensions.");
            std::process::exit(0);
        }
    }

    match matches.value_of("CELL") {
        Some(cell) => {
            match cell.parse::<f32>() {
                Err(_) => {
                    eprintln!("[error] invalid cell dimensions. cell size must be a float");
                    std::process::exit(0);
                },
                Ok(c) => cell_size = c
            }
        },
        None => {
            eprintln!("[error] missing cell dimensions.");
            std::process::exit(0);
        }
    }

    match matches.value_of("FPS") {
        Some(cell) => {
            match cell.parse::<u32>() {
                Err(_) => {
                    eprintln!("[error] invalid simulation FPS. fps must be an int");
                    std::process::exit(0);
                },
                Ok(f) => fps = f
            }
        },
        None => {
            eprintln!("[error] missing cell dimensions.");
            std::process::exit(0);
        }
    }


    // remove
    println!("{} x {}. {}px @ {} FPS", grid_w, grid_h, cell_size, fps);

    // Create ggez WindowMode.
    let w_mode: conf::WindowMode = conf::WindowMode::default().dimensions(grid_w, grid_h);
    


    


    match matches.value_of("AUTOMATON") {
        None => {
            println!("No Automaton specified");
            std::process::exit(0);
        },
        Some(name) => {
            match name {
                "gameoflife" => {
                    let automata_name = "Conway's Game of Life";

                    println!("Running {}", automata_name);

                    // Create ggez WindowSetup
                    let w_setup: conf::WindowSetup = conf::WindowSetup::default().title(automata_name);

                    // Create a ggez context with the window mode and window setup
                    let cb = ggez::ContextBuilder::new(name, author)
                        .window_mode(w_mode)
                        .window_setup(w_setup);

                    // Build the context and event loop
                    let (ctx, event_loop) = &mut cb.build()?;

                    // Create the simulation
                    let sim = &mut Simulation::<automata::gameoflife::grid::Grid>::new(cell_size, fps)?;

                    // Start the event loop with the simulation
                    event::run(ctx, event_loop, sim)
                },
                _ => {
                    println!("Invalid Automaton specified. {}", name);
                    std::process::exit(0);
                }
                
            }
        },
    }
}