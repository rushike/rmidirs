use std::ops::{Deref, BitAnd};

use crate::utils::{functions::number, ByteEncodingFormat};

pub type Word = u32;

#[derive(Debug, Clone, Copy)]
pub struct M3Byte(u32);

impl From<&[u8]>  for M3Byte {
    fn from(buf: &[u8]) -> Self {
      assert!(buf.len() >= 3, "exptected the input buffer to have at least 3 bytes. But passed buffer with {} length", buf.len());
      M3Byte(number(buf, ByteEncodingFormat::BigEndian))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct M2Byte(u32);


#[derive(Debug, Clone, Copy)]
pub struct M1Byte(u32);


#[derive(Debug, Clone, Copy)]
pub struct M4Bits(u32);

impl Into<u8> for M4Bits {
  fn into(self) -> u8 {
      self.0 as u8
  }
}

#[derive(Debug, Clone, Copy)]
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

impl_midi_dtypes!(m3byte, M3Byte, 0xFF);
impl_midi_dtypes!(m2byte, M2Byte, 0xFF);
impl_midi_dtypes!(m1byte, M1Byte, 0xFF);
impl_midi_dtypes!(m4bits, M4Bits, 0xF);
impl_midi_dtypes!(m1bit, M1Bit, 0x1);

pub use {m1byte, m2byte, m3byte, m4bits, m1bit};

// #[macro_export]
// macro_rules! m3byte {
//   ($num : expr) => {
//     M3Byte::from($num)
//     // M3Byte($num)
//   }
// }