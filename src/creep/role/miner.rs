use std::{borrow::BorrowMut, cell::Cell};

use log::{debug, warn};
use screeps::{
    find, Creep, ErrorCode, HasTypedId, Part, ResourceType, SharedCreepProperties, Source,
};

use crate::{
    creep::CreepManager,
    mem::creep::ParserMemeory,
    room::ROOM_MANAGERS,
    spawn::recepie::Recepie,
    util::error::{MyError, Result, ToMyErr},
};

use super::{Role, RoleManager};

#[derive(Debug, Clone)]
pub struct MinerManager {
    source: Option<Source>,
}

pub fn recepie() -> Recepie {
    Recepie {
        parts: vec![Part::Work, Part::Work, Part::Move],
        role: Role::MINER,
    }
}

impl MinerManager {
    pub fn new(creep: Creep) -> Self {
        let room = creep.get_parsed_memory().unwrap().room;
        ROOM_MANAGERS.with(|room_manager| {
            let mut room_managers = room_manager.borrow_mut();

            let room = room_managers.get_mut(&room).unwrap();
            let source = room.assign_miner().unwrap();

            if let Some(source) = source.clone() {
                let mut mem = creep.get_parsed_memory().unwrap();
                mem.role_mem = Some(source.id().to_string());
                creep.set_parsed_memory(mem).unwrap();
            }
            MinerManager { source }
        })
    }
}

impl RoleManager for MinerManager {
    fn run(&mut self, creep_manager: &mut CreepManager) -> Result<()> {
        let room = creep_manager.room()?;
        let creep = creep_manager.creep()?;

        let source = self.source.clone().to_my_err("source not set")?;

        match creep.harvest(&source) {
            Err(ErrorCode::NotInRange) => creep.move_to(&source),
            x => x,
        }
        .map_err(MyError::from)?;
        Ok(())
    }
}
