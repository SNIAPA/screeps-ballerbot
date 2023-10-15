use std::str::FromStr;

use js_sys::JsString;
use log::{debug, info};
use screeps::{game, Creep};
use serde::{Deserialize, Serialize};

use crate::{
    creep::{role::Role, CREEP_MANAGERS},
    util::{error::MyError, Result},
};

use super::RootMem;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreepMem {
    pub role: Role,
    pub _move: Option<MoveMem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MoveMem {
    path: String,
    room: String,
    time: u64,
    dest: DestMem,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DestMem {
    room: String,
    x: u8,
    y: u8,
}

pub trait GetParsedCreepMemory {
    fn get_parsed_memory(&self) -> Result<CreepMem>;
}

impl GetParsedCreepMemory for Creep {
    fn get_parsed_memory(&self) -> Result<CreepMem> {
        let raw_mem = js_sys::JSON::stringify(&self.memory())
            .unwrap()
            .as_string()
            .ok_or(MyError {
                message: "Cant get memory".to_string(),
            })?;
        Ok(serde_json::from_str::<CreepMem>(&raw_mem)?)
    }
}

pub fn clean_creeps() -> Result<()> {
    let raw_mem = &screeps::raw_memory::get().as_string().ok_or(MyError {
        message: "Cant get memory".to_string(),
    })?;

    if raw_mem == "{}" {
        return Ok(());
    }

    let mut mem = serde_json::from_str::<RootMem>(raw_mem).unwrap();
    debug!("{:?}", mem.creeps.keys());

    let alive_creeps = game::creeps().keys().collect::<Vec<String>>();

    for creep in mem.clone().creeps.keys() {
        if !alive_creeps.contains(creep) {
            mem.creeps.remove(creep);
            info!("removing mem: {:?}", creep)
        }
    }

    screeps::raw_memory::set(&JsString::from_str(&serde_json::to_string(&mem).unwrap()).unwrap());

    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creep_managers = creep_managers_refcell.borrow_mut();
        creep_managers.retain(|creep_manager| {
            if !alive_creeps.contains(&creep_manager.name) {
                info!("removing manager: {:?}", &creep_manager.name);
                return false;
            }
            return true;
        });
    });

    Ok(())
}
