
use std::{fs::File, default};

use serde_json;

use lazy_static::{self, __Deref};


use crate::primitive::{
  M4Bits, M1Byte,
  m1byte, m4bits
};


lazy_static::lazy_static!(
  #[derive(Debug)]
  static ref CHANNEL_EVENT_SCHEMA : serde_json::Value = serde_json::de::from_reader(File::open("./midis/midi-channel-event-schema.json").expect("File should open read only")).unwrap();
);

#[derive(Debug, Clone)]
pub struct NoteOn {
  channel : M4Bits,
  note : M1Byte,
  velocity : M1Byte
}

#[derive(Debug, Clone)]
pub struct NoteOff {
  channel : M4Bits,
  note : M1Byte,
  velocity : M1Byte
}

#[derive(Debug, Clone)]
pub struct AfterTouch {
  channel : M4Bits,
  note : M1Byte,
  amount : M1Byte
}

#[derive(Debug, Clone)]
pub struct Controller {
  channel : M4Bits,
  controller_type : M1Byte,
  value : M1Byte
}

#[derive(Debug, Clone)]
pub struct ProgramChange {
  channel : M4Bits,
  program_number : M1Byte
}

#[derive(Debug, Clone)]
pub struct ChannelAfterTouch {
  channel : M4Bits,
  amount : M1Byte
}

#[derive(Debug, Clone)]
pub struct PitchBend {
  channel : M4Bits,
  vlsb : M1Byte,
  vmsb : M1Byte
}

#[derive(Debug, Clone)]
#[repr(u32)]
pub enum ChannelMessage {
  NoteOn(NoteOn) = 0x8,
  NoteOff(NoteOff) = 0x9,
  AfterTouch(AfterTouch) = 0xA,
  Controller(Controller) = 0xB,
  ProgramChange(ProgramChange)= 0xC,
  ChannelAfterTouch(ChannelAfterTouch) = 0xD,
  PitchBend(PitchBend) = 0xE,
  Invalid(String)
}

impl ChannelMessage {

  pub fn event_byte(&self) -> Option<u8> {
    match self.event_channel() {
      Some(channel) => match self {
        ChannelMessage::NoteOn(_) => Some(0x8 << 4 | channel),
        ChannelMessage::NoteOff(_) => Some(0x9 << 4 | channel),
        ChannelMessage::AfterTouch(_) => Some(0xA << 4 | channel),
        ChannelMessage::Controller(_) => Some(0xB << 4 | channel),
        ChannelMessage::ProgramChange(_) => Some(0xC << 4 | channel),
        ChannelMessage::ChannelAfterTouch(_) => Some(0xD << 4 | channel),
        ChannelMessage::PitchBend(_) => Some(0xE << 4 | channel),
        ChannelMessage::Invalid(_) => None
      },
      None => None,
    }
  }

  pub fn event_channel(&self) -> Option<u8> {
    match self {
        ChannelMessage::NoteOn(e) => Some(e.channel.into()),
        ChannelMessage::NoteOff(e) => Some(e.channel.into()),
        ChannelMessage::AfterTouch(e) => Some(e.channel.into()),
        ChannelMessage::Controller(e) => Some(e.channel.into()),
        ChannelMessage::ProgramChange(e) => Some(e.channel.into()),
        ChannelMessage::ChannelAfterTouch(e) => Some(e.channel.into()),
        ChannelMessage::PitchBend(e) => Some(e.channel.into()),
        ChannelMessage::Invalid(_) => None
    }
  }

  pub fn is_note_on_off_event(&self) -> bool {
    match self {
      ChannelMessage::NoteOn(_) | ChannelMessage::NoteOff(_) => true,
      _=> false
    } 
  }

  pub fn is_note_on_event(&self) -> bool {
    match self {
      ChannelMessage::NoteOn(_) => true,
      ChannelMessage::NoteOff(event) => *event.velocity != 0,
      _=> false
    } 
  }

  pub fn is_note_off_event(&self) -> bool {
    match self {
      ChannelMessage::NoteOn(event) => *event.velocity == 0,
      ChannelMessage::NoteOff(_) => true,
      _=> false
    } 
  }

  pub fn get_note_number(&self) -> Option<M1Byte> {
    match self {
      ChannelMessage::NoteOn(event) => Some(event.note),
      ChannelMessage::NoteOff(event) => Some(event.note),
      _=> None
    } 
  }

  pub fn is_channel_event(byte : u8) -> bool {
    return byte & 0xF0 >= 0x80 && byte & 0xF0 < 0xF0;
  }

  pub fn is_controller_event(byte : u8) -> bool {
    return byte & 0xF0 == 0xB0;
  }

  pub fn info_from_str(name : &str) -> &serde_json::Value {
    return &CHANNEL_EVENT_SCHEMA["map_str"][name];
  }
  
  pub fn info_from_byte(byte : u8) -> &'static serde_json::Value {
    return &CHANNEL_EVENT_SCHEMA[format!("{:X}", byte)];
  }

  pub fn get_info(byte : u8) -> &'static serde_json::Value {
    return Self::info_from_byte(byte);
  }
}

impl From<(u8, &[u8])> for ChannelMessage {
    fn from((byte, rest): (u8, &[u8])) -> Self {
      
      match byte & 0xF0 {
        0x80 => {
          ChannelMessage::NoteOn(NoteOn {
            channel: m4bits!(byte & 0xF),
            note: m1byte!(rest[0]),
            velocity: m1byte!(rest[1]),
          })
        },
        0x90 => {
          ChannelMessage::NoteOff(NoteOff {
            channel: m4bits!(byte & 0xF),
            note: m1byte!(rest[0]),
            velocity: m1byte!(rest[1]),
          })
        }
        0xA0 => {
          ChannelMessage::AfterTouch(AfterTouch {
            channel: m4bits!(byte & 0xF),
            note: m1byte!(rest[0]),
            amount : m1byte!(rest[1]),
          }) 
        }, 
        0xB0 => {
          ChannelMessage::Controller(Controller {
            channel: m4bits!(byte & 0xF),
            controller_type : m1byte!(rest[0]),
            value: m1byte!(rest[1]),
          }) 
        },
        0xC0 => {
          ChannelMessage::ProgramChange(ProgramChange {
            channel: m4bits!(byte & 0xF),
            program_number : m1byte!(rest[0]),
          })
        },
        0xD0 => {
          ChannelMessage::ChannelAfterTouch(ChannelAfterTouch {
            channel: m4bits!(byte & 0xF),
            amount : m1byte!(rest[0]),
          })
        },
        0xE0 => {
          ChannelMessage::PitchBend(PitchBend {
            channel: m4bits!(byte & 0xF),
            vlsb : m1byte!(rest[0]),
            vmsb : m1byte!(rest[1]),
          })
        },
        byte    => panic!("From<&[u8]> trait not implemented for Channel-event with start byte 0x{:02X} ", byte) 
    }
  }
}

impl From<&[u8]> for ChannelMessage{
  fn from(bytes: &[u8]) -> Self {
    Self::from((bytes[0], &bytes[1..]))    
  }
}