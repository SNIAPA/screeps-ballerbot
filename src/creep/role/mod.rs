use std::{cell::Cell, fmt::Debug};

use screeps::Creep;
use serde::{Deserialize, Serialize};

use crate::{spawn::recepie::Recepie, util::Result};

use self::{hauler::HaulerManager, miner::MinerManager};

use super::CreepManager;

pub mod hauler;
pub mod miner;
pub mod upgrader;

// #[derive(Debug,Clone)]
// pub enum RoleManager {
//     HAULER(HaulerManager),
//     MINER(MinerManager),
// }

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Role {
    HAULER,
    MINER,
    UPGRADER,
}

impl Role {
    pub fn as_string(&self) -> &str {
        match self.get_recepie().role {
            Role::HAULER => "HAULER",
            Role::MINER => "MINER",
            Role::UPGRADER => "UPGRADER",
        }
    }
    pub fn get_recepie(&self) -> Recepie {
        match self {
            Role::HAULER => hauler::recepie(),
            Role::MINER => miner::recepie(),
            Role::UPGRADER => upgrader::recepie(),
        }
    }
}

pub trait RoleManager {
    fn run(&mut self, creep: Creep) -> Result<()>;
}

impl Debug for dyn RoleManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
