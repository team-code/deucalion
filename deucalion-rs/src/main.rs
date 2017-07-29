#![allow(dead_code)]

extern crate sfml;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hlua;
extern crate tiled;

use sfml::window::{ContextSettings, VideoMode};
use sfml::window::style as window_style;
use sfml::window::Event;
use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::graphics::Color;

mod error;

mod config;
mod utility;
mod scripting;
mod resource;

fn fake_main<'engine>() -> i32 {
    // Init'ing the log system is the first thing to try. Without it, nothing else
    // can be done or reported, so unwrap() will be used here.
    env_logger::init().unwrap();
    info!("env_logger has been initialized successfully.");

    // Init the scripting subsystem
    let mut engine_scripting_environment = scripting::get_scripting_environment();

    // Acquire the engine configuration.
    let engine_config = config::engine_config::get_engine_config(&mut engine_scripting_environment);

    // Acquire the game's configuration.
    let game_config = config::game_config::get_game_config(&mut engine_scripting_environment);

    // Initialize the game window. If this can't be done, there's really no point in
    // continuing on.
    let mut window = match RenderWindow::new(
        VideoMode::new(engine_config.screen_width, engine_config.screen_height, 32),
        &game_config.title,
        window_style::CLOSE,
        &ContextSettings::default(),
    ) {
        Some(window) => window,
        None => {
            error!("Could not init a RenderWindow. There is likely a problem with your system.");
            return 1;
        }
    };
    // Set the game's maximum framerate.
    window.set_framerate_limit(engine_config.maximum_framerate);

    // Load the initial map into memory. This should go somewhere else later, but that design work
    // has not yet been done.
    let current_map = resource::map::get_map_by_name(&game_config.starting_map);

    while window.is_open() {
        // poll_event() returns Some(e) if there's an event to look at

        while let Some(current_event) = window.poll_event() {
            if current_event == Event::Closed {
                window.close();
            }
        }

        // Clear the window to ready it for rendering
        window.clear(&Color::black());

        // TODO: Change the world's state here.

        // Present the new frame to the user
        window.display();
    }
    return 0;
}

fn main() {
    // Fake-main technique. This allows the main() function to return an int (i32) status code.
    std::process::exit(fake_main());
}
