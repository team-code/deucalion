# Guidelines

## What is this document?
This is a set of guidelines for the deucalion-rs project. Any deviation from
these guidelines should be explained in comments.

## Datatypes
### Integers
Signed and unsigned integers should be `isize` or `usize`, respectively, except
when they are meant to be passed to API functions that require specific
types. In those cases, the reason for the deviation should be documented.
### Errors
`error.rs` defines DeucalionError. When a function might encounter an error, it
should return a Result<T, DeucalionError>. If DeucalionError doesn't have the
type of error available, you should implement it or use OtherError
(by DeucalionError::from("string literal") or DeucalionError::from(string)).

## Logging Practices
deucalion-rs uses the Rust `log` logging abstraction. The macros `error!()`,
`warning!()`, etc should be used to log messages.
### Logging Levels
* The _error_ logging level is for information the player needs to be aware of -
  for example, why the game crashed, or why a certain savefile can't be opened.
* The _warning_ logging level is for information the developer of a game using
  the engine needs to be aware of - for example, why a Lua script can't be run,
  or that an Info whose value is `nil` has been retrieved.
* The _info_ logging level is for information that a game developer needs in
  order to effectively debug their game - for example, that a map or resource
  was loaded, or that a Lua script was executed.
* The _debug_ logging level is for information about the internal operation
  of the engine that no game developer needs - it is for engine development
  and debugging only. For example, the debug log might include
  messages on every registered keypress.
* The _trace_ logging level is for extremely detailed information about the
  internal operation of the engine: for example, "Drew a tile at {x},{y}",
  or "Drew sprite {name}". This is generally useful only for
  debugging thorny engine logic issues, and therefore is allowed to do
  expensive computations.
