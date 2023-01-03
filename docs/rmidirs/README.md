## rmidirs
**rmidirs** is divided into 4 parts mentioned below

### reader
  - This will read static midi file from various sources like
    - **buffer**
      - From byte array residing in memory
    - **local**
      - From local **_.mid_** file
    - **web**
      - From web hosted **_.mid_** file

### writer
  - This will write static midi file to file in memory, local or web

### parser 
  - It will parse the entire midi file, or part, or midi messages recivieng online.

### model
  - **model** has datasructures define to store MIDI data type, like 
    - MIDI
    - MIDI Header
    - MIDI Track
    - MIDI Event

### transform
  - **transform** should transform the midi object to data structure like piano roll, sflat, etc.