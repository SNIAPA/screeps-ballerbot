use std::{cell::RefCell, collections::HashMap};

use log::{debug, info};
use screeps::{game, Creep};

use crate::{mem::creep::GetParsedCreepMemory, util::Result};

use self::role::{miner::MinerManager, RoleManager};

pub mod role;

#[derive(Debug)]
pub struct CreepManager {
    pub name: String,
    role_manager: Option<RoleManager>,
}

impl CreepManager {
    pub fn new(creep: Creep, name: String) -> Self {
        let mem = creep.get_parsed_memory().unwrap();

        let mut creep_manager = Self {
            name,
            role_manager: None,
        };

        let role_manager = match mem.role {
            role::Role::MINER => Some(MinerManager {
                creep: creep.clone(),
            }),
            role::Role::HAULER => None,
        };
        creep_manager.role_manager = Some(RoleManager::MINER(role_manager.unwrap()));
        creep_manager
    }

    pub fn run_all() -> Result<()> {
        CREEP_MANAGERS.with(|creep_managers_refcell| {
            let mut creeps = game::creeps()
                .keys()
                .zip(game::creeps().values())
                .collect::<HashMap<String, Creep>>();

            let mut creep_managers = creep_managers_refcell.borrow_mut();
            for (name, creep_manager) in creep_managers.iter_mut() {
                let creep = game::creeps().get(name.clone()).unwrap();
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
    pub fn setup() -> Result<()> {
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
    fn creep(&self) -> Creep {
        game::creeps().get(self.name.clone()).unwrap()
    }
    fn run(&mut self) -> Result<()> {
        let creep = self.creep();
        self.role_manager.as_mut().unwrap().run(creep);
        Ok(())
    }
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<HashMap<String,CreepManager>> = RefCell::new(HashMap::new());
}
