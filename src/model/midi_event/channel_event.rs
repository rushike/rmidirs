
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

#[derive(Debug, Clone, Default)]
pub enum ChannelEvent {
  NoteOn(NoteOn),
  NoteOff(NoteOff),
  AfterTouch(AfterTouch),
  Controller(Controller),
  ProgramChange(ProgramChange),
  ChannelAfterTouch(ChannelAfterTouch),
  PitchBend(PitchBend),
  #[default] Uinit
}

impl ChannelEvent {

  pub fn event_byte(&self) -> u8 {
    match self {
      ChannelEvent::NoteOn(_) => 0x8 << 4 | self.event_channel(),
      ChannelEvent::NoteOff(_) => 0x9 << 4 | self.event_channel(),
      ChannelEvent::AfterTouch(_) => 0xA << 4 | self.event_channel(),
      ChannelEvent::Controller(_) => 0xB << 4 | self.event_channel(),
      ChannelEvent::ProgramChange(_) => 0xC << 4 | self.event_channel(),
      ChannelEvent::ChannelAfterTouch(_) => 0xD << 4 | self.event_channel(),
      ChannelEvent::PitchBend(_) => 0xE << 4 | self.event_channel(),
      ChannelEvent::Uinit => panic!("Can't get event byte for uninitialized channel event. Channel event is {:?}", self),
    }
  }

  pub fn event_channel(&self) -> u8 {
    match self {
        ChannelEvent::NoteOn(e) => e.channel.into(),
        ChannelEvent::NoteOff(e) => e.channel.into(),
        ChannelEvent::AfterTouch(e) => e.channel.into(),
        ChannelEvent::Controller(e) => e.channel.into(),
        ChannelEvent::ProgramChange(e) => e.channel.into(),
        ChannelEvent::ChannelAfterTouch(e) => e.channel.into(),
        ChannelEvent::PitchBend(e) => e.channel.into(),
        ChannelEvent::Uinit => panic!("Can't get event channel for uninitialized channel event. Channel event is {:?}", self),
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

impl From<(u8, &[u8])> for ChannelEvent {
    fn from((byte, tail): (u8, &[u8])) -> Self {
      match byte & 0xF0 {
        0x80 => {
          ChannelEvent::NoteOn(NoteOn {
            channel: m4bits!(byte & 0xF),
            note: m1byte!(tail[0]),
            velocity: m1byte!(tail[1]),
          })
        },
        0x90 => {
          ChannelEvent::NoteOff(NoteOff {
            channel: m4bits!(byte & 0xF),
            note: m1byte!(tail[0]),
            velocity: m1byte!(tail[1]),
          })
        }
        0xA0 => {
          ChannelEvent::AfterTouch(AfterTouch {
            channel: m4bits!(byte & 0xF),
            note: m1byte!(tail[0]),
            amount : m1byte!(tail[1]),
          }) 
        }, 
        0xB0 => {
          ChannelEvent::Controller(Controller {
            channel: m4bits!(byte & 0xF),
            controller_type : m1byte!(tail[0]),
            value: m1byte!(tail[1]),
          }) 
        },
        0xC0 => {
          ChannelEvent::ProgramChange(ProgramChange {
            channel: m4bits!(byte & 0xF),
            program_number : m1byte!(tail[0]),
          })
        },
        0xD0 => {
          ChannelEvent::ChannelAfterTouch(ChannelAfterTouch {
            channel: m4bits!(byte & 0xF),
            amount : m1byte!(tail[0]),
          })
        },
        0xE0 => {
          ChannelEvent::PitchBend(PitchBend {
            channel: m4bits!(byte & 0xF),
            vlsb : m1byte!(tail[0]),
            vmsb : m1byte!(tail[1]),
          })
        },
        byte    => panic!("From<&[u8]> trait not implemented for Channel event with start byte {byte} ") 
    }
  }
}

impl From<&[u8]> for ChannelEvent{
  fn from(bytes: &[u8]) -> Self {
    Self::from((bytes[0], &bytes[1..]))    
  }
}