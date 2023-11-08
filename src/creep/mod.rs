use std::{cell::RefCell, collections::HashMap, sync::{Arc, Mutex}};

use log::info;
use screeps::{game, Creep, SharedCreepProperties};

use crate::{
    mem::creep::ParserMemeory,
    util::{error::*, ToRustHashMap},
};

use self::role::{
    hauler::HaulerManager, miner::MinerManager, starter::StarterManager, upgrader::UpgraderManager,
    Role, RoleManager,
};

pub mod role;

pub struct CreeepManager {
    role_manager: Arc<Mutex<Box<dyn RoleManager>>>,
    name: String,
}
impl CreeepManager {
    pub fn new(creep: Creep) -> CreeepManager {
        let name = creep.name();

        let role = creep.get_parsed_memory().unwrap().role;
        let role_manager: Box<dyn RoleManager> = match role {
            Role::HAULER => Box::new(HaulerManager {}),
            Role::MINER => Box::new(MinerManager::new(creep).unwrap()),
            Role::UPGRADER => Box::new(UpgraderManager {}),
            Role::STARTER => Box::new(StarterManager {}),
        };
        CreeepManager { name, role_manager: Arc::new(Mutex::new(role_manager)) }
    }
    pub fn run(&mut self) -> Result<()> {
        let role_manager = self.role_manager.clone();
        role_manager.lock().unwrap().run(self)
    }
    pub fn get_creep(&self) {
    }
    pub fn setup() {
        CreeepManager::create_managers()
    }
    pub fn run_all() {
        CreeepManager::create_managers();

        CREEP_MANAGERS.with(|creep_managers_refcell| {
            let mut creep_managers = creep_managers_refcell.borrow_mut();

            let mut creeps = game::creeps().trhm();

            //run existing creep managers
            creep_managers.iter_mut().for_each(|(name, creep_manager)| {
                creep_manager.run().unwrap();
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

                let creep_manager = CreeepManager::new(creep.clone());
                creep_managers.insert(name.to_string(), creep_manager);
            });
        });
    }
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<HashMap<String,CreeepManager>> = RefCell::new(HashMap::new());
}
