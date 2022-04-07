use std::{mem};
use std::collections::LinkedList;
use xxhash_rust::xxh3::xxh3_64;

trait T1 {
  fn one() -> Self;
}
#[derive(Default)]
struct S1 {}
impl T1 for S1 {
  fn one() -> Self {
    S1 {}
  }
}
#[derive(Debug)]
pub(crate) struct LinkedListHashMap<K, V> {
  buckets: Vec<KeyPair<K, V>>,
  len: usize,
  capacity: usize,
  fill: usize,
}
#[derive(Clone,Debug)]
pub struct KeyPair<K, V> {
  key: K,
  value: V,
  hash: u64,
  dib: u8,
  link: LinkedList<KeyPair<K, V>>,
}
impl<K: Default, V:Default> Default for KeyPair<K, V> {
  fn default() -> Self {
    KeyPair {
      key: K::default(),
      value: V::default(),
      hash: 0,
      dib: 0,
      link : LinkedList::default(),
    }
  }
}

impl<K: Eq + ToString + Default, V: Default> LinkedListHashMap<K, V> {
  pub fn new(capacity: usize) -> Self {
    LinkedListHashMap {
      buckets: Vec::with_capacity(capacity),
      len: 0,
      capacity,
      fill: 0,
    }
  }

  pub fn resize(&mut self) {
    let target_size = match self.buckets.len() {
      0 => self.capacity,
      n => n * 2,
    };

    let mut new_buckets = Vec::with_capacity(target_size);
    new_buckets.extend((0..target_size).map(|_| KeyPair::<K, V>::default()));
    for keyPair in self.buckets.drain(..){
      let mut dib = 0;
      let hash = xxh3_64(keyPair.key.to_string().as_bytes());
      let mut index = hash as usize % self.capacity;

      let kp = keyPair;

      new_buckets[index] = kp;
    }

    mem::swap(&mut self.buckets,&mut new_buckets ) 
  }

  pub fn insert(&mut self,key: K, value: V) -> Option<V> {
    if self.buckets.is_empty() || self.buckets.len() - self.capacity < 4 {
      self.resize();
    }
    let mut dib = 0;
    let mut hash = xxh3_64(key.to_string().as_bytes());
    let mut index = hash as usize % self.capacity;

    let kp = KeyPair {
      key,
      value,
      hash,
      dib: 0,
      link: LinkedList::default(),
    };

    match self.buckets[index].link.is_empty(){
      true => {
        self.buckets[index].link.push_front(kp);
        self.len += 1;
      },
      false => {
        self.buckets[index] = kp;
        self.len += 1;
      }
    }
    return None;
  }
}
