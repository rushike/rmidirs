use std::{fs::File, io::Read, ops::Deref, marker::PhantomData};

use super::{MidiFileReader, FileSrcType, buffer::Buffer};

impl<'a> MidiFileReader<'a>  {
  pub fn local(path : &'a str) -> Local<'a> {
    assert!(!path.starts_with("http://") && !path.starts_with("https://"));

    let mut file =File::open(path).unwrap();
    
    let mut contents: Vec<u8> = Vec::new();

    let length = file.read_to_end( &mut contents).unwrap();
    // (length, contents)
    Local(Buffer::from_local(path, contents, length))
  }
}

impl<'a> Local<'a> {
    pub fn from_local(path : &'a str, contents : Vec<u8>, length : usize) -> Self {
      Self(
        Buffer::from_local(path, contents, length)
      )
    }
}

#[derive(Debug)]
pub struct Local<'a>(Buffer<'a>);

impl<'a> Deref for Local<'a>{ 
  type Target = Buffer<'a>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}