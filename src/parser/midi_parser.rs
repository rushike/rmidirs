use crate::{model::core::{midi::Midi, midi_track::MidiTrack}, reader::{Reader, buffer::Buffer}};

use super::{midi_header_parser::MidiHeaderParser, Parser, midi_track_parser::MidiTrackParser, parser_state::ParserState, midi_track_header_parser::{MidiTrackHeaderParser, self}, error::MidiParseError};

pub struct MidiParser;

impl MidiParser {
    /// Parses a midi buffer to Midi instance
    /// Midi buffer should have valid Midi header and Midi track information
    pub fn parse(buf : &[u8]) -> Result<Midi, MidiParseError> {
      let mut state = ParserState::new(String::from("midi"), 0, buf.len());

      let mut midi = Midi::default();

      let midi_header = MidiHeaderParser::parse(buf, &mut state);

      midi.add_header(midi_header.clone());


      let track_header_state = state.with(String::from("track-header-parser"));

      match MidiTrackHeaderParser::parse(buf, midi_header, track_header_state) {
        Ok(mut midi_track_parsers) => {

          midi_track_parsers
            .iter_mut()
            .map(|midi_track_parser| midi_track_parser.parse(buf))
            .map(|midi_track| -> Result<_, MidiParseError>{
              match midi_track {
                Ok(track) => Ok(midi.add_track(track)),
                Err(err) => return Err(err),
              }
            })
            .collect::<Vec<Result<_, MidiParseError>>>()
            ;
        },
        Err(err) => return Err(err)
      };
      return Ok(midi)
    }


    
}