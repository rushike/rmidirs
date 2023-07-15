#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]

use std::marker::PhantomData;

use crate::{
  utils::ByteEncodingFormat, 
  model::core::midi::Midi
};

use self::{buffer::Buffer};

pub mod buffer;
pub mod local;
pub(crate) mod web;


#[derive(Debug, Clone)]
pub enum FileSrcType {
    Local,
    Web,
    Buffer
}

///
/// Args stores all input arguments passed to Reader instance.
/// 
#[derive(Debug, Clone)]
pub struct MidiFileReader<'a> {
  file_src_type : FileSrcType,
  path : Option<&'a str>,
  length : Option<usize>,
  contents : Option<Vec<u8>>,
}


/// Reader trait defines behavior common to all readers.
/// 
/// 
pub trait Reader {
  fn bytes(&self) -> &[u8];

  fn parse(&self) -> Midi;
}



