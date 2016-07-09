//! Basic operations on the Lua context, such as creation, destruction, etc
use hlua::Lua;

/// Do all required work to initialize a game's Lua context.
/// The Lua object returned by this function is fully ready to be used by the engine.
pub fn get_scripting_environment<'environment>() -> Lua<'environment> {
    // Create the new Lua context
    let mut context = Lua::new();
    trace!("get_scripting_environment created a Lua environment.");
    // Open Lua's libs.
    context.openlibs();
    trace!("get_scripting_environment opened the libraries on a new Lua environment.");
    // Done, return the created environment
    context
}
