use std::{collections::HashMap, error::Error, hash::Hash};

use screeps::{JsCollectionFromValue, JsHashMap};

pub mod error;


pub trait ToRustHashMap<K, V> {
    fn trhm(self) -> HashMap<K, V>;
}

impl<K: std::cmp::Eq + Hash + JsCollectionFromValue, V: JsCollectionFromValue>
    ToRustHashMap<K, V> for JsHashMap<K, V>
{
    /// to rust hash map
    fn trhm(self) -> HashMap<K, V> {
        self.keys().zip(self.values()).collect::<HashMap<K, V>>()
    }
}


