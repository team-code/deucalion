# Guidelines

## What is this document?
This is a set of guidelines for the deucalion-rs project. Any deviation from
these guidelines should be explained in comments.

## Datatypes
### Integers
Signed and unsigned integers should be `isize` or `usize`, respectively, except
when they are meant to be passed to API functions that require specific
types. In those cases, the reason for the deviation should be documented.

## Logging Practices
deucalion-rs uses the Rust `log` logging abstraction. The macros `error!()`,
`warning!()`, etc should be used to log messages.
