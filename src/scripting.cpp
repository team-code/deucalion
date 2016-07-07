#include "scripting.hpp"

void scripting_report_errors(lua_State *L, int status)
{
  if ( status!=0 ) {
    std::cerr << "LUA ERROR: " << lua_tostring(L, -1) << std::endl;
    lua_pop(L, 1); // remove error message
  }
}

void scripting_dofile(lua_State* L, sf::String filename)
{
    scripting_report_errors(L, luaL_dofile(L, filename.toAnsiString().c_str()));
}

int scripting_get_global_int (lua_State* L, sf::String name)
{
    lua_getglobal(L, name.toAnsiString().c_str());
    return lua_tointeger(L, -1);
}

sf::String scripting_get_global_string (lua_State* L, sf::String name)
{
    lua_getglobal(L, name.toAnsiString().c_str());
    return sf::String(lua_tostring(L, -1));
}

//void scripting_register_function

lua_State* scripting_init()
{
    debug("Lua subsystem is being init'd.");
    // Init the Lua interpreter
    lua_State *lua_state = luaL_newstate();

    debug("Init Lua libraries...");
    // load Lua libraries
    luaL_openlibs(lua_state);

    luaL_dostring(lua_state, "print(\"Attempting to print from within Lua!\")");

    debug("Lua subsystem fully initialized - ready to execute engine functions.");

    return lua_state;
}

void scripting_register_loading_functions(lua_State* L)
{
    
}