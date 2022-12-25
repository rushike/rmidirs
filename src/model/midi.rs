use super::{midi_header::MidiHeader, midi_track::MidiTrack};

#[derive(Debug, Clone, Default)]
pub struct Midi {
  header : MidiHeader,
  tracks : Vec<MidiTrack>,
  total_tracks : usize,
}

impl Midi {
  pub fn add_header(&mut self, header: MidiHeader)  {
    self.header = header;
    // self
  }

  pub fn header(&self) -> &MidiHeader {
    &self.header
  }

  pub fn add_track(&mut self, track: MidiTrack){
    self.tracks.push(track);
    self.total_tracks += 1;
    // self
  }
}
