use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::{rc::Rc, marker::PhantomData};
use std::ops::{Deref, DerefMut};

use crate::primitive::{FloatWord, DoubleWord, Word};

pub type NodePtr<T> = Rc<RefCell<Node<T>>>;

pub type OptionNodePtr<T> = Option<Rc<RefCell<Node<T>>>>;



#[derive(Debug, Clone)]
pub struct Sequence<T>{
  quanta : Quanta,
  map : HashMap<Quanta, NodePtr<T>>
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quanta(FloatWord);

impl Eq for Quanta {
    
}

impl Hash for Quanta {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      state.write_u64((self.0 * 1000.0) as u64);
    }
}

impl Deref for Quanta {
  type Target = FloatWord;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Quanta {
  pub fn get(&self, at : FloatWord) -> Quanta {
    Quanta((at / self.0).floor())
  }

  pub fn next(&self, at : FloatWord) -> Quanta {
    Quanta(*self.get(at) + self.0)
  }

  pub fn prev(&self, at : FloatWord) -> Quanta {
    Quanta(*self.get(at) - self.0)
  }
}

#[derive(Debug, Clone)]
pub struct Node<T> {
  time : FloatWord,
  element : T,
  forward : OptionNodePtr<T>,
  backward : OptionNodePtr<T>
}

impl<T> Node<T> {
  pub fn new(time : FloatWord, element : T, forward : OptionNodePtr<T>, backward : OptionNodePtr<T>) -> Self {
    Self { time, element, forward, backward }
  }

  pub fn insert(&mut self, at : Quanta, element : NodePtr<T>) -> OptionNodePtr<T>{
    todo!()
  }
}

impl<T> Sequence<T> {
  pub fn new(quanta : FloatWord) -> Self {
    Self {
      quanta : Quanta(quanta),
      map : HashMap::new()
    }
  }


  /// This method will insert a element of type 'T' into the Sequence.
  /// Sequence uses the HashMap of <Quanta, NodePtr<T>> to store element.
  /// 
  /// This finds forward and backward references to current quanta
  /// 
  /// If quanta already present in map
  ///   - then it will get head element of quanta from sequence
  ///   - and will call insert on head element, with new element
  /// 
  /// Else it will insert element in map
  pub fn insert(&mut self, at : FloatWord, element: T) {

    let quanta = self.quanta.get(at) ;

    let forward = self.get_rc(self.quanta.next(at));
    let backward = self.get_rc(self.quanta.prev(at));
    let curr = self.get_rc(quanta);

    let new = Rc::new(RefCell::new(Node::new(at, element, forward, backward)));
          
    match curr {
        Some(head) => head.borrow_mut().insert(quanta, new),
        None => self.map.insert(quanta, new),
    };
  } 

  fn get_rc(&self, at : Quanta) -> OptionNodePtr<T> {
    todo!()
  }

  pub fn get(&self, at : FloatWord) -> T {
    todo!()
  }
}