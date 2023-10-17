use screeps::{find, Creep, ErrorCode, Part, ResourceType, SharedCreepProperties};

use crate::{mem::creep::GetParsedCreepMemory, spawn::recepie::Recepie};

use super::Role;

#[derive(Debug)]
pub struct MinerManager {
    pub creep: Creep,
}

impl MinerManager {
    pub fn recepie() -> Recepie {
        Recepie {
            parts: [Part::Move, Part::Work, Part::Carry].into(),
            role: Role::MINER,
        }
    }
    pub fn run(&mut self, creep: Creep) {
        self.creep = creep.clone();
        let room = creep.room().unwrap();

        let source = room.find(find::SOURCES, None).first().unwrap().clone();
        let target = room.find(find::MY_SPAWNS, None).first().unwrap().clone();

        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            match creep.harvest(&source) {
                Ok(_) => {
                    creep.say("mining", false).unwrap();
                }
                Err(ErrorCode::NotInRange) => {
                    match creep.move_to(source) {
                        Err(ErrorCode::NoPath) => {
                            creep.say("❌", false).unwrap();
                            Ok(())
                        },
                        Err(ErrorCode::Tired) => {
                            creep.say("🚬", false).unwrap();
                            Ok(())
                        },
                        Err(e) => Err(e),
                        Ok(_) => {

                            creep.say("walking", false).unwrap();
                            Ok(())
                            }
                            
                    }
                    .unwrap();
                }
                _ => (),
            }
        } else {
            match creep.transfer(&target, ResourceType::Energy, None) {
                Ok(_) => {
                    creep.say("transfer", false).unwrap();
                }
                Err(ErrorCode::NotInRange) => {
                    match creep.move_to(target) {
                        Err(ErrorCode::Tired | ErrorCode::NoPath) => Ok(()),
                        Err(e) => Err(e),
                        Ok(_) => Ok(()),
                    }
                    .unwrap();
                    creep.say("walking", false).unwrap();
                }
                _ => (),
            }
        }
    }
}
