//! Utilities for loading resources.

use std::path::{Path, PathBuf};
use resource::ResourceKind;
use error::DeucalionError;

use std::fs::File;

/// Get a reader that can read in the requested resource from disk.
/// If requesting a resource whose name doesn't change, the `name` argument is ignored.
pub fn get_resource_reader_by_name(kind: ResourceKind, name: &str) -> Result<File, DeucalionError> {
    // Acquire the path to the resource
    let resource_path = try!(get_resource_path_by_name(kind, name));
    let reader = try!(File::open(&resource_path));
    return Ok(reader);
}

/// Get the path to a resource in the data directory scheme. See BLUEPRINT.md for more info.
/// If requesting a resource whose name doesn't change, the `name` argument is ignored.
pub fn get_resource_path_by_name(kind: ResourceKind,
                                 name: &str)
                                 -> Result<PathBuf, DeucalionError> {
    // Everything is in the data directory.
    let mut path = Path::new(".").join("data");
    match kind {
        // maps are stored at data/maps/<name>/
        ResourceKind::Map => {
            // Select maps directory
            path.push("maps");
            // Select specific map directory
            path.push(name);
            // Select the file
            path.push(name);
            path.set_extension("tmx");
            // Done!
            Ok(path)
        }
        // Game and engine configurations are always in the same place
        ResourceKind::EngineConfig => Ok(path.join("engine_config.lua")),
        ResourceKind::GameConfig => Ok(path.join("game_config.lua")),
        _ => {
            Err(DeucalionError::NotImplementedError(String::from("Currently, this kind \
                                                                         of resource isn't \
                                                                         implemented.")))
        }
    }
}
