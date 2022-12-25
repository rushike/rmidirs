use super::{MidiFileReader, buffer::Buffer};

impl<'a> MidiFileReader<'a>  {
  pub fn web(path : &str) -> MidiFileReader {
    todo!()
  }
}

pub struct Web<'a>(Buffer<'a>);