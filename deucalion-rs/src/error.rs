//! Define the custom error type used for Deucalion
//! Writing this is kind of annoying but it lets try! work in the code, which is very useful

use std::error::Error;
use std::fmt;
use std::io;
use hlua;

#[derive(Debug)]
pub enum DeucalionError {
    IoError(io::Error),
    LuaError(hlua::LuaError),
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
            DeucalionError::OtherError(ref string) => string.as_ref(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            DeucalionError::IoError(ref err) => Some(err as &Error),
            // LuaError doesn't currently implement Error. If it ever does,
            //  this can be changed to be more useful.
            DeucalionError::LuaError(_) => None,
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
            DeucalionError::OtherError(ref string) => fmt::Display::fmt(string, f),
        }
    }
}
