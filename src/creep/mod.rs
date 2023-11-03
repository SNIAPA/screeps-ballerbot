use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::HashMap,
};

use log::{debug, info, warn};
use screeps::{game, Creep, RoomName};

use crate::{
    manager::Manager,
    mem::{creep::ParserMemeory, RootMem},
    util::{Result, ToRustHashMap},
};

use self::role::{hauler::HaulerManager, miner::MinerManager, RoleManager, upgrader::UpgraderManager};

pub mod role;
pub mod go_and_do;

#[derive(Debug)]
pub struct CreepManager {
    pub name: String,
    role_manager: Box<dyn RoleManager>,
}

impl Manager for CreepManager {
    fn run_all() -> Result<()> {
        CREEP_MANAGERS.with(|creep_managers_refcell| {
            let mut creeps = game::creeps().to_rust_hash_map();

            let mut creep_managers = creep_managers_refcell.borrow_mut();
            for (name, creep_manager) in creep_managers.iter_mut() {
                creep_manager.run().unwrap();
                creeps.remove(name);
            }
            for name in creeps.keys() {
                let creep = creeps.get(name).unwrap();
                info!("adding manager: {:?}", name);
                let mut creep_manager = CreepManager::new(creep.clone(), name.to_string());
                creep_manager.run().unwrap();
                creep_managers.insert(name.to_string(), creep_manager);
            }
        });

        Ok(())
    }
    fn setup() -> Result<()> {
        CREEP_MANAGERS.with(|creep_managers| {
            let mut creep_managers = creep_managers.borrow_mut();
            let creeps = game::creeps();

            creeps.keys().for_each(|name| {
                let creep = creeps.get(name.clone()).unwrap();
                creep_managers.insert(name.clone(), CreepManager::new(creep, name));
            });
        });
        Ok(())
    }
    fn run(&mut self) -> Result<()> {
        if let Err(e) = self.role_manager.run(self.creep()) {
            warn!("failed to run {}\n{:#?}", self.name, e)
        };
        Ok(())
    }
}

impl CreepManager {
    fn walk_and_do() -> Result<()> {
        Ok(())
    }
    fn creep(&self) -> Creep {
        game::creeps().get(self.name.clone()).unwrap()
    }
    pub fn new(creep: Creep, name: String) -> Self {
        let mem = creep.get_parsed_memory().unwrap();

        let role_manager: Box<dyn RoleManager> = match mem.role {
            role::Role::MINER => Box::new(MinerManager {}),
            role::Role::HAULER => Box::new(HaulerManager {}),
            role::Role::UPGRADER => Box::new(UpgraderManager {}),
        };

        Self { name, role_manager }
    }
    pub fn on_death(&self) {
        let raw_mem = screeps::raw_memory::get().as_string().unwrap();
        let mut mem = serde_json::from_str::<RootMem>(&raw_mem).unwrap();
        mem.creeps.unwrap().remove(&self.name);
    }
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<HashMap<String,CreepManager>> = RefCell::new(HashMap::new());
}

