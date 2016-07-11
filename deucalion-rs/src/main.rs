#![allow(dead_code)]

extern crate sfml;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hlua;
extern crate tiled;

use sfml::window::{ContextSettings, VideoMode, window_style};
use sfml::window::event;
use sfml::graphics::RenderWindow;
use sfml::system::Clock;

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
    let mut window = match RenderWindow::new(VideoMode::new_init(engine_config.screen_width,
                                                                 engine_config.screen_height,
                                                                 32),
                                             &game_config.title,
                                             window_style::CLOSE,
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => {
            error!("Could not init a RenderWindow. There is likely a problem with your system.");
            return 1;
        }
    };
    // Set the game's maximum framerate.
    window.set_framerate_limit(engine_config.maximum_framerate);

    // Set up a clock to keep track of how long frames take.
    let frame_clock = Clock::new();
    while window.is_open() {
        // events() returns an iterator over all events in the queue.
        for current_event in window.events() {
            if current_event == event::Closed {
                window.close();
            }
        }

        // TODO: Change the world's state here.


    }
    return 0;
}

fn main() {
    // Fake-main technique. This allows the main() function to return an int (i32) status code.
    std::process::exit(fake_main());
}
