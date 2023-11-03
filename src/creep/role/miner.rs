use std::{borrow::BorrowMut, cell::Cell};

use log::{debug, warn};
use screeps::{find, Creep, ErrorCode, Part, ResourceType, SharedCreepProperties, Source};

use crate::{
    mem::creep::ParserMemeory, room::ROOM_MANAGERS, spawn::recepie::Recepie, util::Result,
};

use super::{Role, RoleManager};

#[derive(Debug, Clone)]
pub struct MinerManager {
    source: Source,
}

pub fn recepie() -> Recepie {
    Recepie {
        parts: vec![Part::Work, Part::Work, Part::Move],
        role: Role::MINER,
    }
}

impl MinerManager {
    pub fn new(creep: Creep) -> Result<Self> {
        let room = creep.get_parsed_memory().unwrap().room;
        ROOM_MANAGERS.with(|room_manager| {
            let room_managers = room_manager.borrow();
            let room = room_managers.get(&room).unwrap();
            let source = room.assign_miner().unwrap();
            Ok(MinerManager { source })
        })
    }
}

impl RoleManager for MinerManager {
    fn run(&mut self, creep: Creep) -> Result<()> {
        let room = creep.room().unwrap();

        let source = room.find(find::SOURCES, None).first().unwrap().clone();

        match creep.harvest(&source) {
            Ok(_) => {
                creep.say("⛏️", false).unwrap();
                Ok(())
            }
            Err(ErrorCode::NotInRange) => match creep.move_to(source) {
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
            },
            x => {
                warn!("{:?}", x);
                Ok(())
            }
        }
    }
}
