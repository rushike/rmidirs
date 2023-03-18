use std::ops::{Deref, DerefMut};

use crate::primitive::FloatWord;

pub struct Node<T>(FloatWord, T);

impl<T> Node<T> {
    pub fn new(at: FloatWord, element: T) -> Self {
      Self(at, element)
    }
}

impl<T> Deref for Node<T>  {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.1  
  }
}