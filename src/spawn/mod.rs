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
    pub fn run_all() -> Result<()> {
        SPAWN_MANAGERS.with(|creep_managers_refcell| {
            for spawn_manager in creep_managers_refcell.borrow_mut().iter_mut() {
                let spawn = game::spawns()
                    .get(spawn_manager.name.clone())
                    .ok_or(MyError {
                        message: "Spawn not found".to_string(),
                    })
                    .unwrap();
                spawn_manager.run().unwrap();
            }
        });

        Ok(())
    }
    pub fn setup() -> Result<()> {
        SPAWN_MANAGERS.with(|spawn_managers| {
            let mut spawn_managers = spawn_managers.borrow_mut();
            let spawns = game::spawns().to_rust_hash_map();

            spawns
                .iter()
                .for_each(|(name, spawn)| spawn_managers.push(SpawnManager { name: name.to_string() }));
        });
        Ok(())
    }
    fn run(&mut self) -> Result<()> {
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

thread_local! {
  pub static SPAWN_MANAGERS: RefCell<Vec<SpawnManager>> = RefCell::new(Vec::new());
}
