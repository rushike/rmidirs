use std::collections::HashMap;

use crate::primitive::{M2Byte, M3Byte, m3byte};

use super::{midi_track::MidiTrack, midi_event::{MidiEvent, Event, meta_event::MetaEvent}};

#[derive(Debug, Clone, Default)]
pub struct NoteEvent {
  note_id : u32,
  start_time : f32,
  end_time : f32,
  duration : f32,
}

impl NoteEvent {

  pub fn new(note_id : u32, start_time : f32) -> NoteEvent {
    NoteEvent { note_id, start_time, end_time: -1.0, duration: -1.0 }
  }

  pub fn set_duration(&mut self, duration : f32 ) -> &Self {
    self.duration = duration;
    self
  }

  pub fn set_start_time(&mut self, start_time : f32 ) -> &Self {
    self.start_time = start_time;
    self
  }

  pub fn set_end_time(&mut self, end_time : f32 ) -> &Self { 
    self.end_time = end_time;
    self
  }
}

#[derive(Debug, Clone)]
pub struct  TimeLineEvent<T> {
  time : f32,
  event: T
}

#[derive(Debug, Clone)]
pub struct Timeline{
  timeline: Vec<NoteEvent>,
  time_div: M2Byte,
}


impl<'a> From<&MidiTrack> for Timeline {
  fn from(track: &MidiTrack) -> Self {
    let mut timeline = Vec::new();

    let mut tempo = m3byte!(500_000_u32);

    let time_div = track.time_div;

    let mut time = 0.0;

    let mut timekeeper = HashMap::new();

    for event in track.iter(){
      time += event.delta_time.to_seconds(time_div, tempo);
      
      tempo = update_tempo_if_tempo_event(&event, tempo);

      process_channel_event(time, event, &mut timeline, &mut timekeeper);

    }
    Timeline { timeline, time_div }
  }
}

fn process_channel_event(time : f32, 
                          event : MidiEvent, 
                          timeline : &mut Vec<NoteEvent>, 
                          timekeeper : &mut HashMap<u32, TimeLineEvent<MidiEvent>>
                        ) {
  if !event.is_note_on_off_event() {return;}

  let note_number = *event.get_note_number().unwrap();
  
  if event.is_note_on_event() {
  
    timekeeper.insert(
      note_number,
      TimeLineEvent{
        time,
        event
      }
    );
  } else if event.is_note_off_event() && timekeeper.contains_key(&note_number) {

      let e = timekeeper.remove(&note_number).unwrap();
      
      timeline.push(NoteEvent {
          note_id: note_number,
          start_time: e.time,
          end_time: time,
          duration: time - e.time,
      });
    }
  }

fn update_tempo_if_tempo_event(event : &MidiEvent, tempo : M3Byte) -> M3Byte{
  match &event.event {
    Event::MetaEvent(
      MetaEvent::Tempo(tempo)
    ) => {
      println!("event {:?}", event);
      *tempo},
    _=> tempo
  }
}