use crate::{model::midi::Midi, reader::{Reader, buffer::Buffer}};

use super::{midi_header_parser::MidiHeaderParser, Parser, midi_track_parser::MidiTrackParser};

pub struct MidiParser;

impl MidiParser {
    pub fn parse(buf : &[u8]) -> Midi {
      let length = buf.len();
      let mut midi = Midi::default();
      let ptr = MidiHeaderParser::parse(buf, &mut midi);

      let mut track_parser = MidiTrackParser::new(&buf[ptr..], midi.header().clone());

     
      for (i, track) in track_parser.iter().enumerate() {
        // println!("pushing {:?}", i);
        midi.add_track(track);
        // break;
      }
      // let ptr = MidiTrackParser::parse(1, &buf[ptr..], &mut midi);

      // assert!(ptr == length, "Can't parse entire file buffer. File buffer length :{length}, parse till {ptr}");
      
      midi
      
    }


    
}