use std::{cell::Cell, fmt::Debug};

use screeps::Creep;
use serde::{Deserialize, Serialize};

use crate::{spawn::recepie::Recepie, util::error::Result, mem::creep::ParserMemeory};

use self::{hauler::HaulerManager, miner::MinerManager, upgrader::UpgraderManager, starter::StarterManager};

use super::CreeepManager;

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



pub trait RoleManager {
    fn run(&mut self, creep_manager: &mut CreeepManager) -> Result<()>;
}

