# MIDI File Format Spec. 1.1

MIDI files are stored in 8 bit byte stream. File is diveden into **bytes chunks**, and **chunks** are further divided into **events**. 

MIDI events are seperated with timestamp called **delta time**. Delta Time refers to time difference between two consecutive events.

Delta time are encoded as variable length bytes array (0 byte - 4 bytes). Last byte uses only 7 bits, and MSB bit is used to indicate it has last byte of variable length array.


## MIDI
<midi-header-chunk> <midi-track-chunk>*

### Variable Length Byte Array

## Chunks
Chunks stores the midi events.

### Chunk Format
```
<chunk-header> <length> <midi-events>+
```
- `<length>` : Length is 32 bit int encoded in 4 byte array encoded in Big Endian. 
- `<chunk-header>` : There are two chunk header in MIDI
  - `MThd` : Midi Header
  - `MTrk` : Midi Track Header


## MIDI Events
MIDI Events stores the MIDI messages. 

MIDI Messages are primarily of 3 types
- Channel Event
- Meta Event
- Sys Event

**MIDI Event Format**
```
<delta-time> [<channel-event> | <meta-event> | <sys-event>]
```
- `<delta-time>` : Delta time is stored as [variable length byte array](#variable-length-byte-array)

### Channel Event
**MIDI Channel Event Format**
```
<channel>
```

### Meta Event
**MIDI Meta Event Format**
```
0xFF <type> <length> <bytes>*
```
- `<type>`: [Meta Event Type ](#meta-event-type)
- `<length>`: Length is stored are [variable length byte array](#variable-length-byte-array)

#### Meta Event Type
| Meta Event Type      | Meta Event Id | Length | Description  |
| -------------------- | ------------: | ------:| ----         |
| Sequence Number      | 0x00          | 0x02   |              |
| Text Event           | 0x01          | Var Len| Any text event at any position on track      |
| Copyright Notice     | 0x02          | Var Len| The notice should contain the characters (C), the year of the copyright, and the owner of the copyright      |
| Text Sequence/Track  | 0x03          | Var Len| Name of the sequence or name of track       |
| Instrument Name      | 0x04          | Var Len| Instrument used on track                  |
| Text Lyric           | 0x05          | Var Len| Lyrics of song, splited in syllable at start of note on event   |
| Marker               | 0x06          | Var Len| name of that point in the sequence           |
| Cue Point            | 0x07          | Var Len| A description of something happenings at that point in the musical score    |
| MIDI Channel Prefix  | 0x20          | 0x01   |               |
| End of Track         | 0x2F          | 0x00   |               |
| Set Tempo            | 0x51          | 0x03   | Mirco-secs / Quater Note, stores in 3 byte in Big Endian format |
| SMPTE Offset         | 0x54          | 0x05   |               |
| Time Signature       | 0x58          | 0x04   |               |
| Key Signature        | 0x59          | 0x02   | Key signature of track, it describe scale of track      |
| Sequencer Specific Meta-Event  | 0x7F| Var Len|               |


## References
- [Standard MIDI-File Format Spec. 1.1, updated](http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html#BM1_)