use std::{fs::File, error::Error, string::ParseError, convert::Infallible, io::ErrorKind};

use crate::{
  model::{
    midi_event::{MidiEvent, self, meta_event, channel_event}, 
    midi::Midi
  }, 
  primitive::{
    M1Byte,
    m1byte
  }, errors::MidiParseErrors};

lazy_static::lazy_static!(
  #[derive(Debug)]
  static ref CHANNEL_EVENT_SCHEMA : serde_json::Value = serde_json::de::from_reader(File::open("./src/parser/schema/midi-v1-channel-event-schema.json").expect("File should open read only")).unwrap();
  static ref META_EVENT_SCHEMA : serde_json::Value = serde_json::de::from_reader(File::open("./src/parser/schema/midi-v1-meta-event-schema.json").expect("File should open read only")).unwrap();
);

#[derive(Debug)]
pub struct MidiEventIter<'a>{
  parser : &'a mut MidiEventParser<'a>,
  iter : usize
}

impl<'a> MidiEventIter<'a> {
  pub fn new(parser : &'a mut MidiEventParser<'a>) -> Self{
    Self {
      parser,
      iter: 0
    }
  }
}


impl<'a> Iterator for MidiEventIter<'a> {
  type Item = MidiEvent;
  fn next(&mut self) -> Option<Self::Item> {
    self.iter += 1;
    return self.parser.parse();
  }
}

#[derive(Debug)]
pub struct MidiEventParser<'a> {
  bytes : &'a [u8],
  length : usize,
  track_no : usize,
  last_event : MidiEvent,
}

impl<'a> MidiEventParser<'a> {
  pub fn new(bytes : &'a [u8], length : usize) -> MidiEventParser<'a> {
    MidiEventParser {
      bytes,
      length,
      track_no : 0,
      last_event : MidiEvent::default()
    }
  }
  pub fn iter(&'a mut self) -> MidiEventIter<'a> {
    return MidiEventIter::new(self);
  }
  fn parse(& mut self) -> Option<MidiEvent> {
    let mut buf = self.bytes;
    if buf.len() < 4 {
      return None;
    }
    let (delta_time, ptr) = Self::parse_delta_time_with_ptr(&buf[..4]);
    
    let mut ebyte = buf[ptr];

    buf = &buf[ptr..];

    let end = if buf.len() > 10 {10} else {buf.len()};
    // println!("track_no : {}, ptr : {ptr}, buf head : {:?}", self.track_no, &buf[..end]);
    // println!("event iter : {:?}, ptr : {:?}, curr_buf : {:?}", event_iter.last_event, ptr, buf);
  
    let ptr =  match MidiEvent::event_type(ebyte) {
        "CHANNEL_EVENT" => MidiEventParser::parse_channel_event(buf[0]),

        "META_EVENT" => MidiEventParser::parse_meta_event(buf),

        "SYS_EVENT" => MidiEventParser::parse_sys_event(buf),
        
        _ => {
          if self.last_event.is_channel_event() {
            ebyte = self.last_event.event_byte();
            // println!("!!! Exception Ebyte: {:?}, bytes : {:?}", ebyte, &self.bytes[..end]);
            MidiEventParser::parse_channel_event(ebyte) - 1
          } else {
            panic!("Can't parse MIDI event. Unexpected MIDI event byte in track : {:?}, passed 0x{:0X}", self.track_no, buf[0]) 
          }
        }
      };

    let midi_event = MidiEvent::from((ebyte, &buf[..ptr]));
    
    buf = &buf[ptr..];

    self.bytes = buf;

    // println!("ptr : {ptr}, {:?}", midi_event);

    self.last_event = midi_event.clone(); // MidiEvent::default();
    
    self.track_no += 1;

    return Some(midi_event);
  }

  fn parse_channel_event(byte : u8) -> usize {
    let channel_event_type = (byte & 0xF0) >> 4;
    let channel = byte & 0x0F;
    // println!("channel : {:?}, channel_event_type : {:?}, format : {:?}", channel, channel_event_type, format!("0x{:X}", channel_event_type));
    let event_info = &CHANNEL_EVENT_SCHEMA["info"][format!("0x{:X}", channel_event_type)];
    // println!("event_info : {:?}", event_info);
    let length = event_info["length"].as_u64().unwrap() as usize;
    length + 1
  }

  fn parse_meta_event(buf : &[u8]) -> usize {
    // let meta_event_subtype_info = &META_EVENT_SCHEMA["0xFF"];
    let meta_event_byte = &buf[0];
    let meta_event_subtype_byte = &buf[1];
    let end = if buf.len() < 6 { buf.len() } else { 6 };
    let (length, ptr) = Self::parse_var_len(&buf[2..end]);
    // println!("meta_event_byte: {:?}, meta_event_subtype_byte: {:?}, length: {}, ptr : {:?}", meta_event_byte, meta_event_subtype_byte, length, ptr);
    2 + length as usize + ptr
  }

  fn parse_sys_event(buf : &[u8]) -> usize {
    0
  }

  fn parse_delta_time_with_ptr(buf : &[u8]) -> (u32, usize) {
    assert!(buf.len() <= 4, "delta time must be <= 4 bytes. But passed: {} number of bytes. bytes are : {:?} ", buf.len(), buf);
    
    Self::parse_var_len(buf)
  }

  fn parse_var_len(buf : &[u8]) -> (u32, usize) {
    assert!(buf.len() <= 4, "var length must be <= 4 bytes. But passed: {} number of bytes", buf.len());
    
    let mut num: u32 = 0;
    let mut i = 0;
    while (buf[i] & 0x80) == 0x80 {
      num += (buf[i] & 0xFF) as u32;
      i += 1;
    } num += (buf[i] & 0x7F) as u32; 

    return (num, i + 1);
  }
}