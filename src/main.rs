#![allow(dead_code, unused_variables, unused_must_use, unused_imports, non_camel_case_types, non_upper_case_globals, non_snake_case)]


use std::{ops::Deref, marker::PhantomData, fs::File};

use serde_json::de;

use rmidirs::{
  reader::{MidiFileReader, local::Local, buffer::{Buffer, self}, Reader},
   m3byte, m4bits, m1byte, m2byte
};

lazy_static::lazy_static!(
  #[derive(Debug)]
  static ref json : serde_json::Value = de::from_reader(File::open("./midis/midi-channel-event-schema.json").expect("File should open read only")).unwrap();
);

fn main() {
  // let r = M3Byte::from(12);
  // let q = m1byte!(12);
  // let s = m2byte!(12);
  // let s:&[u8] = &[0, 1, 0];
  // let t = m3byte!(s);
  // println!("t, {:?}", t);
  // let u = m4bits!(24);

  // println!("q : {:?}, s : {:?}, t : {:?}, u : {:?}", q, s, t, u);
  // let json1 : serde_json::Value = de::from_reader(File::open("./midis/midi-channel-event-schema.json")
  //                                 .expect("File should open read only"))
  //                                 .unwrap();

  // println!("json :) {:?}", json["8"].to_string());

  let midi = MidiFileReader::local("./midis/test2.mid").parse();
  println!("midi : ) {:?}", midi);
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