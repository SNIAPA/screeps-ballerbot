use std::{collections::HashMap, error::Error, hash::Hash};

use screeps::{JsCollectionFromValue, JsHashMap};

pub mod error;


pub trait ToRustHashMap<K, V> {
    fn to_rust_hash_map(self) -> HashMap<K, V>;
}

impl<K: std::cmp::Eq + Hash + JsCollectionFromValue, V: JsCollectionFromValue>
    ToRustHashMap<K, V> for JsHashMap<K, V>
{
    fn to_rust_hash_map(self) -> HashMap<K, V> {
        self.keys().zip(self.values()).collect::<HashMap<K, V>>()
    }
}
