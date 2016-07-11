//! Provides facilities for working with tilemaps

use error::DeucalionError;
use resource::loading;
use resource::ResourceKind;
use tiled;

/// Given a name, return the tiled::Map corresponding to it.
pub fn get_map_by_name(name: &str) -> Result<tiled::Map, DeucalionError> {
    // Get the reader for the map's file
    let map_file = try!(loading::get_resource_reader_by_name(ResourceKind::Map, name));
    let map = try!(tiled::parse(map_file));
    trace!("Successfully loaded a map {}", name);
    return Ok(map);
}
