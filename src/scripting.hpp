#pragma once
#include <iostream>
#include "debug.hpp"
#include <SFML/System.hpp>

extern "C" {
#include "lua/lua.h"
#include "lua/lualib.h"
#include "lua/lauxlib.h"
}

// Functions

/*
 * scripting_report_errors
 * Report errors in the Lua environment
 */
void scripting_report_errors(lua_State*, int);

/*
 * scripting_init
 * Set up the scripting environment, load lua libraries, et cetera
 * This should be called from main()
 */
lua_State* scripting_init();

/*
 * scripting_dofile
 * Run a Lua script and report errors
 */
void scripting_dofile(lua_State*, sf::String);

/*
 * scripting_get_global_int
 * Get a Lua global by name, returning an int
 */
int scripting_get_global_int (lua_State*, sf::String);

/*
 * scripting_get_global_string
 * Get a Lua global by name, returning a sf::String
 */
sf::String scripting_get_global_string (lua_State*, sf::String);

/*
 * scripting_register_loading_functions
 * Register functions for creating characters, maps, et cetera from Lua
 *  scripts.
 */
void scripting_register_loading_functions(lua_State*);


