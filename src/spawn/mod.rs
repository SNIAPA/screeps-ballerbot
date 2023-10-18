use std::cell::RefCell;

use log::debug;
use screeps::{game, SpawnOptions, StructureSpawn};

use crate::{
    mem::creep::CreepMem,
    util::{error::MyError, Result, ToRustHashMap},
};

use self::recepie::Recepie;

pub mod recepie;

#[derive(Debug, Clone)]
pub struct SpawnManager {
    pub name: String,
}

impl SpawnManager {
    pub fn run(&mut self) -> Result<()> {
        //self.spawn(MinerManager::recepie())?;
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
            spawn.room().unwrap().name(),
            game::time()
        );
        let mem = CreepMem {
            role: crate::creep::role::Role::MINER,
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

