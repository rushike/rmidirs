use std::ops::{Deref, BitAnd};

pub type Word = u32;

#[derive(Debug, Clone, Copy)]
pub struct M3Byte(u32);


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

macro_rules! impl_from_for_mtypes{
  ($t_in : tt, $t_out : tt) => {
    impl From<$t_in> for $crate::primitive::$t_out {
      fn from(word: $t_in) -> Self {
        $crate::primitive::$t_out((word & 0xF) as u32)
      }
    }              
  };
}

#[macro_export]
macro_rules!  impl_midi_dtypes{
    ($m : ident, $t : tt) => {
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

      impl_from_for_mtypes!(u8, $t);
      impl_from_for_mtypes!(u16, $t);
      impl_from_for_mtypes!(u32, $t);
      impl_from_for_mtypes!(u64, $t);
      impl_from_for_mtypes!(u128, $t);
      impl_from_for_mtypes!(usize, $t);
      impl_from_for_mtypes!(i8, $t);
      impl_from_for_mtypes!(i16, $t);
      impl_from_for_mtypes!(i32, $t);
      impl_from_for_mtypes!(i64, $t);
      impl_from_for_mtypes!(i128, $t);
      impl_from_for_mtypes!(isize, $t);
      
    };
}

impl_midi_dtypes!(m3byte, M3Byte);
impl_midi_dtypes!(m2byte, M2Byte);
impl_midi_dtypes!(m1byte, M1Byte);
impl_midi_dtypes!(m4bits, M4Bits);

pub use {m1byte, m2byte, m3byte, m4bits};

// #[macro_export]
// macro_rules! m3byte {
//   ($num : expr) => {
//     M3Byte::from($num)
//     // M3Byte($num)
//   }
// }