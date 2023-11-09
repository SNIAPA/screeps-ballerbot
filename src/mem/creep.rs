use std::str::FromStr;

use js_sys::JsString;
use log::{debug, info};
use screeps::{game, Creep, RoomName};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::JsValue;

use crate::{
    creep::{self, role::Role, CREEP_MANAGERS},
    util::{
        error::MyError,
        error::{Result, ToMyErr},
    },
};

use super::RootMem;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreepMem {
    pub role: Role,
    pub room: RoomName,
    pub role_mem: Option<String>,
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

pub trait ParserMemeory {
    fn get_parsed_memory(&self) -> Result<CreepMem>;
    fn set_parsed_memory(&self, new_mem: CreepMem) -> Result<()>;
}

impl ParserMemeory for Creep {
    fn get_parsed_memory(&self) -> Result<CreepMem> {
        let raw_mem = js_sys::JSON::stringify(&self.memory())
            .map_err(|_| MyError::new("cant parse creep memory"))?
            .as_string()
            .to_my_err("cant parse creepm memory")?;
        Ok(serde_json::from_str::<CreepMem>(&raw_mem)?)
    }
    fn set_parsed_memory(&self, new_mem: CreepMem) -> Result<()> {
        let new_mem = js_sys::JSON::parse(serde_json::to_string(&new_mem)?.as_str())
            .map_err(|_| MyError::new("cat serialize creep memory"))?;
        self.set_memory(&new_mem);
        Ok(())
    }
}

pub fn get_mem() -> Result<RootMem> {
    let mut raw_mem = screeps::raw_memory::get()
        .as_string()
        .to_my_err("cannot serialize memory")?;

    if raw_mem.as_str() == "" {
        raw_mem = "{}".to_owned();
    }

    let mut parsed_mem = serde_json::from_str::<Value>(&raw_mem)?;
    if let Some(creeps) = parsed_mem.get_mut("creeps") {
        creeps
            .as_object_mut()
            .unwrap()
            .retain(|_, v| v.get("role").is_some());
    }

    Ok(serde_json::from_value::<RootMem>(parsed_mem)?)
}

pub fn clean_creeps() -> Result<()> {
    let mem = get_mem()?;

    let alive_creeps = game::creeps().keys().collect::<Vec<String>>();

    let mut creeps = mem.creeps.clone();
    if creeps.is_none() {
        return Ok(());
    }
    let creeps = creeps.as_mut().to_my_err("cant borrow creep as mut")?;

    creeps.retain(|name, _| alive_creeps.contains(name));

    screeps::raw_memory::set(&JsString::from_str(&serde_json::to_string(&mem)?)?);

    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creep_managers = creep_managers_refcell.borrow_mut();

        creep_managers.retain(|name, _| alive_creeps.contains(name));
    });

    Ok(())
}
