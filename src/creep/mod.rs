use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use log::{info, warn};
use screeps::{game, Creep, Room, SharedCreepProperties};

use crate::{
    mem::creep::ParserMemeory,
    util::{error::*, ToRustHashMap},
};

use self::role::{
    hauler::HaulerManager, miner::MinerManager, starter::StarterManager, upgrader::UpgraderManager,
    Role, RoleManager,
};

pub mod role;

pub struct CreepManager {
    role_manager: Arc<Mutex<Box<dyn RoleManager>>>,
    name: String,
}
impl CreepManager {
    pub fn new(creep: Creep) -> CreepManager {
        let name = creep.name();

        let role = creep.get_parsed_memory().unwrap().role;
        let role_manager: Box<dyn RoleManager> = match role {
            Role::HAULER => Box::new(HaulerManager {}),
            Role::MINER => Box::new(MinerManager::new(creep)),
            Role::UPGRADER => Box::new(UpgraderManager {}),
            Role::STARTER => Box::new(StarterManager {}),
        };
        CreepManager {
            name,
            role_manager: Arc::new(Mutex::new(role_manager)),
        }
    }
    pub fn run(&mut self) {
        let role_manager = self.role_manager.clone();
        let mut role_manager = role_manager.lock().unwrap();
        if let Err(e) = role_manager.run(self) {
            warn!("{} - {}", self.name, e)
        }
    }
    pub fn creep(&self) -> Result<Creep> {
        game::creeps()
            .values()
            .find(|creep| creep.name() == self.name)
            .to_my_err("cannot get creep")
    }
    pub fn room(&self) -> Result<Room> {
        let room = self.creep()?.get_parsed_memory()?.room;
        Ok(game::rooms().get(room).to_my_err("room not found")?)
    }
    pub fn setup() {
        CreepManager::create_managers()
    }
    pub fn run_all() {
        CreepManager::create_managers();

        CREEP_MANAGERS.with(|creep_managers_refcell| {
            let mut creep_managers = creep_managers_refcell.borrow_mut();

            let mut creeps = game::creeps().trhm();

            //run existing creep managers
            creep_managers.iter_mut().for_each(|(name, creep_manager)| {
                creep_manager.run();
                creeps.remove(name);
            });
        });
    }

    fn create_managers() {
        CREEP_MANAGERS.with(|creep_managers_refcell| {
            let mut creep_managers = creep_managers_refcell.borrow_mut();

            let creeps = game::creeps().trhm();

            //create managers for creeps that dont have one
            creeps.iter().for_each(|(name, creep)| {
                if creep_managers.contains_key(name) {
                    return;
                }

                info!("adding manager: {:?}", name);

                let creep_manager = CreepManager::new(creep.clone());
                creep_managers.insert(name.to_string(), creep_manager);
            });
        });
    }
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<HashMap<String,CreepManager>> = RefCell::new(HashMap::new());
}
