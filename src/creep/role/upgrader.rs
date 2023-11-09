use std::cell::Cell;

use log::{debug, warn};
use screeps::{find, look::ENERGY, Creep, ErrorCode, Part, ResourceType, SharedCreepProperties};

use crate::{
    creep::CreepManager,
    mem::creep::ParserMemeory,
    spawn::recepie::Recepie,
    util::error::{MyError, Result, ToMyErr},
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
    fn run(&mut self, creep_manager: &mut CreepManager) -> Result<()> {
        let room = creep_manager.room()?;
        let creep = creep_manager.creep()?;
        let source = match room
            .find(screeps::constants::find::DROPPED_RESOURCES, None)
            .first()
        {
            Some(x) => x.clone(),
            None => return Ok(()),
        };

        let mut mem = creep.get_parsed_memory()?;
        let energy = creep.store().get_used_capacity(Some(ResourceType::Energy));
        if energy == 0 {
            mem.role_mem = Some("harvesting".to_owned());
        } else if energy == creep.store().get_capacity(Some(ResourceType::Energy)) {
            mem.role_mem = Some("upgrading".to_owned());
        }

        creep.set_parsed_memory(mem.clone())?;
        let task = mem.role_mem.to_my_err("invalid task")?;
        match task.as_str() {
            "harvesting" => match creep.pickup(&source) {
                Err(ErrorCode::NotInRange) => creep.move_to(source),
                x => x,
            },
            "upgrading" => {
                let controller = room
                    .controller()
                    .to_my_err("controller not found")?;

                match creep.upgrade_controller(&controller) {
                    Err(ErrorCode::NotInRange) => creep.move_to(controller),
                    x => x,
                }
            }
            _ => Ok(()),
        }
        .map_err(MyError::from)?;
        Ok(())
    }
}
