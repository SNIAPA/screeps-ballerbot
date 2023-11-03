use std::cell::Cell;

use log::{debug, warn};
use screeps::{find, look::ENERGY, Creep, ErrorCode, Part, ResourceType, SharedCreepProperties};

use crate::{
    creep::CreepManager, mem::creep::GetParsedCreepMemory, spawn::recepie::Recepie, util::Result,
};

use super::{Role, RoleManager};

#[derive(Debug, Clone)]
pub struct UpgraderManager {}

pub fn recepie() -> Recepie {
    Recepie {
        parts: vec![Part::Move, Part::Carry, Part::Work],
        role: Role::UPGRADER,
    }
}
impl RoleManager for UpgraderManager {
    fn run(&mut self, creep: Creep) -> Result<()> {
        let room = creep.room().unwrap();
        let source = match room
            .find(screeps::constants::find::DROPPED_RESOURCES, None)
            .first()
        {
            Some(x) => x.clone(),
            None => return Ok(()),
        };

        if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
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
            let controller = room.controller().unwrap();

            match creep.upgrade_controller(&controller) {
                Ok(_) => {
                    creep.say("🏗️", false).unwrap();
                }
                Err(ErrorCode::NotInRange) => match creep.move_to(controller) {
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
