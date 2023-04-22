use crate::{primitive::{Word, M2Byte}, model::core::midi::Midi};

use super::note_seq::NoteSeq;



#[derive(Debug)]
pub struct TrackSeq {
  tracks : Vec<NoteSeq>,
  time_div : Word,
  format : Word,
}

impl From<Midi> for TrackSeq {
  fn from(midi: Midi) -> Self {
    let midi_header = midi.header();
    let midi_division = midi_header.division().metric_time().unwrap();
    let midi_format : M2Byte = midi_header.format().into();

    let mut track_seq = Self {
      time_div : midi_division.into(),
      format : midi_format.into(),
      tracks: Vec::new(),
    };

    for track in midi.tracks() {
      track_seq.tracks.push((midi_header, track).into())
    }

    track_seq
  }
}