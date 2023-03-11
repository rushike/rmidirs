#![allow(dead_code, unused_variables, unused_must_use, unused_imports, non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::iter::FromIterator;

use crate::primitive::{MXByte, mxbyte};

use super::ByteEncodingFormat;

/// @string process string with `format` specified from `length` number of bytes.
pub fn string(buf: &[u8], format : ByteEncodingFormat) -> &str {
  std::str::from_utf8(buf).unwrap()
}

/// @number process the number from `length` number of bytes.
/// This will return u32 int, since u32 is max what midi support.
pub fn number(buf: &[u8], format : ByteEncodingFormat) -> u32 {
  masked_number(buf, &[0xFF], format)
}

pub fn masked_number(buf : &[u8], mask : &[u8], format : ByteEncodingFormat) -> u32 {
  let mut num :u32 = 0;
  assert!(mask.len() == 1 || mask.len() == buf.len(), "mask should have single length or should be of length of buf. But provided mask : {mask:?} with length : {}", mask.len());
  
  let mask = if mask.len() == 1 {
    mask.repeat(buf.len())
  } else {mask.to_vec()};

  let mut lbits = 0;
  for (byte, MASK) in buf.iter().zip(mask.iter()) {
    let bits = MASK.trailing_ones();
    lbits += bits;
    match format {
      ByteEncodingFormat::BigEndian => num = ( num << bits ) | (byte & MASK) as u32,
      ByteEncodingFormat::LittleEndian => num = num | (((byte & MASK) as u32) << lbits),
    }
  }
  num
}

/// @varnumber process the variable length number from `start` position.
  /// It will panic if number exceed u32 int.
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
pub fn from_var_len(buf: &[u8]) -> MXByte {
  
  let mut num:u32 = 0_u32;
  let mut i = 0;

  while (buf[i] & 0x80) == 0x80 {
    num = (num << 7) | (buf[i] & 0x7F) as u32;
    i += 1;
  } num = (num << 7) | (buf[i] & 0x7F) as u32; 

  mxbyte!(num)
}

/// @masked process `length` number of bytes and masked with `mask`,
fn masked<'a>(buf : &'a [u8], mask : &'a [u8]) -> Vec<u8> {
  assert!(mask.len() == 1 || mask.len() == buf.len(), "mask should have single length or should be of length of buf. But provided mask : {mask:?} with length : {}", mask.len());
  
  let mask = if mask.len() == 1 {
    mask.repeat(buf.len())
  } else {mask.to_vec()};

  let mut out_buf = Vec::with_capacity(buf.len());
  for (i, (b, M)) in buf.iter().zip(mask.iter()).enumerate() {
    out_buf[i] = b & M;
  }
  out_buf       
}

/// @get_bytes_with_cntrl_bit process bytes till `cntrl_bit` is set to `cntrl_val`
/// 
/// # Panics
/// 1. It will panic if `cntrl_bit` > 8.
/// 2. It will panic if `cntrl_val` not in  [0, 1].
fn get_bytes_with_cntrl_bit(buf : &[u8], start : usize, cntrl_bit : u8, cntrl_val : u8 ) -> &[u8] {
  buf
}

