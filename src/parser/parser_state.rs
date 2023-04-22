use core::fmt;

use crate::primitive::MXByte;

/// Parser State stores the ptr to position where parser is running currently
#[derive(Debug, Clone)]
pub struct ParserState {
  name: String,
  curr : usize,
  start: usize,
  end: usize,
}

impl<'a> ParserState {
  pub fn new(name : String, start : usize, end : usize) -> ParserState {
    return ParserState { name, start, curr : start, end};
  }

  pub fn with_name(&self, name : String) -> ParserState { 
    return ParserState { name, start : self.start, curr : self.curr, end : self.end};
  }

  /// moves the current position 'forward' bytes
  pub fn forward(&mut self, forward : usize) {
    self.curr += forward;
  }

  /// moves the current position 'back' bytes
  pub fn back(&mut self, back : usize) {
    self.curr += back;
  }

  /// Returns the next 'len' bytes from the current position and moves the current position
  pub fn next(&mut self, buf : &'a [u8], len : usize) -> &'a [u8] {
    self.curr += len;
    &buf[self.curr - len .. self.curr]
  }
  /// returns the next 'len' bytes from the current position
  pub fn take(&self, buf : &'a [u8], len : usize) -> &'a [u8] {
    &buf[self.curr .. self.curr + len]
  }

  /// returns the prev 'len' bytes from the current position
  pub fn retake(&self, buf : &'a [u8], len : usize) -> &'a [u8] {
    &buf[self.curr - len .. self.curr]
  }

  /// return curr byte from buffer
  pub fn byte(&self, buf : &'a [u8]) -> u8 {
    buf[self.curr]
  }

  /// returns the mxbyte from current position in buffer, and moved current position accordingly.
  pub fn mxbyte(&mut self, buf : &'a [u8]) -> MXByte {
    let mxbyte = MXByte::from(&buf[self.curr .. ]);
    self.curr += mxbyte.len();
    mxbyte
  }
  pub fn name(&self) -> String { self.name.to_string() }
  
  pub fn curr(&self) -> usize { self.curr }

  pub fn start(&self) -> usize { self.start }

  pub fn end(&self) -> usize { self.end }

}

impl fmt::Display for ParserState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Parser State ( name : {}, curr : {} )", self.name, self.curr)
  }
}
