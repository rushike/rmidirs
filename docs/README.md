# rmidirs

rmidirs is pure rust libary to parse the midi files (both _MIDI v1 and MIDI v2_)

<!-- ## Table of Contents
1. Getting Started
2. **rmidirs** -->


## Getting Started
**Open the midi file**
```rust
use rmidirs::reader::MidiFileReader;
fn main() {
  let midi = MidiFileReader::local("test.mid").parse();
}
```



