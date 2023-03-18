use crate::{
  utils::{ByteEncodingFormat, functions::number}, 
  parser::midi_track_parser::MidiTrackParser, 
  model::core::midi_header::MidiHeader
};

use super::{
  parser_state::ParserState, 
  error::{
    MidiParseError,
    MidiParseErrorKind::InvalidMidiTrackHeader
  }};

#[derive(Debug, Clone)]
pub struct MidiTrackHeaderParser;

impl MidiTrackHeaderParser {

  /// Parses the the Midi Tracks Header.
  /// 
  /// It just retrived the header and track length information.
  pub fn parse(buf : &[u8], midi_header : MidiHeader, mut state : ParserState) -> Result<Vec<MidiTrackParser>, MidiParseError>{
    
    const ENC_FORMAT: ByteEncodingFormat = ByteEncodingFormat::BigEndian;

    let mut midi_track_headers = Vec::new();

    let mut track_no = -1; 
    
    loop {

      if state.curr() >= state.end() { return Ok(midi_track_headers); }

      track_no += 1;

      let track_name = format!("track-{}", track_no);

      let ptr = state.curr();

      match &buf[ptr .. ptr + 4] {
        b"MTrk" => {
          let length = number(&buf[ptr + 4 .. ptr + 8], ENC_FORMAT);

          let total_length = 8 + length as usize; // header + track length

          let mut track_state = ParserState::new(
            track_name, 
            ptr, 
            ptr + total_length
          );

          track_state.forward(8);

          midi_track_headers.push(MidiTrackParser::new(midi_header.clone(), track_state));

          state.forward(total_length);
        }
        header => {
          let track_err_start = ParserState::new(
            format!("{}-err-header-{:?}", track_name, header.to_vec()),
            state.curr(),
            buf.len()
          );
          return Err(MidiParseError::new(track_err_start, InvalidMidiTrackHeader, None))
        }
      }
    }
  }
}