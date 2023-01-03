use std::ops::{Deref, BitAnd};

use crate::utils::{functions::{number, from_var_len}, ByteEncodingFormat};

pub type Word = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MXByte(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M4Byte(u32);

impl From<&[u8]>  for M4Byte {
    fn from(buf: &[u8]) -> Self {
      assert!(buf.len() >= 4, "exptected the input buffer to have at least 4 bytes. But passed buffer with {} length", buf.len());
      M4Byte(u32::from_be_bytes(buf.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M3Byte(u32);

impl From<&[u8]>  for M3Byte {
    fn from(buf: &[u8]) -> Self {
      assert!(buf.len() >= 3, "exptected the input buffer to have at least 3 bytes. But passed buffer with {} length", buf.len());
      M3Byte((buf[0] as u32) << 16 | (buf[1] as u32) << 8 | buf[2] as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M2Byte(u32);

impl From<&[u8]>  for M2Byte {
  fn from(buf: &[u8]) -> Self {
    assert!(buf.len() >= 2, "exptected the input buffer to have at least 2 bytes. But passed buffer with {} length", buf.len());
    M2Byte((buf[0] as u32) << 8 | buf[1] as u32)
  }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M1Byte(u32);


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M4Bits(u32);

impl Into<u8> for M4Bits {
  fn into(self) -> u8 {
      self.0 as u8
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M1Bit(u32);

impl Into<u8> for M1Bit {
  fn into(self) -> u8 {
      self.0 as u8
  }
}


macro_rules! impl_from_for_mtypes{
  ($t_in : tt, $t_out : tt, $mask : literal) => {
    impl From<$t_in> for $crate::primitive::$t_out {
      fn from(word: $t_in) -> Self {
        $crate::primitive::$t_out((word as u32) & $mask)
      }
    }              
  };
}

#[macro_export]
macro_rules!  impl_midi_dtypes{
    ($m : ident, $t : tt, $mask : literal) => {
      impl Deref for $t {
        type Target = Word;
    
        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }
      #[macro_export]
      macro_rules! $m {
        ($num : expr) => {
          $crate::primitive::$t::from($num)
        }
      }

      impl_from_for_mtypes!(u8, $t, $mask);
      impl_from_for_mtypes!(u16, $t, $mask);
      impl_from_for_mtypes!(u32, $t, $mask);
      impl_from_for_mtypes!(u64, $t, $mask);
      impl_from_for_mtypes!(u128, $t, $mask);
      impl_from_for_mtypes!(usize, $t, $mask);
      impl_from_for_mtypes!(i8, $t, $mask);
      impl_from_for_mtypes!(i16, $t, $mask);
      impl_from_for_mtypes!(i32, $t, $mask);
      impl_from_for_mtypes!(i64, $t, $mask);
      impl_from_for_mtypes!(i128, $t, $mask);
      impl_from_for_mtypes!(isize, $t, $mask);
      
    };
}

impl_midi_dtypes!(mxbyte, MXByte, 0xFFFFFFFF); // here mask makes no sense, so kept max possible
impl_midi_dtypes!(m4byte, M4Byte, 0xFFFFFFFF);
impl_midi_dtypes!(m3byte, M3Byte, 0xFFFFFF);
impl_midi_dtypes!(m2byte, M2Byte, 0xFFFF);
impl_midi_dtypes!(m1byte, M1Byte, 0xFF);
impl_midi_dtypes!(m4bits, M4Bits, 0xF);
impl_midi_dtypes!(m1bit, M1Bit, 0x1);

pub use {mxbyte, m4byte, m3byte, m2byte, m1byte, m4bits, m1bit};
