use std::default;

use self::channel_event::ChannelEvent;

pub mod channel_event;
pub mod meta_event;
pub mod sys_event;



#[derive(Debug, Clone, Default)]
pub enum  MidiEvent {
    ChannelEvent(ChannelEvent),
    MetaEvent,
    SysEvent,
    #[default] Uinit // Uninitialize MIDI Event 
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
    match self {
        Self::ChannelEvent(_) => true,
        _ => false
    }
  }

  pub fn event_byte(&self) -> u8 {
    match self {
        Self::ChannelEvent(event) => event.event_byte(),
        Self::MetaEvent => 0xFF,
        Self::SysEvent => 0xF0,
        Self::Uinit => panic!("can't get event byte from uninitialized event {:?}", self)
    }
  }
}

impl From<(u8, &[u8])> for MidiEvent {
    fn from((byte, tail): (u8, &[u8])) -> Self {
      match MidiEvent::event_type(byte) {
        "CHANNEL_EVENT" => { 
          MidiEvent::ChannelEvent(ChannelEvent::from((byte, tail)))
        }, 
        "META_EVENT" => { // meta event
          MidiEvent::MetaEvent
        },
        "SYS_EVENT" => { // sysex event
          MidiEvent::SysEvent
        }
        _ => panic!("Can't create MIDI event. Unexpected MIDI event byte, passed 0x{:0X}", byte)
    }
    }
}

impl From<&[u8]> for MidiEvent {
    fn from(bytes: &[u8]) -> Self {
       Self::from((bytes[0], &bytes[1..]))
    }
}