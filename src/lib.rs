#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]

/// primitive types used by rmidirs, like 1 byte, 2 bytes, 3 bytes etc.
pub mod primitive;

/// parser parses the byte sequence to core Midi struct
pub(crate) mod parser;

/// hold utility functions
pub(crate) mod utils;

/// reader module will 
pub mod reader;

/// core MIDI models and some of its derivatives
pub mod model;

/// web module will expose rmidirs to web-assembly in js world.
pub mod web;
// pub mod ds;
