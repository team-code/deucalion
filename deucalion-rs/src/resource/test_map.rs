#[allow(unused_imports)]
use resource::map::*;

#[test]
// Try to load a map.
fn test_map_loading() {
    match get_map_by_name("map001") {
        Ok(_) => {
            // Success!
        }
        Err(e) => panic!("{}", e),
    }
}
