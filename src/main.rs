#![allow(dead_code, unused_variables, unused_must_use, unused_imports, non_camel_case_types, non_upper_case_globals, non_snake_case)]


use std::{ops::Deref, marker::PhantomData, fs::File};

use serde_json::de;

use rmidirs::{
  reader::{MidiFileReader, local::Local, buffer::{Buffer, self}, Reader},
   m3byte, m4bits, m1byte, m2byte, 
   model::{
    core::midi_track::AbsoluteMidiTrack,
    note_seq::TrackSeq
  }
};

lazy_static::lazy_static!(
  #[derive(Debug)]
  static ref json : serde_json::Value = de::from_reader(File::open("./midis/midi-channel-event-schema.json").expect("File should open read only")).unwrap();
);

fn main() {
  
  let midi = MidiFileReader::local("./midis/test2.mid").parse();
  // println!("midi : ) {:?}", midi.track(0));
  // println!("midi : ) {:?}", AbsoluteMidiTrack::from( midi.track(0)));
  let tracksq = TrackSeq::from(midi);
  println!("midi : track_seq , {tracksq:?}")
  // println!("leading ones {}, {}", i8::from_be_bytes([0xfa]));
}



pub(crate) trait SplitChecked: Sized {
  fn split_checked(&mut self, at: usize) -> Option<Self>;
}
impl<'a> SplitChecked for &'a [u8] {
  #[inline]
  fn split_checked(&mut self, at: usize) -> Option<&'a [u8]> {
      if at > self.len() {
          None
      } else {
          let (extracted, remainder) = self.split_at(at);
          *self = remainder;
          Some(extracted)
      }
  }
}

pub struct Test<'a> {
  val : usize,
  arr : Vec<u8>,
  slice : Box<[u8]>,
  _fake : &'a str
}

trait Tester {
    fn test(&self, start : usize) -> &[u8];
}

impl<'a> Tester for Test<'a> {
  fn test(&self, start : usize) -> &[u8] {
    &self.arr[start..]
  }
}

// impl<'a> Iterator for Test<'a> {
//     type Item = (u8, &'a [u8]);

//     fn next(&mut self) -> Option<Self::Item> {
//       if self.val < self.arr.len() - 1 {
//         self.val += 1;
//         self.slice = Box::new(self.arr[self.val..]);
//         Some((self.arr[self.val - 1], &self.arr[self.val..]))
//       } else {
//           None
//       }
//     }
// }
pub struct BTest(Test<'static>);


impl Deref for BTest {
    type Target = Test<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}