
pub(crate) mod functions;

pub enum ByteEncodingFormat {
  /// A big-endian stores the MSB of a word at the smallest memory address / index and the LSB at the largest.
  BigEndian,
  /// A little-endian stores the LSB at the smallest address / index and the MSB at the largest memory address / index
  LittleEndian,
}

pub enum StringEncoding {
  UTF8
}
