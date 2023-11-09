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
pub struct HaulerManager {}

pub fn recepie() -> Recepie {
    Recepie {
        parts: vec![
            Part::Move,
            Part::Move,
            Part::Move,
            Part::Carry,
            Part::Carry,
            Part::Carry,
        ],
        role: Role::HAULER,
    }
}
impl RoleManager for HaulerManager {
    fn run(&mut self, creep_manager: &mut CreepManager) -> Result<()> {
        let creep = creep_manager.creep()?;
        let room = creep_manager.room()?;
        let source = match room
            .find(screeps::constants::find::DROPPED_RESOURCES, None)
            .first()
        {
            Some(x) => x.clone(),
            None => return Ok(()),
        };

        if creep.store().get_free_capacity(None) > 0 {
            match creep.pickup(&source) {
                Err(ErrorCode::NotInRange) => creep.move_to(source),
                x => x,
            }
            .map_err(MyError::from)?;
        } else {
            let spawn = room
                .find(screeps::constants::find::MY_SPAWNS, None)
                .first()
                .to_my_err("cant find spawn")?
                .clone();

            match creep.transfer(&spawn, ResourceType::Energy, None) {
                Err(ErrorCode::NotInRange) => creep.move_to(spawn),
                x => x,
            }
            .map_err(MyError::from)?;
        }
        Ok(())
    }
}
