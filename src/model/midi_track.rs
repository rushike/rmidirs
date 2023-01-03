#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]

use crate::{primitive::{M4Byte, M2Byte, m2byte, m4byte}, };

use super::{midi_event::MidiEvent, timeline::Timeline, midi_header::MidiHeader};

#[derive(Debug, Clone)]
pub struct MidiTrack {
  pub(crate) header : String,
  pub(crate) length : M4Byte,
  pub(crate) events : Vec<MidiEvent>,
  pub(crate) time_div : M2Byte,
}

impl Default for MidiTrack {
    fn default() -> Self {
        Self { 
          header: "MTrk".to_string(), 
          length: m4byte!(0), 
          events: Vec::new(), 
          time_div : m2byte!(540)
        }
    }
}

impl From<(&MidiHeader, u32)> for MidiTrack {
  fn from((midi_header, length): (&MidiHeader, u32)) -> Self {
    Self{
      header: "MTrk".to_string(), 
      length : m4byte!(length), 
      events: Vec::new(),
      time_div : midi_header
                    .division
                    .metric_time()
                    .unwrap_or(m2byte!(540))
    }
  }
}
impl MidiTrack {
  /// Create a new MidiTrack
  pub fn new(length : u32) -> Self {
    Self{
      header: "MTrk".to_string(), 
      length : m4byte!(length), 
      events: Vec::new(),
      time_div : m2byte!(540)
    }
  }
  /// Create a new MidiTrackIter
  pub fn iter(&self) -> MidiTrackIter {
    MidiTrackIter { events: &self.events, top : 0 }
  }

  pub fn timeline(&self) -> Timeline {
    Timeline::from(self)
  }

  /// Add MidiEvent at end off track
  pub fn add_event(&mut self, event : MidiEvent) {
    self.events.push(event);
    self.length = m4byte!(*self.length + 1); //Todo: Add Assign might be the way
  }
}

pub struct MidiTrackIter<'a> {
  events : &'a [MidiEvent],
  top : usize
}


impl<'a> Iterator for MidiTrackIter<'a> {
  type Item = MidiEvent;

  fn next(&mut self) -> Option<Self::Item> {
    self.top += 1;
    if self.top >= self.events.len() {
      None
    } else {
      Some(self.events[self.top - 1].clone())
    }
  }
}

