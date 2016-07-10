# Data Directory Blueprint

The data directory is divided into various subdirectories, each one of which
contains a specific type of resource.

Typically, a resource that might have more than one file has a directory with
its name.

## Notation

In this document, `<name>` denotes the name of the resource; for example, a
character's info is stored at `data/characters/<name>/info.lua`. Given a
character named Joe, this path means `data/characters/Joe/info.lua`.

Similarly, `<event>` denotes an event; for example, `on_<event>.lua` means all
of `on_interact.lua`, `on_collide.lua`, et cetera.

`<ext>` means that multiple filetypes are supported.

## Structure

```
data/
    engine_config.lua # Contains the current configuration of the engine
    characters/
        <name>/
            info.lua # Contains the character's info: name, description, etc.
            spritesheet.<ext> # The visual representation of the character
            on_<event>.lua # Contains the code run on <event>
    maps/
        <name>/
            <name>.tmx # Contains the Base64 encoded, GZIPped map data.
            ... # Other files here are generally tilemaps
    music/
        <name>.<ext> # Music sound file
    sound_fx/
        <name>.<ext> # FX sound file
```
