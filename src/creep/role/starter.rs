use std::{borrow::BorrowMut, cell::Cell};

use log::{debug, warn};
use screeps::{
    find, Creep, ErrorCode, HasTypedId, Part, ResourceType, SharedCreepProperties, Source,
};

use crate::{
    mem::creep::ParserMemeory, room::ROOM_MANAGERS, spawn::recepie::Recepie, util::error::Result,
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
    fn run(&mut self, creep: Creep) -> Result<()> {
        let room = creep.room().unwrap();

        let source = room.find(find::SOURCES, None).first().unwrap().clone();

        if creep.store().get_free_capacity(None) > 0 {
            match creep.harvest(&source) {
                Ok(_) => {
                    creep.say("⛏️", false).unwrap();
                    Ok(())
                }
                Err(ErrorCode::NotInRange) => match creep.move_to(&source) {
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
        } else {
            let spawn = room
                .find(screeps::constants::find::MY_SPAWNS, None)
                .first()
                .unwrap()
                .clone();

            match creep.transfer(&spawn, ResourceType::Energy, None) {
                Ok(_) => (),
                Err(ErrorCode::NotInRange) => match creep.move_to(spawn) {
                    Ok(_) => (),
                    Err(ErrorCode::NoPath) => {
                        creep.say("❌", false).unwrap();
                    }
                    Err(ErrorCode::Tired) => {
                        creep.say("🚬", false).unwrap();
                    }
                    Err(x) => {
                        warn!("{:#?}", x);
                    }
                },
                Err(x) => {
                    warn!("{:#?}", x);
                }
            };
            Ok(())
        }
    }
}
