use std::cell::RefCell;

use log::{debug, info, warn};
use screeps::{game, Part, StructureSpawn};

use crate::util::{MyError, Result};

#[derive(Debug, Clone)]
pub struct SpawnManager {
    spawn: StructureSpawn,
    pub name: String,
}

impl SpawnManager {
    fn run(&self, spawn: StructureSpawn) {
        info!("running {:?}", spawn)
    }
    pub fn run_all() -> Result<()> {
        SPAWN_MANAGERS.with(|creep_managers_refcell| {
            for spawn_manager in creep_managers_refcell.borrow_mut().iter() {
                let spawn = game::spawns()
                    .get(spawn_manager.name.clone())
                    .ok_or(MyError{ message: "Spawn not found" }).unwrap();
                spawn_manager.run(spawn);
            }
        });

        Ok(())
    }
    pub fn setup() -> Result<()> {
        SPAWN_MANAGERS.with(|spawn_manager| {
            let mut spawn_manager = spawn_manager.borrow_mut();
            let spawns = game::spawns();

            spawns.keys().for_each(|spawn| {
                spawn_manager.push(SpawnManager {
                    spawn: spawns.get(spawn.clone()).unwrap(),
                    name: spawn,
                })
            });
        });
        Ok(())
    }
}

thread_local! {
  pub static SPAWN_MANAGERS: RefCell<Vec<SpawnManager>> = RefCell::new(Vec::new());
}
