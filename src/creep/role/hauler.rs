use std::cell::Cell;

use log::{debug, warn};
use screeps::{find, look::ENERGY, Creep, ErrorCode, Part, ResourceType, SharedCreepProperties};

use crate::{
    creep::CreepManager, mem::creep::ParserMemeory, spawn::recepie::Recepie, util::Result,
};

use super::{Role, RoleManager};

#[derive(Debug, Clone)]
pub struct HaulerManager {}

pub fn recepie() -> Recepie {
    Recepie {
        parts: vec![
            Part::Move,
            Part::Move,
            Part::Carry,
            Part::Carry,
            Part::Carry,
            Part::Carry,
        ],
        role: Role::HAULER,
    }
}
impl RoleManager for HaulerManager {
    fn run(&mut self, creep: Creep) -> Result<()> {
        let room = creep.room().unwrap();
        let source = match room
            .find(screeps::constants::find::DROPPED_RESOURCES, None)
            .first()
        {
            Some(x) => x.clone(),
            None => return Ok(()),
        };

        if creep.store().get_free_capacity(None) > 0 {
            match creep.pickup(&source) {
                Ok(_) => (),
                Err(ErrorCode::NotInRange) => match creep.move_to(source) {
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
        }
        Ok(())
    }
}
