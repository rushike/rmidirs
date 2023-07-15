use std::{vec, collections::HashMap};

use crate::{
  primitive::{M2Byte, Word, FloatWord, FractionWord}, 
  model::core::{midi_track::MidiTrack, midi::Midi, midi_header::MidiHeader, 
    midi_event::{
      MidiMessage::{MetaEvent, ChannelEvent, SysEvent, Invalid},
      channel_message::{ChannelMessage, self},
      meta_message::Tempo
    },
  }
};

use super::{node::Node, note::Note};

/// Note Sequence stores note progression similar to midi track,
/// but instead of NoteOn and NoteOff events, single Note event is stored,
/// and note duration and note length are stored instead of delta time.
/// 
/// All other meta / channel events information are store as part of note itself,
/// or as part of note sequence attributes.
#[derive(Debug)]
pub struct NoteSeq {
  /// total time of the note sequence stored in seconds
  total_time : FloatWord,

  notes : Vec<Node<Note>>,
  
  /// stores tempo in **qbpm** (quater beats per minute) 
  tempos : Vec<Node<Word>>,

  /// default : 4/4 is assumed per MIDI standard. 
  /// represented as (4, 4)
  time_signatures : Vec<Node<FractionWord>>,
  
  /// default : C Major, is assumed per MIDI standard.
  /// represented as cmajor
  key_signatures : Vec<Node<String>>
}

impl Default for NoteSeq {
  fn default() -> Self {
    Self { 
      total_time: 0.0, 
      notes: Default::default(), 
      tempos: vec![Node::new(0.0, 120)], 
      time_signatures: vec![Node::new(0.0, (4, 4))], 
      key_signatures: vec![Node::new(0.0, "c-major".to_string())], 
    }
  }
}


impl From<(MidiHeader, MidiTrack)> for NoteSeq {
  fn from(value: (MidiHeader, MidiTrack)) -> Self {
    Self::from((&value.0, &value.1))
  }
}

impl From<(&MidiHeader, &MidiTrack)> for NoteSeq {
  fn from((midi_header, track): (&MidiHeader, &MidiTrack)) -> Self {
    let mut note_seq = Self::default();

    // midi division, stores ticks per quater note
    let midi_division : M2Byte = midi_header.division().metric_time().unwrap();
    
    // midi format
    let midi_format : M2Byte = midi_header.format().into();

    // track iterator to iterate through all events.
    let track_iter = track.iter();
    
    let mut tempo = Tempo::default();

    // Running time
    let mut time = 0.0;

    // Keeps start time of all events that are yet to be closed.
    let mut timekeeper = HashMap::new();

    track.iter().for_each(|event| {
      // incrementing time counter with current event delta time in seconds
      time += event.delta_time().to_seconds(midi_division.into(), tempo.micro_secs());
      
      // adjusting current tempo to new tempo if current event is tempo change event.
      tempo = match event.get_tempo() {
        Some(tempo) =>tempo,
        None => tempo,
      };

      match event.message() {
        ChannelEvent(channel_message) => {
          if channel_message.is_note_on_event() {
            // Will insert the Note On event into timekeeper, and mark the current time
            println!("insert note on in timekeeper : {channel_message:?}, {time}");
            // dbg!(&timekeeper);
            let note_no : Word = channel_message.get_note_number().unwrap().into();
            timekeeper.insert(note_no, (time, channel_message.clone()));
          }
          else if channel_message.is_note_off_event() {
            
            let note_number : Word = match channel_message.get_note_number() {
                Some(note) => note.into() ,
                None => return,
            };
          
            let (start_time, note) = match timekeeper.remove(&note_number) {
                Some(note_on) => note_on,
                None => return,
            };
            
            // adding the event to note_seq
            note_seq.notes.push(Node::new(
              time, 
              Note::from((note, start_time, time))
            ))
          }
        },
        _ => {}
      };
    });
    note_seq
  }
}

impl From<Midi> for NoteSeq{
  fn from(midi: Midi) -> Self {
    Self::from((midi.header(), midi.track(0)))
  }
}

impl From<&Midi> for NoteSeq{
  fn from(midi: &Midi) -> Self {
    Self::from((midi.header(), midi.track(0)))
  }
}