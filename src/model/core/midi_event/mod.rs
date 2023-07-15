use std::default;

use crate::primitive::{MXByte, M1Byte};

use self::{channel_message::ChannelMessage, meta_message::{MetaMessage, Tempo}, delta_time::DeltaTime, sys_event::SysEvent};

pub mod channel_message;
pub mod meta_message;
pub mod sys_event;

pub mod delta_time;



#[derive(Debug, Clone)]
pub enum MidiMessage {
    ChannelEvent(ChannelMessage),
    MetaEvent(MetaMessage),
    SysEvent(SysEvent),
    Invalid(String)
}


#[derive(Debug, Clone)]
pub enum MidiMessageType {
  Channel,
  Meta,
  Sys,
  Invalid(String)
}

impl MidiMessage {
  pub fn event_type<'a>(byte : u8) -> Option<MidiMessageType> {
    if byte & 0xF0 >= 0x80 && byte & 0xF0 < 0xF0 { return Some(MidiMessageType::Channel) };
    match byte {
       0xFF => Some(MidiMessageType::Meta),
       0xF0 | 0xF7  => Some(MidiMessageType::Sys),
       _ => None
    }
  }
}

impl From<(u8, &[u8])> for MidiMessage {

  fn from((byte, rest): (u8, &[u8])) -> Self {

    let event_type = Self::event_type(byte).unwrap_or(MidiMessageType::Invalid(format!("Can't create MIDI event. Unexpected MIDI event byte, passed 0x{:0X}", byte)));
    
    match event_type  {
      MidiMessageType::Channel => { 
        MidiMessage::ChannelEvent(ChannelMessage::from((byte, rest)))
      }, 
      MidiMessageType::Meta => { // meta event
        MidiMessage::MetaEvent(MetaMessage::from((byte, rest)))
      },
      MidiMessageType::Sys => { // sysex event
        MidiMessage::SysEvent(SysEvent)
      }
      MidiMessageType::Invalid(msg) => MidiMessage::Invalid(msg),
    } 
  }
}


#[derive(Debug, Clone)]
pub struct MidiEvent {
  delta_time : DeltaTime,
  message : MidiMessage
}

impl MidiEvent {
  pub fn new(delta_time : DeltaTime, message : MidiMessage) -> MidiEvent {
    MidiEvent{delta_time, message}
  }
  pub fn event_type(byte : u8) -> Option<MidiMessageType> {
    MidiMessage::event_type(byte)
  }

  pub fn is_channel_byte(byte : u8) -> bool {
    byte >= 0x80 && byte < 0xFF
  }

  pub fn is_meta_byte(byte : u8) -> bool {
    byte == 0xFF
  }

  pub fn is_sys_byte(byte : u8) -> bool {
    byte == 0xF7 || byte ==  0xF0
  }

  pub fn is_channel_event(&self) -> bool {
    match self.message {
      MidiMessage::ChannelEvent(_) => true,
        _ => false
    }
  }

  pub fn is_note_on_off_event(&self) -> bool {
    match &self.message {
      MidiMessage::ChannelEvent(event) => event.is_note_on_off_event(),
      _ => false
  }
  }

  pub fn is_note_on_event(&self) -> bool {
    match &self.message {
      MidiMessage::ChannelEvent(event) => event.is_note_on_event(),
      _ => false
    }
  }

  pub fn is_note_off_event(&self) -> bool {
    match &self.message {
      MidiMessage::ChannelEvent(event) => event.is_note_off_event(),
      _ => false
    }
  }

  pub fn delta_time(&self) -> &DeltaTime {&self.delta_time}

  pub fn message(&self) -> &MidiMessage {&self.message}

  pub fn is_tempo_event(&self) -> bool {
    match &self.message {
      MidiMessage::MetaEvent(event) => event.is_tempo_event(),
      _=> false
    }
  }

  pub fn get_tempo(&self) -> Option<Tempo> {
    match &self.message {
      MidiMessage::MetaEvent(msg) => msg.get_tempo(),
      _ => None,
    }
  } 

  pub fn get_note_number(&self) -> Option<M1Byte> {
    match &self.message {
      MidiMessage::ChannelEvent(event) => event.get_note_number(),
      _ => None
    }
  }

  pub fn event_byte(&self) -> Option<u8> {
    match &self.message {
        MidiMessage::ChannelEvent(event) => event.event_byte(),
        MidiMessage::MetaEvent(_) => Some(0xFF),
        MidiMessage::SysEvent(_) => Some(0xF0),
        MidiMessage::Invalid(_) => None
    }
  }
}

impl From<(DeltaTime, MidiMessage)> for MidiEvent {
    fn from((delta_time, message): (DeltaTime, MidiMessage)) -> Self {
      MidiEvent { delta_time, message }
    }
}

impl From<(MXByte, u8, &[u8])> for MidiEvent {
    fn from((delta_time, byte, tail): (MXByte, u8, &[u8])) -> Self {
      
      Self::from(
        (
          DeltaTime::from(delta_time),
          MidiMessage::from((byte, tail)),
        )
      )
    }
}

impl From<(MXByte, &[u8])> for MidiEvent {
    fn from((delta_time, bytes): (MXByte, &[u8])) -> Self {
       Self::from((delta_time, bytes[0], &bytes[1..]))
    }
}

#[derive(Debug, Clone)]
pub struct AbsoluteMidiEvent {
  time : f32,
  message : MidiMessage
}

impl AbsoluteMidiEvent {
  pub fn new(time : f32, message: MidiMessage) -> Self {
    AbsoluteMidiEvent { time, message}
  }
}
