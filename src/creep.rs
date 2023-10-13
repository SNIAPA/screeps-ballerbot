use std::cell::RefCell;

use log::info;
use screeps::{game, Creep};
use serde::Deserialize;

use crate::{
    mem::clean_creeps,
    util::{MyError, Result},
};

#[derive(Deserialize, Debug, Clone)]
pub enum Role {
    Miner,
    Hauler,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreepMem {
    role: Role,
}

#[derive(Debug,Clone)]
pub struct CreepManager {
    creep: Creep,
    pub name: String,
}

impl CreepManager {
    fn run(&self, creep: Creep) {

        info!("running {:?}",self.creep)
    }
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<Vec<CreepManager>> = RefCell::new(Vec::new());
}

pub fn run_creeps() -> Result<()> {
    clean_creeps()?;
    CREEP_MANAGERS.with(|creep_managers_refcell| {
        for creep_manager in creep_managers_refcell.borrow_mut().iter() {
            let creep = game::creeps()
                .get(creep_manager.name.clone())
                .ok_or(MyError).unwrap();
            creep_manager.run(creep);
        }
    });

    Ok(())
}
