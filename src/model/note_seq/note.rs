use crate::{primitive::{Word, FloatWord}, model::core::midi_event::channel_message::{ChannelMessage, self}};

/// Note stores the information of Midi Note Event.
/// Unlike the Note ON and Note OFF events in MIDI, 
/// Note stores the start time and end time in single object.
#[derive(Debug)]
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


impl From<ChannelMessage> for Note {
  fn from(channel_message: ChannelMessage) -> Self {
    Self::from((channel_message, 0.0, 0.0))
  }
}

impl From<(ChannelMessage, FloatWord, FloatWord)> for Note {
  fn from((channel_message, start_time, end_time): (ChannelMessage, FloatWord, FloatWord)) -> Self {
    let (pitch , velocity) = match channel_message {
      ChannelMessage::NoteOn(note_on) => (note_on.note, note_on.velocity),
      ChannelMessage::NoteOff(note_off) => (note_off.note, note_off.velocity),
      rest => panic!("Trying to create a Note from {rest:?} event, which is not NOTE_ON_OFF event.")
    };
    Note::new(pitch.into(), velocity.into(), start_time, end_time)      
  }
}