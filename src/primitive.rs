

use std::ops::{Deref, BitAnd};

pub type FractionWord = (u32, u32);

pub type FloatWord = f32;

pub type Word = u32;

pub type DoubleWord = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M4Byte(Word);

impl From<&[u8]>  for M4Byte {
    fn from(buf: &[u8]) -> Self {
      assert!(buf.len() >= 4, "exptected the input buffer to have at least 4 bytes. But passed buffer with {} length", buf.len());
      M4Byte(Word::from_be_bytes(buf.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M3Byte(Word);

impl From<&[u8]>  for M3Byte {
    fn from(buf: &[u8]) -> Self {
      assert!(buf.len() >= 3, "exptected the input buffer to have at least 3 bytes. But passed buffer with {} length", buf.len());
      M3Byte((buf[0] as Word) << 16 | (buf[1] as Word) << 8 | buf[2] as Word)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M2Byte(Word);

impl From<&[u8]>  for M2Byte {
  fn from(buf: &[u8]) -> Self {
    assert!(buf.len() >= 2, "exptected the input buffer to have at least 2 bytes. But passed buffer with {} length", buf.len());
    M2Byte((buf[0] as Word) << 8 | buf[1] as Word)
  }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M1Byte(Word);


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M4Bits(Word);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M1Bit(Word);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MNBits(Word, Word);

impl Deref for MNBits {
  type Target = Word;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<Word> for MNBits {
  fn from(word: Word) -> Self {
    if word == 0 {return Self(0, 1)}
    Self(word, (word as f32).log2() as Word)
  }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MXByte(Word, usize);

impl MXByte {
    pub fn len(&self) -> usize {self.1}
}

impl From<Word> for MXByte {
  fn from(word: Word) -> Self {
    if word == 0 {return Self(0, 1)}
    MXByte(word, ((word as f64).log2() / 7.0 + 1.0) as usize)
  }
}

impl  From<&[u8]> for MXByte {
  /// @varnumber process the variable length number from `start` position.
  /// 
  /// ## Midi Var Number Format
  /// Strategy used is based on delta time encoding in MIDI messages
  /// Last 7 bits in each byte will carry info, 
  /// 1 bits of every byte is set to 1 expect 1 bit of last byte is set to 0
  /// 
  /// e.g. 
  /// 
  /// | 8 bytes number| Variable Length encoding |
  /// |---------------|--------------------------|
  /// | 00000040      |    40                    |
  /// | 0000007F	    |    7F                    |
  /// | 00000080	    |    81 00                 |
  /// | 00002000	    |    C0 00                 |
  /// | 00003FFF	    |    FF 7F                 |
  /// | 00004000	    |    81 80 00              |
  /// | 00100000	    |    C0 80 00              |
  /// | 001FFFFF	    |    FF FF 7F              |
  /// | 00200000	    |    81 80 80 00           |
  /// 
  fn from(buf: &[u8]) -> MXByte {

    let mut num:Word = 0;
    let mut i = 0;
  
    while (buf[i] & 0x80) == 0x80 {
      num = (num << 7) | (buf[i] & 0x7F) as Word;
      i += 1;
    } num = (num << 7) | (buf[i] & 0x7F) as Word; 
  
    mxbyte!(num)
  }
}
  

impl Deref for MXByte {
  type Target = Word;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
#[macro_export]
macro_rules! mxbyte {
  ($num : expr) => {
    MXByte::from($num as u32)
  }
}

#[macro_export]
macro_rules! impl_from_for_mtypes{
  ($t_in : tt, $t_out : tt, $mask : literal) => {
    impl From<$t_in> for $crate::primitive::$t_out {
      fn from(word: $t_in) -> Self {
        $crate::primitive::$t_out((word as u32) & $mask)
      }
    }
    
    impl From<$crate::primitive::$t_out> for $t_in  {
      fn from(word: $crate::primitive::$t_out) -> Self {
        (word.0 & $mask) as $t_in
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

// impl_midi_dtypes!(mxbyte, MXByte, 0xFFFFFFFF); // here mask makes no sense, so kept max possible
impl_midi_dtypes!(m4byte, M4Byte, 0xFFFFFFFF);
impl_midi_dtypes!(m3byte, M3Byte, 0xFFFFFF);
impl_midi_dtypes!(m2byte, M2Byte, 0xFFFF);
impl_midi_dtypes!(m1byte, M1Byte, 0xFF);
impl_midi_dtypes!(m4bits, M4Bits, 0xF);
impl_midi_dtypes!(m1bit, M1Bit, 0x1);

pub use {mxbyte, m4byte, m3byte, m2byte, m1byte, m4bits, m1bit};
