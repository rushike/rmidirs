use crate::{model::{midi::Midi, midi_header::MidiHeader}, utils::{functions::number, ByteEncodingFormat}};

use super::{Parser, parser_state::ParserState};
pub struct MidiHeaderParser;

impl MidiHeaderParser {
  pub fn parse(buf : &[u8], state : &mut ParserState) -> MidiHeader {
    
    match &buf[..4] {
      b"MThd" => {
        const ENC_FORMAT: ByteEncodingFormat = ByteEncodingFormat::BigEndian;
        let end = 14;
        
        let length = number(&buf[4..8], ENC_FORMAT);
        
        assert!(length == 6, "invalid MIDI header length. Length must be 6, but got {}", length);

        // Midi header in midi v1 is always 14 bytes long
        state.forward(14);

        return MidiHeader::new_raw(
          &buf[8..10],
          &buf[10..12],
          &buf[12..14]
        );
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