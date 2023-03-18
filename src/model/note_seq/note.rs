use crate::primitive::{Word, FloatWord};

/// Note stores the information of Midi Note Event.
/// Unlike the Note ON and Note OFF events in MIDI, 
/// Note stores the start time and end time in single object.
pub struct Note {
  /// MIDI pitch; see https://en.wikipedia.org/wiki/MIDI_Tuning_Standard for details.
  pitch : Word,

  /// Velocity ranging between 0 and 127.
  velocity : Word,
  
  /// Start time in seconds.
  start_time : FloatWord,

  /// End time in seconds.
  end_time : FloatWord,
}

impl Note {
  pub fn new(pitch: Word, velocity: Word, start_time: FloatWord, end_time: FloatWord) -> Self {
    Self { pitch, velocity, start_time, end_time }
  }

  pub fn pitch_name(&self) -> &str { 
    return "C";
  }
}