use resource::loading::get_resource_path_by_name;
use resource::ResourceKind;
use std::path::PathBuf;

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
fn test_loading() {
    let correct_path = PathBuf::from("./data/engine_config.lua");
    let result = match get_resource_path_by_name(ResourceKind::EngineConfig, "Doesn't matter~~") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
    assert_eq!(correct_path, result);
}
