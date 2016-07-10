//! Define the custom error type used for Deucalion
//! Writing this is kind of annoying but it lets try! work in the code, which is very useful

use std::error::Error;
use std::fmt;
use std::io;
use hlua;
use tiled;

#[derive(Debug)]
pub enum DeucalionError {
    /// An error in I/O caused this error
    IoError(io::Error),
    /// An error in a script caused this error
    LuaError(hlua::LuaError),
    /// A problem with tilemaps caused this error
    TiledError(tiled::TiledError),
    /// Some functionality that is not yet implemented was called, causing this error.
    /// Note that there is not ::from that creates this error; it must be created explicitly.
    NotImplementedError(String),
    /// Something else bad happened.
    OtherError(String),
}

impl From<io::Error> for DeucalionError {
    fn from(err: io::Error) -> DeucalionError {
        DeucalionError::IoError(err)
    }
}

impl From<hlua::LuaError> for DeucalionError {
    fn from(err: hlua::LuaError) -> DeucalionError {
        DeucalionError::LuaError(err)
    }
}

impl From<tiled::TiledError> for DeucalionError {
    fn from(err: tiled::TiledError) -> DeucalionError {
        DeucalionError::TiledError(err)
    }
}

// String and &str -> OtherError Froms allow errors not covered by other types to be expressed
impl From<String> for DeucalionError {
    fn from(description: String) -> DeucalionError {
        DeucalionError::OtherError(description)
    }
}

impl<'a> From<&'a str> for DeucalionError {
    fn from(description: &str) -> DeucalionError {
        DeucalionError::OtherError(String::from(description))
    }
}

impl Error for DeucalionError {
    fn description(&self) -> &str {
        match *self {
            DeucalionError::IoError(ref err) => err.description(),
            DeucalionError::LuaError(_) => "There was an error in Lua code.",
            DeucalionError::TiledError(ref err) => err.description(),
            DeucalionError::NotImplementedError(ref string) => string.as_ref(),
            DeucalionError::OtherError(ref string) => string.as_ref(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            DeucalionError::IoError(ref err) => Some(err as &Error),
            // LuaError doesn't currently implement Error. If it ever does,
            //  this can be changed to be more useful.
            DeucalionError::LuaError(_) => None,
            // TiledError currently doesn't implement Error.
            DeucalionError::TiledError(ref err) => Some(err as &Error),
            DeucalionError::NotImplementedError(_) => None,
            DeucalionError::OtherError(_) => None,
        }
    }
}

impl fmt::Display for DeucalionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeucalionError::IoError(ref err) => fmt::Display::fmt(err, f),
            // Currently, LuaError doesn't implement Display. If it ever does,
            //  this can be changed in order to be more useful
            DeucalionError::LuaError(_) => Err(fmt::Error),
            DeucalionError::TiledError(ref err) => fmt::Display::fmt(err, f),
            DeucalionError::NotImplementedError(ref string) => fmt::Display::fmt(string, f),
            DeucalionError::OtherError(ref string) => fmt::Display::fmt(string, f),
        }
    }
}
