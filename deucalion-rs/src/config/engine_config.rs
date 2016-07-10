//! Functions for managing and acquiring engine configuration
use error::DeucalionError;
use scripting::{execute_script, Lua};
use resource;
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

/// Acquire the engine's configuration. If acquiring it from data/engine_config.lua fails,
/// this function will return the default configuration, which will typically work for every
/// configuration.
pub fn get_engine_config(environment: &mut Lua) -> EngineConfig {
    let path =
        match resource::loading::get_resource_path_by_name(resource::ResourceKind::EngineConfig,
                                                           "") {
            Ok(v) => v,
            Err(e) => {
                // If the engine config's path can't be generated, it certainly can't be loaded.
                error!("Failed to generate the path for the engine configuration script: {}",
                       e);
                return get_default_engine_config();
            }
        };

    // Syntax monster explaination for the &* in the second argument to execute_script:
    //  to_string_lossy returns a Cow<str>, which we dereference to str and then borrow to &str
    match execute_script(environment, &*path.to_string_lossy()) {
        Ok(_) => {
            match get_engine_config_from_environment(environment) {
                Ok(w) => {
                    info!("Succesfully acquired engine config at {}",
                          path.to_string_lossy());
                    trace!("Engine config is {:?}", w);
                    w // This is the valid EngineConfig struct built from the script
                }
                Err(e) => {
                    error!("Failed to acquire engine config from script at {}: {}",
                           path.to_string_lossy(),
                           e);
                    get_default_engine_config()
                }
            }
        }
        Err(e) => {
            error!("Failed to run engine config script at {}: {}",
                   path.to_string_lossy(),
                   e);
            get_default_engine_config()
        }
    }
}

/// Check variables in the Lua environment, bringing their values into an EngineConfig struct
fn get_engine_config_from_environment(environment: &mut Lua) -> Result<EngineConfig, DeucalionError> {
    // Here, we have a set of match expressions that attempt to fetch the global config variables.
    // At some point, this action (extract Lua global or error) should be abstracted into a macro
    let screen_width = match environment.get("SCREEN_WIDTH") {
        Some(v) => v,
        None => {
            return Err(DeucalionError::from("SCREEN_WIDTH is not defined or is the wrong type"));
        }
    };
    let screen_height = match environment.get("SCREEN_HEIGHT") {
        Some(v) => v,
        None => {
            return Err(DeucalionError::from("SCREEN_HEIGHT is not defined or is the wrong type"));
        }
    };
    let maximum_framerate = match environment.get("MAXIMUM_FRAMERATE") {
        Some(v) => v,
        None => {
            return Err(DeucalionError::from("MAXIMUM_FRAMERATE is not defined or is the wrong \
                                             type"));
        }
    };
    // Simply build the EngineConfig struct. Making it to this point means the config is O.K.
    Ok(EngineConfig {
        screen_height: screen_height,
        screen_width: screen_width,
        maximum_framerate: maximum_framerate,
    })
}

/// Get the engine's default configuration state. This cannot fail.
fn get_default_engine_config() -> EngineConfig {
    EngineConfig {
        screen_width: 640,
        screen_height: 480,
        maximum_framerate: 60,
    }
}
