extern crate sfml;
#[macro_use]
extern crate log;
extern crate env_logger;

use sfml::window::{ContextSettings, VideoMode, window_style};
use sfml::window::event;
use sfml::graphics::RenderWindow;
use sfml::system::Clock;

mod engine_config;

fn fake_main() -> i32 {
    // Init'ing the log system is the first thing to try. Without it, nothing else
    // can be done or reported, so unwrap() will be used here and ONLY here.
    env_logger::init().unwrap();
    debug!("env_logger has been initialized successfully.");

    // TODO: Init Lua here.

    // Acquire the engine configuration. Failing this is a fatal error.
    let engine_config = match engine_config::get_engine_config() {
        Ok(config) => config,
        Err(error) => {
            error!("Failed to acquire engine configuration:\n\t{}", error);
            return 1;
        }
    };

    // TODO: Init the game state here.

    // Initialize the game window. If this can't be done, there's really no point in
    // continuing on.
    let mut window = match RenderWindow::new(VideoMode::new_init(engine_config.screen_width,
                                                                 engine_config.screen_height,
                                                                 32),
                                             "Game Title",
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
                window.close()
            }
        }
    }
    return 0;
}

fn main() {
    // Fake-main technique. This allows the main() function to return an int (i32) status code.
    std::process::exit(fake_main());
}
