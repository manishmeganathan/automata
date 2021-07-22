#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ggez::{conf, event};
use automata::simulation::sim::Simulation;

fn main() -> ggez::GameResult {
    // Setup WindowMode and WindowSetup for ggez
    let w_mode: conf::WindowMode = conf::WindowMode::default().dimensions(700.0, 700.0);
    let w_setup: conf::WindowSetup = conf::WindowSetup::default().title("Conway's Game Of Life");

    // Create a ggez context with the window mode and window setup
    let cb = ggez::ContextBuilder::new("game_of_life", "Manish Meganathan")
        .window_mode(w_mode)
        .window_setup(w_setup);

    // Build the context and event loop
    let (ctx, event_loop) = &mut cb.build()?;

    // Create the simulation
    let sim = &mut Simulation::<automata::gameoflife::grid::Grid>::new(10.0, 0)?;

    // Start the event loop with the simulation
    event::run(ctx, event_loop, sim)
}
