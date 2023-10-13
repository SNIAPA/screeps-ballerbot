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
        info!("running {:?}", self.spawn)
    }
}

thread_local! {
  pub static SPAWN_MANAGERS: RefCell<Vec<SpawnManager>> = RefCell::new(Vec::new());
}

pub fn run_spawns() -> Result<()> {
    SPAWN_MANAGERS.with(|creep_managers_refcell| {
        for spawn_manager in creep_managers_refcell.borrow_mut().iter() {
            let spawn = game::spawns()
                .get(spawn_manager.name.clone())
                .ok_or(MyError)
                .unwrap();
            spawn_manager.run(spawn);
        }
    });

    Ok(())
}
