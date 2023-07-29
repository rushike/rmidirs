use crate::{utils::{functions::{number, masked_number}, ByteEncodingFormat}, primitive::{M2Byte, m2byte}};


#[derive(Debug, Clone)]
#[repr(i16)]
pub enum MidiFormat {
  SingleTracksMultiChannel = 0,
  MultiTracks = 1,
  MultiTracksIndependentSingleChannel = 2,
  Invalid(String) = -1
}

impl From<MidiFormat> for M2Byte {
  fn from(value: MidiFormat) -> Self {
    match value {
      MidiFormat::SingleTracksMultiChannel => m2byte!(0),
      MidiFormat::MultiTracks => m2byte!(1),
      MidiFormat::MultiTracksIndependentSingleChannel => m2byte!(2),
      MidiFormat::Invalid(_) => m2byte!(-1),
    }
  }
}

#[derive(Debug, Clone)]
pub enum MidiDivision {
  MetricTime(u16),
  SubDivision((i8, u8)),
  Invalid(String)
}

impl MidiDivision {
  pub fn metric_time(&self) -> Option<M2Byte> {
    match self {
        MidiDivision::MetricTime(val) => Some(m2byte!(*val)),
        MidiDivision::SubDivision(_) => None,
        MidiDivision::Invalid(_) => None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct MidiHeader {
  header : String,
  length : u32,
  format : MidiFormat,
  ntrk : M2Byte,
  division : MidiDivision,
}

impl Default for MidiHeader {
  fn default() -> MidiHeader {
    MidiHeader { 
      header : "MThd".to_string(),
      length : 6,
      format : MidiFormat::SingleTracksMultiChannel,
      ntrk : m2byte!(0),
      division : MidiDivision::MetricTime(480)
    }
  }
}

impl MidiHeader {
  pub fn format(&self) -> MidiFormat {self.format.clone()}
  pub fn division(&self) -> MidiDivision {self.division.clone()}
  pub fn ntrk(&self) -> M2Byte {self.ntrk}

  pub fn new(format : MidiFormat, ntrk : M2Byte, division : MidiDivision) -> Self {
    Self {
      header : "MThd".to_string(),
      length : 6,
      format,
      ntrk,
      division
    }
  }    

  pub fn new_raw(format : &[u8], ntrks : &[u8], division : &[u8]) -> Self { 
    const ENC_FORMAT: ByteEncodingFormat = ByteEncodingFormat::BigEndian;

    MidiHeader::new(
      Self::parse_format(format),
      m2byte!(ntrks), 
      Self::parse_division(division)
    )
  }

  fn parse_format(format : &[u8]) -> MidiFormat{
    assert!(format.len() == 2, "Midi Format is 2 byte value. But passed div : {:?} with len {} number of bytes", format, format.len());

    let format = number(format, ByteEncodingFormat::BigEndian);
    
    match format {
      0 => MidiFormat::SingleTracksMultiChannel,
      1 => MidiFormat::MultiTracks,
      2 => MidiFormat::MultiTracksIndependentSingleChannel,
      _ => MidiFormat::Invalid(format!("'format' should be either 0, 1, or 2, but got {format}"))
    }
  }

  fn parse_division(div : &[u8]) -> MidiDivision {
    assert!(div.len() == 2, "Midi Division is 2 byte value. But passed div : {:?} with len {} number of bytes", div, div.len());
    const BIT_MASK : u8 = 0x80;
    const ENC_FORMAT : ByteEncodingFormat = ByteEncodingFormat::BigEndian;
    match div[0] & BIT_MASK {

      // ticks per quarter-note
      0 => { 
        let ticks = masked_number(div, &[0x7F, 0xFF], ENC_FORMAT);
        MidiDivision::MetricTime(ticks as u16)
      }

      // Sub Division;  SMPTE and MIDI Time Code.
      BIT_MASK => {
        let n_smpte =  i8::from_be_bytes([div[0] & 0x7f]);
        let frame_resolution = div[1] & 0xFF;
        assert!(!(n_smpte == -24 || n_smpte ==-25 || n_smpte == 29 || n_smpte == 30), "SMPTE (1st byte) of division should be from list [-24, -25, -29, -30]. But {} was passed with byte val : {}", n_smpte, div[0]);
        MidiDivision::SubDivision((n_smpte, frame_resolution))
      }

      _=>MidiDivision::Invalid(format!("midi_header > division : This should never happen"))
    }
  }
}