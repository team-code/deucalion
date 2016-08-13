//! Utilities for loading and managing resources, including images, maps, and sounds.
pub mod map;

// Only ResourceKind is used from here; no need for the extra indirection.
mod resource_kind;
pub use resource::resource_kind::ResourceKind;

pub mod loading;
pub mod repository;
mod test_repository;

// Imports to run unit tests
mod test_map;
mod test_loading;
