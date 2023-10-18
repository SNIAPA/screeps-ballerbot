use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod creep;

use creep::CreepMem;

#[derive(Deserialize,Serialize, Debug, Clone)]
struct RootMem {
    creeps: Option<HashMap<String, Option<CreepMem>>>,
}

#[derive(Deserialize, Serialize,Debug, Clone)]
struct RoomMem {}

#[derive(Deserialize, Serialize,Debug, Clone)]
struct SpawnMem {}
