use std::{marker::PhantomData};

use crate::{
  utils::{ByteEncodingFormat}, 
  model::core::midi::Midi, parser::MidiParser
};

use super::{FileSrcType, MidiFileReader, Reader};

impl<'a> MidiFileReader<'a>{
  pub fn buffer(contents : &'a [u8]) -> Buffer<'a> {
   Buffer::from_buf(contents)
  }
}

#[derive(Debug, Clone)]
pub struct Buffer<'a> {
  src : FileSrcType,
  path : Option<&'a str>,
  contents : Vec<u8>,
  length : usize,
  iter : usize,
}

impl<'a> Buffer<'a> {
  pub fn len(&self) -> usize {
    self.length
  }

  pub fn from_buf(contents : &'a [u8]) -> Self {
    Self {
      src : FileSrcType::Buffer,
      path : None,
      length : contents.len(),
      contents : contents.to_vec(),
      iter : 0,
    }
  }
  pub fn from_local(path : &'a str, contents : Vec<u8>, length : usize) -> Self {
    Self {
      src : FileSrcType::Local,
      path : Some(path),
      contents,
      length,
      iter: 0,
    }
  }

  pub fn from_web(path : &'a str, contents : Vec<u8>, length : usize) -> Buffer<'a> {
    Buffer {
      src : FileSrcType::Web,
      path : Some(path),
      contents,
      length,
      iter: 0,
    }
  }
}


impl<'a> Reader for Buffer<'a> {
  fn bytes(&self) -> &[u8] {
    &self.contents
  }

  fn parse(&self) -> Midi {
      MidiParser::parse(&self.contents).unwrap()
  }
}