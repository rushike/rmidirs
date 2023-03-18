#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]

use std::ops::Add;

use crate::{primitive::{M4Byte, M2Byte, m2byte, m4byte, m3byte} };

use super::{midi_event::{MidiEvent, meta_message::Tempo, AbsoluteMidiEvent}, midi_header::MidiHeader};

#[derive(Debug, Clone)]
pub struct MidiTrack {
  pub(crate) events : Vec<MidiEvent>,
  pub(crate) time_div : M2Byte,
}

impl Default for MidiTrack {
    fn default() -> Self {
        Self { 
          events: Vec::new(), 
          time_div : m2byte!(540)
        }
    }
}

impl From<(&MidiHeader, u32)> for MidiTrack {
  fn from((midi_header, length): (&MidiHeader, u32)) -> Self {
    Self{
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
      events: Vec::new(),
      time_div : m2byte!(540)
    }
  }
  /// Create a new MidiTrackIter
  pub fn iter(&self) -> MidiTrackIter {
    MidiTrackIter { events: &self.events, top : 0 }
  }

  /// Add MidiEvent at end off track
  pub fn add_event(&mut self, event : MidiEvent) {
    self.events.push(event);
  }
}

pub struct MidiTrackIter<'a> {
  events : &'a [MidiEvent],
  top : usize
}

impl<'a> MidiTrackIter<'a> {
    pub fn get_tempo(&self) -> Option<Tempo> {
      let tempo =  self.events[self.top ..]
        .iter()
        .find(|event| event.get_tempo().is_some());
      
      match tempo {
            Some(event) => event.get_tempo(),
            None => None,
        }
    }
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

#[derive(Debug, Clone)]
pub struct AbsoluteMidiTrack {
  events : Vec<AbsoluteMidiEvent>,
  time_div : M2Byte
}

impl AbsoluteMidiTrack {
  pub fn add_event(&mut self, event : AbsoluteMidiEvent){
    self.events.push(event);
  }
}

impl Default for AbsoluteMidiTrack {
  fn default() -> Self {
      Self { 
        events: Vec::new(), 
        time_div : m2byte!(540)
      }
  }
}

impl<'a> From<&MidiTrack> for AbsoluteMidiTrack {
  fn from(track: &MidiTrack) -> Self {
    let mut abs_midi_track = AbsoluteMidiTrack::default();

    let track_iter = track.iter();
    
    let tempo = 500_000.0;

    let time_div = track.time_div.into();

    let mut time = 0.0;

    
    for event in track_iter{
      let tempo = match event.get_tempo() {
        Some(tempo) => tempo.micro_secs(),
        None => tempo,
      };

      time += event.delta_time().to_seconds(time_div, tempo);

      abs_midi_track.add_event(AbsoluteMidiEvent::new(time, event.message().clone()));
    }

    abs_midi_track
  }
}
