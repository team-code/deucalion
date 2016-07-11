//! Datatype for representing different kinds of resource

/// A datatype that represents a kind of resource. Each kind has its own loading strategy, storage,
/// et cetera.
#[derive(Debug, Copy, Clone)]
pub enum ResourceKind {
    /// A Map is a tilemap and its associated data.
    Map,
    /// A Music - a sound asset meant to be streamed from disk.
    Music,
    /// A sound effect - a sound asset meant to be loaded into memory, for low-latency playback
    SoundFX,
    /// A character - the basic actors of a JRPG.
    Character,
    /// Engine configuration files
    EngineConfig,
    /// Game configuration
    GameConfig,
}
