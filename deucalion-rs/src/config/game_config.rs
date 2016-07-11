//! Functions for managing and acquiring game configuration
use error::DeucalionError;
use scripting::{execute_script, Lua};
use resource;

/// A datastructure containing configuration details for the game
#[derive(PartialEq, Eq, Debug)]
pub struct GameConfig {
    /// The game's title
    pub title: String,
    /// The first map to load when starting the game
    pub starting_map: String,
}

/// Acquire the game's configuration. If acquiring it from data/game_config.lua fails,
/// this function will return the default configuration, which is for debugging.
pub fn get_game_config(environment: &mut Lua) -> GameConfig {
    let path =
        match resource::loading::get_resource_path_by_name(resource::ResourceKind::GameConfig,
                                                           "") {
            Ok(v) => v,
            Err(e) => {
                // If the game config's path can't be generated, it certainly can't be loaded.
                error!("Failed to generate the path for the game configuration script: {}",
                       e);
                return get_default_game_config();
            }
        };

    // Syntax monster explaination for the &* in the second argument to execute_script:
    //  to_string_lossy returns a Cow<str>, which we dereference to str and then borrow to &str
    match execute_script(environment, &*path.to_string_lossy()) {
        Ok(_) => {
            match get_game_config_from_environment(environment) {
                Ok(w) => {
                    info!("Succesfully acquired game config at {}",
                          path.to_string_lossy());
                    trace!("Game config is {:?}", w);
                    w // This is the valid EngineConfig struct built from the script
                }
                Err(e) => {
                    error!("Failed to acquire game config from script at {}: {}",
                           path.to_string_lossy(),
                           e);
                    get_default_game_config()
                }
            }
        }
        Err(e) => {
            error!("Failed to run game config script at {}: {}",
                   path.to_string_lossy(),
                   e);
            get_default_game_config()
        }
    }
}

/// Check variables in the Lua environment, bringing their values into a GameConfig struct
fn get_game_config_from_environment(environment: &mut Lua) -> Result<GameConfig, DeucalionError> {
    // Here, we have a set of match expressions that attempt to fetch the global config variables.
    // At some point, this action (extract Lua global or error) should be abstracted into a macro
    let title: String = match environment.get("TITLE") {
        Some(v) => v,
        None => {
            return Err(DeucalionError::from("TITLE is not defined or is the wrong type"));
        }
    };
    let starting_map: String = match environment.get("STARTING_MAP") {
        Some(v) => v,
        None => {
            return Err(DeucalionError::from("STARTING_MAP is not defined or is the wrong type"));
        }
    };
    // Simply build the EngineConfig struct. Making it to this point means the config is O.K.
    Ok(GameConfig {
        title: title,
        starting_map: starting_map,
    })
}

/// Get the engine's default configuration state. This cannot fail.
fn get_default_game_config() -> GameConfig {
    GameConfig {
        title: String::from("Untitled"),
        starting_map: String::from("map001"),
    }
}
