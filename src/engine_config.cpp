#include "engine_config.hpp"

struct engine_config_s get_engine_config(lua_State *L, sf::String filename){
    debug("Fetching engine config from %s", filename.toAnsiString().c_str());
    scripting_dofile(L, filename);
    
    struct engine_config_s engine_config;
    
    engine_config.screen_width = scripting_get_global_int(L, "SCREEN_WIDTH");
    engine_config.screen_height = scripting_get_global_int(L, "SCREEN_HEIGHT");
    engine_config.maximum_framerate = scripting_get_global_int(L, "MAXIMUM_FRAMERATE");
    
    return engine_config;
}