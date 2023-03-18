use crate::{model::core::midi::Midi};

pub use self::midi_parser::MidiParser;

mod parser_state;
mod midi_parser;
mod midi_header_parser;
mod midi_track_parser;
mod midi_event_parser;
mod error;
mod midi_track_header_parser;

pub trait Parser {
  fn parse(buf : &[u8], midi : &mut Midi) -> usize;
}

