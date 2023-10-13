use std::{collections::HashMap, borrow::BorrowMut};

use crate::{
    creep::{CreepMem, CREEP_MANAGERS},
    util::{MyError, Result},
};
use log::debug;
use screeps::game;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct RootMem {
    creeps: HashMap<String, CreepMem>,
    rooms: RoomMem,
    spawns: SpawnMem,
}

#[derive(Deserialize, Debug, Clone)]
struct RoomMem {}

#[derive(Deserialize, Debug, Clone)]
struct SpawnMem {}

pub fn clean_creeps() -> Result<()> {
    let mut mem =
        serde_json::from_str::<RootMem>(&screeps::raw_memory::get().as_string().ok_or(MyError)?)?;

    let alive_creeps = game::creeps().keys().collect::<Vec<String>>();

    for creep in mem.clone().creeps.keys() {
        if !alive_creeps.contains(creep) {
            mem.creeps.remove(creep);
            debug!("removing: {:?}", creep)
        }
    }

    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creep_managers = creep_managers_refcell.borrow_mut();
        for (i,creep) in  creep_managers.clone().iter().enumerate() {
            if !alive_creeps.contains(&creep.name) {
                creep_managers.remove(i);
                debug!("removing: {:?}", &creep)
            }
        }
    });

    Ok(())
}
