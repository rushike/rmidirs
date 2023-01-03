use core::time;

use crate::{primitive::{MXByte, mxbyte, M2Byte, M3Byte}};

#[derive(Debug, Clone)]
pub struct DeltaTime(MXByte);

impl  DeltaTime {
  pub fn to_microseconds(&self, time_div : M2Byte, tempo : M3Byte) -> f32 {
    (((*self.0 as f32 * 16.0 / *time_div as f32).ceil() / 2.0).floor() * *tempo as f32) / 8.0
  }

  pub fn to_milliseconds(&self, time_div : M2Byte, tempo : M3Byte) -> f32 {
    self.to_microseconds(time_div, tempo) / 1000.0
  }

  pub fn to_seconds(&self, time_div : M2Byte, tempo : M3Byte) -> f32 {
    self.to_microseconds(time_div, tempo) / 1000000.0
  }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(mxbyte!(0))
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