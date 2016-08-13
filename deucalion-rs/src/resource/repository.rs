//! Provides the definition of the Repository trait for memory repositories which store
//! large objects for the lifecycle of the engine so that they don't have to be reacquired
//! The repository is designed to be relatively opaque to the programmer; it can be told to acquire
//! resources but not to free them, as the whole idea is to keep them available for the lifetime
//! of the repository.

pub use std::rc::Rc;

pub trait Repository {
    /// T represents the type actually stored by this repository.
    type T;
    /// I represents the type used to identify objects stored in the repository.
    /// For example, a string. This needs to be somehow directly mapped to the
    /// acquisition of objects; for example, a map repository could expect that
    /// maps were at "<somedir>/resources/maps/<identifier>.tmx"
    type I;

    /// E represents the error returned if a call fails.
    type E;

    /// Return a new repository that is completely empty.
    fn new() -> Self;

    /// Returns true if the repository has an object with the given ID in memory;
    /// otherwise returns false.
    fn is_available(&self, id: Self::I) -> bool;

    /// Loads the object referred to by the given ID into memory so it can later be gotten
    /// Returns an error upon failure, or () if successful.
    fn load_object(&mut self, id: Self::I) -> Result<(), Self::E>;

    /// Returns a reference to the object referred to by the identifier given.
    /// This function must try to acquire the resource, and block while doing so.
    fn get_object(&mut self, id: Self::I) -> Result<Rc<Self::T>, Self::E>;

    /// Returns a reference to the object referred to by the identifier given, if and only if
    /// that object has already been loaded into memory. Otherwise returns None.
    /// This function is advantageous in that it allows the caller to refrain from mutating the
    /// repository.
    fn get_object_if_available(&self, id: Self::I) -> Option<Rc<Self::T>>;
}
