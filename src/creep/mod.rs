use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::HashMap,
};

use log::{debug, info, warn};
use screeps::{game, Creep, RoomName};

use crate::{
    manager::Manager,
    mem::{creep::ParserMemeory, RootMem},
    util::{Result, ToRustHashMap}, creep::role::new_role_manager,
};

use self::role::{
    hauler::HaulerManager, miner::MinerManager, upgrader::UpgraderManager, RoleManager,
};

pub mod go_and_do;
pub mod role;

pub fn run_all() -> Result<()> {
    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creeps = game::creeps().to_rust_hash_map();

        let mut creep_managers = creep_managers_refcell.borrow_mut();
        for (name, creep_manager) in creep_managers.iter_mut() {
            let creep = creeps.get(name).unwrap();
            creep_manager.run(creep.clone()).unwrap();
            creeps.remove(name);
        }
        for name in creeps.keys() {
            let creep = creeps.get(name).unwrap();
            info!("adding manager: {:?}", name);
            let mut creep_manager = new_role_manager(creep.clone(), name.to_string());
            creep_manager.run(creep.clone()).unwrap();
            creep_managers.insert(name.to_string(), creep_manager);
        }
    });

    Ok(())
}
pub fn setup() -> Result<()> {
    CREEP_MANAGERS.with(|creep_managers| {
        let mut creep_managers = creep_managers.borrow_mut();
        let creeps = game::creeps();

        creeps.keys().for_each(|name| {
            let creep = creeps.get(name.clone()).unwrap();
            creep_managers.insert(name.clone(), new_role_manager(creep, name));
        });
    });
    Ok(())
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<HashMap<String,Box<dyn RoleManager>>> = RefCell::new(HashMap::new());
}
