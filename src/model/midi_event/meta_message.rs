use crate::{primitive::{M3Byte, M1Byte, m1byte, m3byte, m1bit, M1Bit}, };

#[derive(Debug, Clone)]
pub struct TextEvent;

#[derive(Debug, Clone)]
pub struct ChannelPrefix;

#[derive(Debug, Clone)]
pub struct EndOfTrack;

#[derive(Debug, Clone)]
pub struct Tempo(M3Byte);

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
  mi : M1Bit
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
  EndOfTrack = 0x2F,
  Tempo(Tempo) = 0x51,
  SMPTEOffset = 0x54,
  TimeSignature(TimeSignature) = 0x58,
  KeySignature (KeySignature) = 0x59,
  Invalid(String)
}

impl MetaMessage {

  pub fn is_tempo_event(&self) -> bool {
    match &self {
      Self::Tempo(_) => true,
      _=> false
    }
  }

  fn get_tempo_from(buf : &[u8]) -> Self{
    assert!(buf.len() >= 5, "input slice passsed should be >= 5 bytes long. But passed input slice with {} length", buf.len());
    // assert!(buf[0] == 0x51, "tempo event should start with 0x51 byte. But passed {:X}", buf[0]);
    assert!(buf[1] == 3, "tempo event must be 3 bytes long. But passed '{:X}' instead.", buf[1]);
    
    MetaMessage::Tempo(Tempo(m3byte!(&buf[2..5])))
  }

  fn get_time_signature_from(buf : &[u8]) -> Self {
    assert!(buf.len() >= 6, "input slice passsed for 'time_signature' should be >= 6 bytes long. But passed input slice with {} length", buf.len());

    assert!(buf[1] == 4, "tempo event must be 4 bytes long. But passed '{:X}' instead.", buf[1]);

    MetaMessage::TimeSignature (TimeSignature {
      nn : m1byte!(buf[0]),
      dd : m1byte!(buf[1]),
      cc : m1byte!(buf[2]),
      bb : m1byte!(buf[3])
    })
  }

  fn get_key_signature_from(buf : &[u8]) -> Self {
    MetaMessage::KeySignature(KeySignature {
      sf : m1byte!(buf[0]),
      mi : m1bit!(buf[1])
    })
  }
}



impl From<(u8, &[u8])> for MetaMessage {
  fn from((byte, rest): (u8, &[u8])) -> Self {
    let subtype = rest[0];
    
    match subtype {
      0x01..=0x07 => Self::Text(TextEvent), 
      0x20 => Self::ChannelPrefix(ChannelPrefix),
      0x2F => Self::EndOfTrack,

      0x51 => Self::get_tempo_from(rest),
      0x58 => Self::get_time_signature_from(rest),
      0x59 => Self::get_key_signature_from(rest),
      
      _=> Self::Invalid(format!("From<&[u8]> trait not implemented for Meta-event sub-type with start byte {subtype:X}"))
    }
  }
}

impl From<&[u8]> for MetaMessage{
  fn from(bytes: &[u8]) -> Self {
    MetaMessage::from((bytes[0], &bytes[1..]))
  }
}