use std::{borrow::BorrowMut, cell::Cell};

use log::{debug, warn};
use screeps::{
    find, Creep, ErrorCode, HasTypedId, Part, ResourceType, SharedCreepProperties, Source,
};

use crate::{
    mem::creep::ParserMemeory,
    room::ROOM_MANAGERS,
    spawn::recepie::Recepie,
    util::error::{MyError, Result}, creep::CreepManager,
};

use super::{Role, RoleManager};

#[derive(Debug, Clone)]
pub struct StarterManager {}

pub fn recepie() -> Recepie {
    Recepie {
        parts: vec![Part::Work, Part::Carry, Part::Move],
        role: Role::STARTER,
    }
}

impl StarterManager {}

impl RoleManager for StarterManager {
    fn run(&mut self, creep_manager: &mut CreepManager) -> Result<()> {
        let room = creep_manager.room()?;
        let creep = creep_manager.creep()?;

        let source = room.find(find::SOURCES, None).first().unwrap().clone();

        if creep.store().get_free_capacity(None) > 0 {
            match creep.harvest(&source) {
                Err(ErrorCode::NotInRange) => creep.move_to(&source),
                x => x,
            }
        } else {
            let spawn = room
                .find(screeps::constants::find::MY_SPAWNS, None)
                .first()
                .unwrap()
                .clone();

            match creep.transfer(&spawn, ResourceType::Energy, None) {
                Err(ErrorCode::NotInRange) => creep.move_to(spawn),
                x => x,
            }
        }
        .map_err(MyError::from)?;
        Ok(())
    }
}
