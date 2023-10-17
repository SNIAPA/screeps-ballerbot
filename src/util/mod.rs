use std::{error::Error, collections::HashMap};

use screeps::JsHashMap;

pub mod error;

pub type Result<T> = std::result::Result<T,Box<dyn Error>>;

pub trait ToRustHashMap<K:Copy,V: Copy>{
    fn to_rust_hash_map(self) -> HashMap<K,V>;
}

impl<K:Copy + std::cmp::Eq,V:Copy> ToRustHashMap<K,V> for HashMap<K,V>{
    fn to_rust_hash_map(self) -> HashMap<K,V> {
      self.keys().copied().zip(self.values().copied()).collect::<HashMap<K,V>>()

    }    
}

