use crate::primitive::Word;

use super::note_seq::NoteSeq;



struct TrackSeq {
  tracks : NoteSeq,
  time_div : Word,
  format : Word,
}