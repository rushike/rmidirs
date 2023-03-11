use std::{fmt, error};

use super::parser_state::ParserState;

#[derive(Debug, Clone)]
pub enum MidiParseErrorKind {
  EndOfBuffer,
  InvalidEventByte,
  NotMidiMetricTime,
  InvalidMidiTrackHeader,
  InvalidMidiHeader,
}

impl fmt::Display for MidiParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct MidiParseError {
  state : ParserState,
  kind : MidiParseErrorKind,
  trace : Option<String>,
}

impl MidiParseError {
  pub fn new(state: ParserState, kind: MidiParseErrorKind, trace : Option<String>) -> MidiParseError {
    return MidiParseError { state, kind, trace};
  }
}

impl fmt::Display for MidiParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Error occurred while parsing at position {}: {}", self.state, self.kind)
    }
}

impl error::Error for MidiParseError {}