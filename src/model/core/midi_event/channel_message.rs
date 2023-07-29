
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
  pub(crate) channel : M4Bits,
  pub(crate) note : M1Byte,
  pub(crate) velocity : M1Byte
}

impl From<NoteOn> for Vec<u8> {
  fn from(note_on: NoteOn) -> Self {
    vec![0x90 & u8::from(note_on.channel), note_on.note.into(), note_on.velocity.into()]
  }
}

#[derive(Debug, Clone)]
pub struct NoteOff {
  pub(crate) channel : M4Bits,
  pub(crate) note : M1Byte,
  pub(crate) velocity : M1Byte
}

impl From<NoteOff> for Vec<u8> {
  fn from(note_off: NoteOff) -> Self {
    vec![0x80 & u8::from(note_off.channel), note_off.note.into(), note_off.velocity.into()]
  }
}

#[derive(Debug, Clone)]
pub struct AfterTouch {
  channel : M4Bits,
  note : M1Byte,
  amount : M1Byte
}

impl From<AfterTouch> for Vec<u8> {
  fn from(after_touch: AfterTouch) -> Self {
    vec![0xA0 & u8::from(after_touch.channel), after_touch.note.into(), after_touch.amount.into()]
  }
}

#[derive(Debug, Clone)]
pub struct Controller {
  channel : M4Bits,
  controller_type : M1Byte,
  value : M1Byte
}

impl From<Controller> for Vec<u8> {
  fn from(controller: Controller) -> Self {
    vec![0xB0 & u8::from(controller.channel), controller.controller_type.into(), controller.value.into()]
  }
}


#[derive(Debug, Clone)]
pub struct ProgramChange {
  channel : M4Bits,
  program_number : M1Byte
}

impl From<ProgramChange> for Vec<u8> {
  fn from(program_change: ProgramChange) -> Self {
    vec![0xC0 & u8::from(program_change.channel), program_change.program_number.into()]
  }
}

#[derive(Debug, Clone)]
pub struct ChannelAfterTouch {
  channel : M4Bits,
  amount : M1Byte
}

impl From<ChannelAfterTouch> for Vec<u8> {
  fn from(channel_after_touch: ChannelAfterTouch) -> Self {
    vec![0xD0 & u8::from(channel_after_touch.channel), channel_after_touch.amount.into()]
  }
}


#[derive(Debug, Clone)]
pub struct PitchBend {
  channel : M4Bits,
  vlsb : M1Byte,
  vmsb : M1Byte
}

impl From<PitchBend> for Vec<u8> {
  fn from(pitch_bend: PitchBend) -> Self {
    vec![0xE0 & u8::from(pitch_bend.channel), pitch_bend.vlsb.into(), pitch_bend.vmsb.into()]
  }
}


#[derive(Debug, Clone)]
#[repr(u32)]
pub enum ChannelMessage {
  NoteOn(NoteOn) = 0x9,
  NoteOff(NoteOff) = 0x8,
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
        ChannelMessage::NoteOn(_) => Some(0x9 << 4 | channel),
        ChannelMessage::NoteOff(_) => Some(0x8 << 4 | channel),
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
      ChannelMessage::NoteOn(note_on) => *note_on.velocity != 0,
      ChannelMessage::NoteOff(note_off) => *note_off.velocity != 0,
      _=> false
    } 
  }

  pub fn is_note_off_event(&self) -> bool {
    match self {
      ChannelMessage::NoteOn(event) => *event.velocity == 0,
      ChannelMessage::NoteOff(note_off) => *note_off.velocity == 0,
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

  pub fn info_from_name(name : &str) -> &serde_json::Value {
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
        0x90 => {
          ChannelMessage::NoteOn(NoteOn {
            channel: m4bits!(byte & 0xF),
            note: m1byte!(rest[0]),
            velocity: m1byte!(rest[1]),
          })
        },
        0x80 => {
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
        byte => ChannelMessage::Invalid(format!("From<&[u8]> trait not implemented for Channel-event with start byte 0x{:02X} ", byte))
    }
  }
}

impl From<&[u8]> for ChannelMessage{
  fn from(bytes: &[u8]) -> Self {
    Self::from((bytes[0], &bytes[1..]))    
  }
}

/// Converts ChannelMessage to bytes (midi v1 format).
/// 
/// It simply calls `.into()` method on the individual message type
/// 
impl From<ChannelMessage> for  Vec<u8>{
  fn from(msg: ChannelMessage) -> Self {
    match msg {
        ChannelMessage::NoteOn(note_on) => note_on.into(),
        ChannelMessage::NoteOff(note_off) => note_off.into(),
        ChannelMessage::AfterTouch(after_touch) => after_touch.into(),
        ChannelMessage::Controller(controller) => controller.into(),
        ChannelMessage::ProgramChange(program_change) => program_change.into(),
        ChannelMessage::ChannelAfterTouch(channel_after_touch) => channel_after_touch.into(),
        ChannelMessage::PitchBend(pitch_bend) => pitch_bend.into(),
        ChannelMessage::Invalid(_) => vec![],
    }
  }
}