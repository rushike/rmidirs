use std::{fs::File, error::Error, fmt::format};

use crate::{
  model::{
    midi_event::{MidiEvent, self, meta_message::{self, MetaMessage}, channel_message::{ChannelMessage}, sys_event::SysEvent, MidiMessage}, 
    midi::Midi, midi_header::MidiHeader
  }, 
  primitive::{
    M1Byte,
    m1byte, MXByte
  }, utils::functions::from_var_len};

use super::{parser_state::ParserState, error::{MidiParseError, MidiParseErrorKind}};

lazy_static::lazy_static!(
  #[derive(Debug)]
  static ref CHANNEL_EVENT_SCHEMA : serde_json::Value = serde_json::de::from_reader(File::open("./src/parser/schema/midi-v1-channel-event-schema.json").expect("File should open read only")).unwrap();
  static ref META_EVENT_SCHEMA : serde_json::Value = serde_json::de::from_reader(File::open("./src/parser/schema/midi-v1-meta-event-schema.json").expect("File should open read only")).unwrap();
);

#[derive(Debug)]
pub struct MidiEventParser<'a> {
  midi_header : &'a MidiHeader,
}

impl<'a> MidiEventParser<'a> {
  pub fn new(midi_header : &MidiHeader) -> MidiEventParser {
    MidiEventParser { midi_header}
  }

  pub fn parse_channel_event(&mut self, buf : &[u8], state : &mut ParserState, last_event_byte : Option<u8>) -> Result<ChannelMessage, MidiParseError> {

    let event_byte = if MidiEvent::is_channel_byte( state.byte(buf)) { 
      state.byte(buf)
    } else {
      last_event_byte.expect(format!("{:X} not a channel event", state.byte(buf)).as_str())
    };
    
    let event_type = (event_byte & 0xF0) >> 4;

    let event_info = &CHANNEL_EVENT_SCHEMA["info"][format!("0x{:1X}", event_type)];

    if event_info.is_null() {return Err(MidiParseError::new(state.with(format!("{}@channel[0x{:1X}]", state.name(), event_byte)), MidiParseErrorKind::InvalidEventByte, None));}

    let length = event_info["length"].as_u64().unwrap() as usize;
  
    let total_length = 1 + length;
    
    state.forward(total_length);

    Ok(ChannelMessage::from((event_byte, &buf[state.curr() - length.. state.curr()])))
  }

  pub fn parse_meta_event(&mut self, buf : &[u8], state : &mut ParserState) -> Result<MetaMessage, MidiParseError> {
    
    let event_type = state.next(buf, 1)[0];
    let event_sub_type = state.next(buf, 1)[0];

    if META_EVENT_SCHEMA["info"]["0xFF"]["types"][format!("0x{:02X}", event_sub_type)].is_null() {return Err(MidiParseError::new(state.with(format!("{}@meta[0X{:02X}][0X{:02X}]", state.name(), event_type, event_sub_type)), MidiParseErrorKind::InvalidEventByte, None))}

    let event_length = *state.mxbyte(buf) as usize;
    
    state.forward(event_length);

    let total_length = event_length + 2;

    Ok(MetaMessage::from(state.retake(buf, total_length)))
  }

  pub fn parse_sys_event(&mut self, buf : &[u8]) -> Result<SysEvent, MidiParseError> {
    Ok(SysEvent)
  }
}