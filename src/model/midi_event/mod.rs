use std::default;

use crate::primitive::{MXByte, M1Byte};

use self::{channel_event::ChannelEvent, meta_event::MetaEvent, delta_time::DeltaTime};

pub mod channel_event;
pub mod meta_event;
pub mod sys_event;

pub mod delta_time;



#[derive(Debug, Clone, Default)]
pub enum Event {
    ChannelEvent(ChannelEvent),
    MetaEvent(MetaEvent),
    SysEvent,
    #[default] Uinit // Uninitialize MIDI Event 
}

impl Event {
  pub fn event_type<'a>(byte : u8) -> &'a str {
    if byte & 0xF0 >= 0x80 && byte & 0xF0 < 0xF0 {return "CHANNEL_EVENT"};
    match byte {
       0xFF => "META_EVENT",
       0xF0 | 0xF7  => "SYS_EVENT",
       _ => "UNINIT_EVENT"
      // _byte => panic!("can't not recognize valid midi event. Passed start byte as 0x{_byte:0X}")
    }
  }
}

impl From<(u8, &[u8])> for Event {
  fn from((byte, rest): (u8, &[u8])) -> Self {
    match Self::event_type(byte)  {
      "CHANNEL_EVENT" => { 
        Event::ChannelEvent(ChannelEvent::from((byte, rest)))
      }, 
      "META_EVENT" => { // meta event
        Event::MetaEvent(MetaEvent::from((byte, rest)))
      },
      "SYS_EVENT" => { // sysex event
        Event::SysEvent
      }
      _ => panic!("Can't create MIDI event. Unexpected MIDI event byte, passed 0x{:0X}", byte)
    } 
  }
}


#[derive(Debug, Clone, Default)]
pub struct MidiEvent {
  pub(crate) delta_time : DeltaTime,
  pub(crate) event : Event
}

impl MidiEvent {
  pub fn event_type<'a>(byte : u8) -> &'a str {
    if byte & 0xF0 >= 0x80 && byte & 0xF0 < 0xF0 {return "CHANNEL_EVENT"};
    match byte {
       0xFF => "META_EVENT",
       0xF0 | 0xF7  => "SYS_EVENT",
       _ => "UNINIT_EVENT"
      // _byte => panic!("can't not recognize valid midi event. Passed start byte as 0x{_byte:0X}")
    }
  }

  pub fn is_channel_byte(byte : u8) -> bool {
    byte & 0xF0 >= 0x80 && byte & 0xF0 < 0xF
  }
  pub fn is_meta_byte(byte : u8) -> bool {
    byte == 0xFF
  }

  pub fn is_sys_byte(byte : u8) -> bool {
    byte == 0xF7 || byte ==  0xF0
  }

  pub fn is_channel_event(&self) -> bool {
    match self.event {
        Event::ChannelEvent(_) => true,
        _ => false
    }
  }

  pub fn is_note_on_off_event(&self) -> bool {
    match &self.event {
      Event::ChannelEvent(event) => event.is_note_on_off_event(),
      _ => false
  }
  }

  pub fn is_note_on_event(&self) -> bool {
    match &self.event {
      Event::ChannelEvent(event) => event.is_note_on_event(),
      _ => false
    }
  }

  pub fn is_note_off_event(&self) -> bool {
    match &self.event {
      Event::ChannelEvent(event) => event.is_note_off_event(),
      _ => false
    }
  }

  pub fn is_tempo_event(&self) -> bool {
    match &self.event {
      Event::MetaEvent(event) => event.is_tempo_event(),
      _=> false
    }
  }

  pub fn get_note_number(&self) -> Option<M1Byte> {
    match &self.event {
      Event::ChannelEvent(event) => event.get_note_number(),
      _ => None
    }
  }

  pub fn event_byte(&self) -> u8 {
    match &self.event {
        Event::ChannelEvent(event) => event.event_byte(),
        Event::MetaEvent(_) => 0xFF,
        Event::SysEvent => 0xF0,
        Event::Uinit => panic!("can't get event byte from uninitialized event {:?}", self)
    }
  }
}

impl From<(DeltaTime, Event)> for MidiEvent {
    fn from((delta_time, event): (DeltaTime, Event)) -> Self {
        MidiEvent { delta_time, event }
    }
}

impl From<(MXByte, u8, &[u8])> for MidiEvent {
    fn from((delta_time, byte, tail): (MXByte, u8, &[u8])) -> Self {
      
      Self::from(
        (
          DeltaTime::from(delta_time),
          Event::from((byte, tail)),
        )
      )
    }
}

impl From<(MXByte, &[u8])> for MidiEvent {
    fn from((delta_time, bytes): (MXByte, &[u8])) -> Self {
       Self::from((delta_time, bytes[0], &bytes[1..]))
    }
}