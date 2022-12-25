
use std::default;

use crate::{model::{midi::Midi, midi_track::MidiTrack, midi_header::MidiHeader}, utils::{ByteEncodingFormat, functions::number}, parser::midi_event_parser::{MidiEventIter, MidiEventParser}};

use super::Parser;

pub struct MidiTrackIter<'a>{
  parser : &'a mut MidiTrackParser<'a>, 
  iter : usize,
}
impl<'a> MidiTrackIter<'a> {
  fn new(parser : &'a mut MidiTrackParser<'a>) -> MidiTrackIter<'a> {
    return Self {
      iter : 0,
      parser,
    };
  }
}

impl<'a> Iterator for MidiTrackIter<'a> {
    type Item = MidiTrack;

    fn next(&mut self) -> Option<Self::Item> {
      self.iter += 1;
      return self.parser.parse();
    }
}

pub struct MidiTrackParser<'a> {
  bytes : &'a [u8],
  midi_header : MidiHeader,
}

impl<'a> MidiTrackParser<'a> {

  pub fn new(bytes : &'a [u8], midi_header : MidiHeader) -> MidiTrackParser<'a> {
    MidiTrackParser { 
      bytes, 
      midi_header 
    }
  }

  pub fn iter(&'a mut self) -> MidiTrackIter<'a> {
    return MidiTrackIter::new(self);
  }

  /// Parses a MIDI track. And move the slice pointer forward.
  fn parse(&mut self) -> Option<MidiTrack> {
    if self.bytes.len() < 8 {return None}

    const ENC_FORMAT: ByteEncodingFormat = ByteEncodingFormat::BigEndian;

    let buf = self.bytes;
    
    match &buf[..4] {
      b"MTrk" => {
        let length = number(&buf[4..8], ENC_FORMAT);
        let total_length = 8 + length as usize;
        // println!("Track length: {} & total length : {:?}", length, total_length);
        let mut track = MidiTrack::new(length);

        let mut event_parser = MidiEventParser::new(&buf[8..total_length], length as usize);
        
        for event in event_parser.iter() {
          track.add_event(event);
        }

        self.bytes = &buf[total_length..];

        return Some(track);
      }
      // _ => return None
      _header => panic!("Midi Track header should start with 'MTrk'. But was got {_header:?}.")
    }
  }
}