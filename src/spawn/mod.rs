use std::{cell::RefCell, collections::HashMap};

use log::debug;
use screeps::{game, Room, SpawnOptions, StructureSpawn};

use crate::{
    creep::role::Role,
    mem::creep::{CreepMem, GetParsedCreepMemory},
    room::{RoomManager, ROOM_MANAGERS},
    util::{error::MyError, Result, ToRustHashMap},
};

use self::recepie::Recepie;

pub mod recepie;

#[derive(Debug, Clone)]
pub struct SpawnManager {
    pub name: String,
}

impl SpawnManager {
    pub fn run(&mut self, room_manager: &mut RoomManager) -> Result<()> {
        let recepie = room_manager.get_next_creep_to_spawn();
        match recepie {
            Some(x) => self.spawn_creep(x).unwrap(),
            None => (),
        };
        Ok(())
    }
    fn spawn(&self) -> StructureSpawn {
        game::spawns().get(self.name.clone()).unwrap()
    }
    fn spawn_creep(&self, recepie: Recepie) -> Result<()> {
        let spawn = self.spawn();

        let name = format!(
            "{}@{}#{}",
            recepie.role.as_string(),
            spawn.room().unwrap().name(), game::time()
        );

        let mem = CreepMem {
            room: spawn.room().unwrap().name(),
            role: recepie.role,
            _move: None,
        };

        let options = SpawnOptions::new()
            .memory(js_sys::JSON::parse(&serde_json::to_string(&mem).unwrap()).unwrap());

        let options_test = SpawnOptions::new()
            .memory(js_sys::JSON::parse(&serde_json::to_string(&mem).unwrap()).unwrap())
            .dry_run(true);

        let test = self
            .spawn()
            .spawn_creep_with_options(&recepie.parts, &name, &options_test);

        match test {
            Ok(_) => {
                debug!("SPAWNING{:?} {:?}", recepie, mem);
                spawn
                    .spawn_creep_with_options(&recepie.parts, &name, &options)
                    .unwrap();
                Ok(())
            }
            Err(e) => match e {
                _ => Ok(()),
            },
        }
    }
}
