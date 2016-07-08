//! Functions for managing and acquiring engine configuration

// TODO: Make this work with the Lua subsystem.

/// A datastructure containing configuration details for the engine
#[derive(PartialEq, Eq, Debug)]
pub struct EngineConfig {
    /// The width of the window in which the engine will run.
    pub screen_width: u32,
    /// The height of the window in which the engine will run.
    pub screen_height: u32,
    /// The maximum framerate at which the engine should attempt to run.
    pub maximum_framerate: u32,
}

/// Acquire the engine's configuration. Currently this is arbitrary defaults; in
/// the future, it will be from a Lua script.
pub fn get_engine_config() -> Result<EngineConfig, String> {
    Ok(EngineConfig {
        screen_width: 640,
        screen_height: 480,
        maximum_framerate: 60,
    })
}
