use crate::{model::{midi::Midi, midi_header::MidiHeader}, utils::{functions::number, ByteEncodingFormat}};

use super::Parser;
pub struct MidiHeaderParser;

impl Parser for MidiHeaderParser {
  fn parse(buf : &[u8], midi : &mut Midi) -> usize {
    
    match &buf[..4] {
      b"MThd" => {
        const ENC_FORMAT: ByteEncodingFormat = ByteEncodingFormat::BigEndian;
        let end = 14;
        
        let length = number(&buf[4..8], ENC_FORMAT);
        assert!(length == 6, "invalid MIDI header length. Length must be 6, but got {}", length);

        midi.add_header(
          MidiHeader::new_raw(
          &buf[8..10],
          &buf[10..12],
          &buf[12..14]
        ));
        end
      },
      _header => panic!("MIDI header should start with 'MThd', but got {_header:?}")
    }
  }
}
/*
.add_header(
  MidiHeader::new_raw(
    &buf[8..10],
    &buf[10..12],
    &buf[12..14]
  ),
)
*/