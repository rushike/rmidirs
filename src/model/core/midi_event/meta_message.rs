use crate::primitive::{M3Byte, M1Byte, m1byte, m3byte, m1bit, M1Bit, Word, MNBits};

use super::MidiMessage;

#[derive(Debug, Clone)]
pub struct TextEvent;

#[derive(Debug, Clone)]
pub struct ChannelPrefix;

#[derive(Debug, Clone)]
pub struct MIDIPort(M1Byte);

impl From<MIDIPort> for Vec<u8> {
  fn from(midi_port: MIDIPort) -> Self {
    midi_port.0.into()
  }
}

#[derive(Debug, Clone)]
pub struct EndOfTrack;


/// Tempo Event in MIDI.v1
/// 
/// In MIDI.v1 tempo event is stored as 3 bytes (Big Endian format). 
///
/// rmidirs stores it in M3Byte, which is u32, but with 3 byte mask. 
#[derive(Debug, Clone, Copy)]
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

  pub fn bpm(&self) -> u32 {
    ((60.0 * 1000_000.0) / *self.0 as f32 ).floor() as u32
  }
}

impl From<Tempo> for Vec<u8> {
  fn from(tempo: Tempo) -> Self {
    tempo.0.into()
  }
}

impl Default for Tempo {
  fn default() -> Self {
    Self(500_000.into())
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

impl From<&[u8]> for TimeSignature {
  fn from(buf: &[u8]) -> Self {
    assert!(buf.len() == 4, "time_signature must be 4 bytes long. But passed '{:X}' instead.", buf[1]);

    TimeSignature {
      nn : m1byte!(buf[0]),
      dd : m1byte!(buf[1]),
      cc : m1byte!(buf[2]),
      bb : m1byte!(buf[3])
    }
  }
}

impl From<TimeSignature> for Vec<u8> {
  fn from(time_signature: TimeSignature) -> Self {
    vec![time_signature.nn.into(), time_signature.dd.into(), time_signature.cc.into(), time_signature.bb.into()]
  }
}

/// Key Signature stores the key / scala of staff of music sheet
/// 
/// KeySignature stores the information in two 1 byte variable
/// - sf               : sf > 0 no of sharp keys<br>
/// &nbsp;&nbsp;&nbsp; : sf < 0 no of flat keys if sf is negative
/// - mi : minor(1) or major (0)
#[derive(Debug, Clone)]
pub struct KeySignature {
  sf : M1Byte,
  mi : M1Byte
}

impl From<&[u8]> for KeySignature {
  fn from(buf: &[u8]) -> Self {
    KeySignature {
      sf : buf[0].into(),
      mi : buf[1].into()
    }
  }
}

impl From<KeySignature> for Vec<u8> { 
  fn from(key_signature: KeySignature) -> Self {
    vec![key_signature.sf.into(), key_signature.mi.into()]
  }
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
    
    Self::Tempo(Tempo(m3byte!(buf)))
  }

  fn get_time_signature_from(buf : &[u8]) -> Self {
    Self::TimeSignature (TimeSignature::from(buf))
  }

  fn get_key_signature_from(buf : &[u8]) -> Self {
    Self::KeySignature(KeySignature::from(buf))
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

impl From<MetaMessage> for Vec<u8> {
  fn from(meta_message: MetaMessage) -> Self {
    match meta_message {
      MetaMessage::Text(_) => todo!(),
      MetaMessage::CopyrightNotice(_) => todo!(),
      MetaMessage::TrackName(_) => todo!(),
      MetaMessage::InstrumentName(_) => todo!(),
      MetaMessage::Lyrics(_) => todo!(),
      MetaMessage::Marker(_) => todo!(),
      MetaMessage::CuePoint(_) => todo!(),
      MetaMessage::ChannelPrefix(_) => todo!(),
      MetaMessage::MIDIPort(midi_port) => midi_port.into(),
      MetaMessage::EndOfTrack => vec![0x2F],
      MetaMessage::Tempo(tempo) => tempo.into(),
      MetaMessage::SMPTEOffset => todo!(),
      MetaMessage::TimeSignature(time_signature) => time_signature.into(),
      MetaMessage::KeySignature(key_signature) => key_signature.into(),
      MetaMessage::Invalid(_) => todo!(),
      }
  }
}