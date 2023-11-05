use std::cell::Cell;

use log::{debug, warn};
use screeps::{find, look::ENERGY, Creep, ErrorCode, Part, ResourceType, SharedCreepProperties};

use crate::{
    mem::creep::ParserMemeory, spawn::recepie::Recepie, util::error::Result,
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

        let mut mem = creep.get_parsed_memory().unwrap();
        let energy = creep.store().get_used_capacity(Some(ResourceType::Energy));
        if energy == 0 {
            mem.role_mem = Some("harvesting".to_owned());
        } else if energy == creep.store().get_capacity(Some(ResourceType::Energy)) {
            mem.role_mem = Some("upgrading".to_owned());
        }
        creep.set_parsed_memory(mem.clone()).unwrap();
        if mem.role_mem.is_some() {
            let task = mem.role_mem.unwrap();
            match task.as_str() {
                "harvesting" => {
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
                }
                "upgrading" => {
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
                _ => (),
            };
        }
        Ok(())
    }
}
