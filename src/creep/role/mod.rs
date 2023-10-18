use screeps::Creep;
use serde::{Deserialize, Serialize};

use self::miner::MinerManager;

pub mod miner;

#[derive(Debug)]
pub enum RoleManager {
    //HAULER,
    MINER(MinerManager),
}

impl RoleManager {
    pub fn run(&mut self, creep: Creep) {
        match self {
            RoleManager::MINER(x) => x.run(creep),
        }
    }
}

//TODO: having a emun and a trait like that is a bad idea
pub trait RoleManagerTrait {
    fn run(&self) {}
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Hash,PartialEq,Eq)]
pub enum Role {
    HAULER,
    MINER,
}

impl Role {
    pub fn as_string(&self) -> &str {
        match self {
            Role::HAULER => "HAULER",
            Role::MINER => "MINER",
        }
    }
}
