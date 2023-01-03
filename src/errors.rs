pub enum MidiParseErrors<'a> {
  EndOfBuffer(&'a str),
  InvalidEventByte(&'a str),
  NotMidiMetricTime(&'a str),
}