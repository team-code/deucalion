//! Basic operations on the Lua context, such as creation, destruction, etc
use hlua::Lua;
use hlua::AnyLuaValue;
use error::DeucalionError;

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

/// Execute a script from a file, returning whatever that script returns. If no data is expected,
/// that return value can be safely ignored.
pub fn execute_script(
    environment: &mut Lua,
    filename: &str,
) -> Result<AnyLuaValue, DeucalionError> {
    use std::fs::File;
    use std::io::prelude::*;
    // Open the file
    let mut f = try!(File::open(filename));
    // With the file open, read its contents into a String
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    // Try to execute the contents of the file in the given scripting environment
    // Implementation note: AnyLuaValue typearg here means that the return value will be
    //  AnyLuaValue as well, which is what we want.
    match environment.execute::<AnyLuaValue>(&contents) {
        Ok(v) => Ok(v),
        Err(e) => Err(DeucalionError::from(e)),
    }
}
