use crate::{model::midi::Midi};

pub use self::midi_parser::MidiParser;

mod midi_parser;
mod midi_header_parser;
mod midi_track_parser;
mod midi_event_parser;

pub trait Parser {
  fn parse(buf : &[u8], midi : &mut Midi) -> usize;
}

