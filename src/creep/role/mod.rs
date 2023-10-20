use std::{fmt::Debug, cell::Cell};

use screeps::Creep;
use serde::{Deserialize, Serialize};

use crate::{spawn::recepie::Recepie, util::Result};

use self::{
    hauler::{HaulerManager},
    miner::{MinerManager},
};

use super::CreepManager;

pub mod hauler;
pub mod miner;

// #[derive(Debug,Clone)]
// pub enum RoleManager {
//     HAULER(HaulerManager),
//     MINER(MinerManager),
// }

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
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
    pub fn get_recepie(&self) -> Recepie {
        match self {
            Role::HAULER => hauler::recepie(),
            Role::MINER => miner::recepie(),
        }
    }
}

pub trait RoleManager {
    fn run(&mut self, creep: Creep ) -> Result<()>;
}

impl Debug for dyn RoleManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"")
    }
}
