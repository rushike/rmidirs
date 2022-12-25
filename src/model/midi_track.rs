#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]

use super::{midi_event::MidiEvent, timeline::Timeline};


#[derive(Debug, Clone)]
pub struct MidiTrack {
  header : String,
  length : u32,
  events : Vec<MidiEvent>,
  timeline : Timeline,
}

impl Default for MidiTrack {
    fn default() -> Self {
        Self { 
          header: "MTrk".to_string(), 
          length: 0, 
          events: Vec::new(), 
          timeline: Default::default() 
        }
    }
}

impl MidiTrack {
  pub fn new(length : u32) -> Self {
    Self{
      header: "MTrk".to_string(), 
      length, 
      events: Vec::new(), 
      timeline: Default::default() 
    }
  }
  pub fn add_event(&mut self, event : MidiEvent) {
    self.events.push(event)
  }
}