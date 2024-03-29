
use std::default;

use crate::{
  model::core::{
    midi::Midi, 
    midi_track::MidiTrack, 
    midi_header::MidiHeader, midi_event::{delta_time::DeltaTime, MidiEvent, MidiMessageType, MidiMessage, channel_message}
  }, 
  utils::{ByteEncodingFormat, functions}, 
  parser::{
    midi_event_parser::MidiEventParser,
    error::{
      MidiParseError,
      MidiParseErrorKind::{InvalidMidiTrackHeader, EndOfBuffer, self}
    }
  }, primitive::MXByte, 
};

use super::{Parser, parser_state::ParserState};


#[derive(Debug, Clone)]
pub struct MidiTrackParser {
  midi_header : MidiHeader,
  state : ParserState
}

impl MidiTrackParser {

  pub fn new(midi_header: MidiHeader, state : ParserState) -> MidiTrackParser {
    MidiTrackParser { midi_header, state }
  }

  /// Parses a bytes into MIDI track.
  pub fn parse(&mut self, buf : &[u8]) -> Result<MidiTrack, MidiParseError> {
    let mut midi_track = MidiTrack::default();

    let mut midi_event_parser = MidiEventParser::new(&self.midi_header);

    let mut last_event_byte = None;

    loop {
      if self.state.curr() >= self.state.end() {return Ok(midi_track)}

      let delta_time = DeltaTime::from(self.state.mxbyte(buf));
      
      let midi_event =  match MidiEvent::event_type(self.state.byte(buf))  {

          Some(MidiMessageType::Channel) | None => {
            match midi_event_parser.parse_channel_event(buf, &mut self.state, last_event_byte) {
              Ok(channel_message) => {
                last_event_byte = channel_message.event_byte();
                MidiEvent::new(delta_time, MidiMessage::ChannelMessage(channel_message))
              },
              Err(err) => return Err(err)
            }
          },  
          Some(MidiMessageType::Meta) => {
            last_event_byte = None;
            let meta_event = midi_event_parser.parse_meta_event(buf, &mut self.state).unwrap();
            MidiEvent::new(delta_time, MidiMessage::MetaMessage(meta_event))
          },
          Some(MidiMessageType::Sys) => {
            last_event_byte = None;
            let sys_event = midi_event_parser.parse_sys_event(buf).unwrap();
            MidiEvent::new(delta_time, MidiMessage::SysMessage(sys_event))
          }

          Some(MidiMessageType::Invalid(msg)) => return Err(
            MidiParseError::new(
              self.state.clone(), 
              MidiParseErrorKind::InvalidEventByte, 
              format!("invalid event byte {}, can't tag to Channel, Meta or Sys event", msg),
              Some(msg))
          )
      };
      midi_track.add_event(midi_event);
    };  
  }

}
