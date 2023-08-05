use std::default;

use crate::primitive::{MXByte, M1Byte};

use self::{channel_message::ChannelMessage, meta_message::{MetaMessage, Tempo}, delta_time::DeltaTime, sys_event::SysEvent};

pub mod channel_message;
pub mod meta_message;
pub mod sys_event;

pub mod delta_time;



#[derive(Debug, Clone)]
pub enum MidiMessage {
    ChannelMessage(ChannelMessage),
    MetaMessage(MetaMessage),
    SysMessage(SysEvent),
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
      MidiMessageType::Channel => { // channel event
        MidiMessage::ChannelMessage(ChannelMessage::from((byte, rest)))
      }, 
      MidiMessageType::Meta => { // meta event
        MidiMessage::MetaMessage(MetaMessage::from((byte, rest)))
      },
      MidiMessageType::Sys => { // sysex event
        MidiMessage::SysMessage(SysEvent)
      }
      MidiMessageType::Invalid(msg) => MidiMessage::Invalid(msg),
    } 
  }
}

impl From<MidiMessage> for Vec<u8> {
  /// Convert MidiMessage to bytes
  fn from(message: MidiMessage) -> Self {
    match message {
        MidiMessage::ChannelMessage(channel_message) => channel_message.into(),
        MidiMessage::MetaMessage(meta_message) => meta_message.into(),
        MidiMessage::SysMessage(sys_event) => sys_event.into(),
        MidiMessage::Invalid(_) => vec![],
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
      MidiMessage::ChannelMessage(_) => true,
        _ => false
    }
  }

  pub fn is_note_on_off_event(&self) -> bool {
    match &self.message {
      MidiMessage::ChannelMessage(event) => event.is_note_on_off_event(),
      _ => false
  }
  }

  pub fn is_note_on_event(&self) -> bool {
    match &self.message {
      MidiMessage::ChannelMessage(event) => event.is_note_on_event(),
      _ => false
    }
  }

  pub fn is_note_off_event(&self) -> bool {
    match &self.message {
      MidiMessage::ChannelMessage(event) => event.is_note_off_event(),
      _ => false
    }
  }

  pub fn delta_time(&self) -> &DeltaTime {&self.delta_time}

  pub fn message(&self) -> &MidiMessage {&self.message}

  pub fn is_tempo_event(&self) -> bool {
    match &self.message {
      MidiMessage::MetaMessage(event) => event.is_tempo_event(),
      _=> false
    }
  }

  pub fn get_tempo(&self) -> Option<Tempo> {
    match &self.message {
      MidiMessage::MetaMessage(msg) => msg.get_tempo(),
      _ => None,
    }
  } 

  pub fn get_note_number(&self) -> Option<M1Byte> {
    match &self.message {
      MidiMessage::ChannelMessage(event) => event.get_note_number(),
      _ => None
    }
  }

  pub fn event_byte(&self) -> Option<u8> {
    match &self.message {
        MidiMessage::ChannelMessage(event) => event.event_byte(),
        MidiMessage::MetaMessage(_) => Some(0xFF),
        MidiMessage::SysMessage(_) => Some(0xF0),
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

impl From<MidiEvent> for Vec<u8> {
  fn from(midi_event: MidiEvent) -> Self {
    let delta_time : Vec<u8> = midi_event.delta_time.into();
    let message     : Vec<u8> = midi_event.message.into();

    [delta_time, message].concat()
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
