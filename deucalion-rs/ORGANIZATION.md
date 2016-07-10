# Organization
This file is a place to document the structure of the deucalion-rs code.

## Basic Structure
This project follows the normal cargo structure.

## Module Structure
* `error` contains the implementation of DeucalionError, the unified error type
  for the entire engine.
* `utility` contains general-purpose utility datatypes that are likely to be
  used by many other modules. Dependencies there should be minimal.
* `scripting` contains all the components necessary for interacting with the
  Lua scripting subsystem.
* `config` contains functions for manipulating and retrieving configuration
  data, for both the engine and the game.
