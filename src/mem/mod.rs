use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod creep;

use creep::CreepMem;

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct RootMem {
    pub creeps: Option<HashMap<String, Option<CreepMem>>>,
}

#[derive(Deserialize, Serialize,Debug, Clone)]
struct RoomMem {}

#[derive(Deserialize, Serialize,Debug, Clone)]
struct SpawnMem {}
