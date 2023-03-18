use std::vec;

use crate::primitive::{M2Byte, Word, FloatWord, FractionWord};

use super::{node::Node, note::Note};

/// Note Sequence stores note progression similar to midi track,
/// but instead of NoteOn and NoteOff events, single Note event is stored,
/// and note duration and note length are stored instead of delta time.
/// 
/// All other meta / channel events information are store as part of note itself,
/// or as part of note sequence attributes.
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
