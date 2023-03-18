use crate::{primitive::{M3Byte, M1Byte, m1byte, m3byte, m1bit, M1Bit, Word, MNBits}, };

use super::MidiMessage;

#[derive(Debug, Clone)]
pub struct TextEvent;

#[derive(Debug, Clone)]
pub struct ChannelPrefix;

#[derive(Debug, Clone)]
pub struct MIDIPort(M1Byte);

#[derive(Debug, Clone)]
pub struct EndOfTrack;

#[derive(Debug, Clone)]
pub struct Tempo(M3Byte);

impl Tempo {
  pub fn new(micro_secs : M3Byte) -> Self {Tempo(micro_secs)}
  pub fn secs(&self) -> f32 {
    *self.0 as f32 / 1000000.0
  }
  pub fn milli_secs(&self) -> f32 {
    *self.0 as f32 / 1000.0
  }
  pub fn micro_secs(&self) -> f32 {
    *self.0 as f32
  }
}

#[derive(Debug, Clone)]
pub struct SMPTEOffset;

#[derive(Debug, Clone)]
pub struct TimeSignature {
  nn : M1Byte,
  dd : M1Byte,
  cc : M1Byte,
  bb : M1Byte,
}

#[derive(Debug, Clone)]
pub struct KeySignature {
  sf : M1Byte,
  mi : M1Byte
}

#[derive(Debug, Clone)]
#[repr(i32)]
pub enum MetaMessage {
  Text(TextEvent) = 0x01,
  CopyrightNotice(TextEvent) = 0x02,
  TrackName(TextEvent) = 0x03,
  InstrumentName(TextEvent) = 0x04,
  Lyrics(TextEvent) = 0x05,
  Marker(TextEvent) = 0x06,
  CuePoint(TextEvent) = 0x07,
  ChannelPrefix(ChannelPrefix) = 0x20,
  MIDIPort(MIDIPort) = 0x21,
  EndOfTrack = 0x2F,
  Tempo(Tempo) = 0x51,
  SMPTEOffset = 0x54,
  TimeSignature(TimeSignature) = 0x58,
  KeySignature (KeySignature) = 0x59,
  Invalid(String)
}

impl MetaMessage {

  pub fn get_tempo(&self) -> Option<Tempo> {
    match self {
        MetaMessage::Tempo(tempo) => Some(tempo.clone()),
        _ => None,
    }
  }

  pub fn is_tempo_event(&self) -> bool {
    match &self {
      Self::Tempo(_) => true,
      _=> false
    }
  }

  fn get_tempo_from(buf : &[u8]) -> Self{
    assert!(buf.len() == 3, "tempo event must be 3 bytes long. But passed '{:X}' instead.", buf[1]);
    
    MetaMessage::Tempo(Tempo(m3byte!(buf)))
  }

  fn get_time_signature_from(buf : &[u8]) -> Self {
    assert!(buf.len() == 4, "time_signature must be 4 bytes long. But passed '{:X}' instead.", buf[1]);

    MetaMessage::TimeSignature (TimeSignature {
      nn : m1byte!(buf[0]),
      dd : m1byte!(buf[1]),
      cc : m1byte!(buf[2]),
      bb : m1byte!(buf[3])
    })
  }

  fn get_key_signature_from(buf : &[u8]) -> Self {
    MetaMessage::KeySignature(KeySignature {
      sf : buf[0].into(),
      mi : buf[1].into()
    })
  }
}



impl From<(u8, u8, &[u8])> for MetaMessage {
  fn from((byte, subtype, rest): (u8, u8, &[u8])) -> Self {
    
    match subtype {
      0x01..=0x07 => Self::Text(TextEvent), 
      0x20 => Self::ChannelPrefix(ChannelPrefix),
      0x2F => Self::EndOfTrack,
      0x21 => Self::MIDIPort(MIDIPort(rest[0].into())),
      0x51 => Self::get_tempo_from(rest),
      0x58 => Self::get_time_signature_from(rest),
      0x59 => Self::get_key_signature_from(rest),
      
      _=> Self::Invalid(format!("From<&[u8]> trait not implemented for Meta-event sub-type with start byte 0x{subtype:X}"))
    }
  }
}

impl From<(u8, &[u8])> for MetaMessage {
  fn from((byte, rest): (u8, &[u8])) -> Self {
    MetaMessage::from((byte, rest[0], &rest[1..]))
  }
}

impl From<&[u8]> for MetaMessage{
  fn from(bytes: &[u8]) -> Self {
    MetaMessage::from((bytes[0], &bytes[1..]))
  }
}