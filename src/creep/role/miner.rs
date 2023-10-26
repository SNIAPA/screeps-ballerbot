use std::cell::Cell;

use log::{debug, warn};
use screeps::{find, Creep, ErrorCode, Part, ResourceType, SharedCreepProperties};

use crate::{creep::CreepManager, mem::creep::GetParsedCreepMemory, spawn::recepie::Recepie, util::Result};

use super::{Role, RoleManager};

#[derive(Debug, Clone)]
pub struct MinerManager {}

pub fn recepie() -> Recepie {
    Recepie {
        parts: vec![Part::Work, Part::Move, Part::Move],
        role: Role::MINER,
    }
}

impl RoleManager for MinerManager {
    fn run(&mut self, creep: Creep) -> Result<()>{
        let room = creep.room().unwrap();

        let source = room.find(find::SOURCES, None).first().unwrap().clone();

        match creep.harvest(&source) {
            Ok(_) => {
                creep.say("⛏️", false).unwrap();
                Ok(())
            }
            Err(ErrorCode::NotInRange) => {
                match creep.move_to(source) {
                    Err(ErrorCode::NoPath) => {
                        creep.say("❌", false).unwrap();
                        Ok(())
                    }
                    Err(ErrorCode::Tired) => {
                        creep.say("🚬", false).unwrap();
                        Ok(())
                    }
                Err(x) => {
                    warn!("{:#?}", x);
                    Ok(())
                }
                    Ok(_) => Ok(()),
                }
            }
            x => {
                warn!("{:?}", x);
                Ok(())
            }
        }
    }
}
