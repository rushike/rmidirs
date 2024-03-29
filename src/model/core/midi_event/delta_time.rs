use core::time;

use crate::primitive::{MXByte, mxbyte, M2Byte, M3Byte};

use super::meta_message::Tempo;

/// Deltatime stores the variable time used before every MIDI Events. 
/// It is not stored as seconds, it unit complete depends on Midi Header,
/// Metric Version / SMPTE resolution byte in Midi Header.
#[derive(Debug, Clone)]
pub struct DeltaTime(MXByte);

impl  DeltaTime {
  pub fn len(&self) -> usize {self.0.len()}

  pub fn to_microseconds(&self, time_div : u32, tempo : f32) -> f32 {
    (((*self.0 as f32 * 16.0 / time_div as f32).ceil() / 2.0).floor() * tempo) / 8.0
  }

  pub fn to_milliseconds(&self, time_div : u32, tempo : f32) -> f32 {
    self.to_microseconds(time_div, tempo) / 1000.0
  }

  pub fn to_seconds(&self, time_div : u32, tempo : f32) -> f32 {
    self.to_microseconds(time_div, tempo) / 1000000.0
  }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(mxbyte!(0))
    }
}

impl From<&[u8]> for DeltaTime {
  fn from(buf: &[u8]) -> Self {
    Self(MXByte::from(buf))
  }
}

impl From<u32> for DeltaTime {
  fn from(delta_time: u32) -> Self {
    Self::from(mxbyte!(delta_time))
  }
}

impl From<MXByte> for DeltaTime {
  fn from(delta_time: MXByte) -> Self {
    DeltaTime(delta_time)
  }
}

impl From<DeltaTime> for Vec<u8> {
  fn from(delta_time: DeltaTime) -> Self {
    delta_time.0.into()
  }
}