#pragma once
#include <SFML/System.hpp>
#include "scripting.hpp"

// The engine configuration is for settings not defined by the developer, such as 
//  resolution, framerate, et cetera

struct engine_config_s {
    int screen_width, screen_height;
    int maximum_framerate;
};

// Load the engine configuration from a Lua script
struct engine_config_s get_engine_config(lua_State*, sf::String);