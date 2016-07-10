use resource::loading::*;
use resource::ResourceKind;
use std::path::PathBuf;

use std::io::prelude::*;
use std::fs::File;

#[test]
// Make sure that get_resource_path_by_name can properly generate the names for maps
fn test_map_path() {
    let correct_path = PathBuf::from("./data/maps/Map001/");
    let result = match get_resource_path_by_name(ResourceKind::Map, "Map001") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
    assert_eq!(correct_path, result);
}

#[test]
// Make sure that get_resource_path_by_name can properly generate the name of the engine config
fn test_engine_config_path() {
    let correct_path = PathBuf::from("./data/engine_config.lua");
    let result = match get_resource_path_by_name(ResourceKind::EngineConfig, "Doesn't matter~~") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
    assert_eq!(correct_path, result);
}

#[test]
// Make sure that get_resource_reader_by_name is able to acquire readers for resources
fn test_engine_config_reader() {
    let mut correct_content = String::new();
    let mut test_content = String::new();

    // Read it in "manually"
    File::open(get_resource_path_by_name(ResourceKind::EngineConfig, "").unwrap())
        .unwrap()
        .read_to_string(&mut correct_content)
        .unwrap();

    // Read it in with get_resource_reader_by_name
    get_resource_reader_by_name(ResourceKind::EngineConfig, "")
        .unwrap()
        .read_to_string(&mut test_content)
        .unwrap();

    assert_eq!(correct_content, test_content);
}
