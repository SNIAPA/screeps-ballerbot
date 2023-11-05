use std::{cell::Cell, fmt::Debug};

use screeps::Creep;
use serde::{Deserialize, Serialize};

use crate::{spawn::recepie::Recepie, util::Result, mem::creep::ParserMemeory};

use self::{hauler::HaulerManager, miner::MinerManager, upgrader::UpgraderManager, starter::StarterManager};

pub mod hauler;
pub mod miner;
pub mod upgrader;
pub mod starter;


#[derive(Debug, Deserialize, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Role {
    HAULER,
    MINER,
    UPGRADER,
    STARTER,
}

impl Role {
    pub fn all() -> Vec<Role>{
        vec![Role::HAULER, Role::MINER, Role::UPGRADER, Role::STARTER]
    }
    pub fn as_string(&self) -> &str {
        match self.get_recepie().role {
            Role::HAULER => "HAULER",
            Role::MINER => "MINER",
            Role::UPGRADER => "UPGRADER",
            Role::STARTER => "STARTER",
        }
    }
    pub fn get_recepie(&self) -> Recepie {
        match self {
            Role::HAULER => hauler::recepie(),
            Role::MINER => miner::recepie(),
            Role::UPGRADER => upgrader::recepie(),
            Role::STARTER => starter::recepie(),
        }
    }
}


pub fn new_role_manager(creep: Creep) -> Box<dyn RoleManager> {
    let role = creep.get_parsed_memory().unwrap().role;
    match role {
        Role::HAULER => Box::new(HaulerManager {}),
        Role::MINER => Box::new(MinerManager::new(creep).unwrap()),
        Role::UPGRADER => Box::new(UpgraderManager {}),
        Role::STARTER => Box::new(StarterManager {}),
    } 
    
}
pub trait RoleManager {
    fn run(&mut self, creep: Creep) -> Result<()>;
}
