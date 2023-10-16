use std::{cell::RefCell, collections::HashMap};

use log::{debug, info};
use screeps::{game, Creep};

use crate::{
    mem::creep::{clean_creeps, GetParsedCreepMemory},
    util::{error::MyError, Result},
};

use self::role::{miner::MinerManager, RoleManager};

pub mod role;

#[derive(Debug)]
pub struct CreepManager {
    creep: Creep,
    pub name: String,
    role_manager: Option<RoleManager>,
}

impl CreepManager {
    pub fn new(creep: Creep, name: String) -> Self {
        let mem = creep.get_parsed_memory().unwrap();

        let mut creep_manager = Self {
            creep: creep.clone(),
            name,
            role_manager: None,
        };

        let role_manager = match mem.role {
            role::Role::MINER => Some(MinerManager { creep: creep.clone() }),
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
            for creep_manager in creep_managers.iter_mut() {
                let creep = game::creeps()
                    .get(creep_manager.name.clone())
                    .ok_or(MyError {
                        message: "Creep not found".to_string(),
                    })
                    .unwrap();
                creep_manager.creep = creep;
                creep_manager.run().unwrap();
                creeps.remove(&creep_manager.name);
            }
            for name in creeps.keys() {
                let creep = creeps.get(name).unwrap();
                info!("adding manager: {:?}", name);
                let mut creep_manager = CreepManager::new(creep.clone(), name.to_string());
                creep_manager.run().unwrap();
                creep_managers.push(creep_manager);
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
                creep_managers.push(CreepManager::new(creep, name))
            });
        });
        Ok(())
    }
    fn run(&mut self) -> Result<()> {
        self.role_manager.as_mut().unwrap().run(self.creep.clone());
        Ok(())
    }
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<Vec<CreepManager>> = RefCell::new(Vec::new());
}
