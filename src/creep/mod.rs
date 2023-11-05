use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::HashMap,
};

use log::{debug, info, warn};
use screeps::{game, Creep, RoomName};

use crate::{
    creep::role::new_role_manager,
    manager::Manager,
    mem::{creep::ParserMemeory, RootMem},
    util::{Result, ToRustHashMap},
};

use self::role::{
    hauler::HaulerManager, miner::MinerManager, upgrader::UpgraderManager, RoleManager,
};

pub mod go_and_do;
pub mod role;

pub fn run_all() {
    create_managers();

    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creep_managers = creep_managers_refcell.borrow_mut();

        let mut creeps = game::creeps().to_rust_hash_map();

        //run existing creep managers
        for (name, creep_manager) in creep_managers.iter_mut() {
            let creep = creeps.get(name).unwrap();
            creep_manager.run(creep.clone()).unwrap();
            creeps.remove(name);
        }
    });
}

pub fn create_managers() {
    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creep_managers = creep_managers_refcell.borrow_mut();

        let creeps = game::creeps().to_rust_hash_map();

        //create managers for creeps that dont have one
        for (name, creep) in creeps.iter() {
            if creep_managers.contains_key(name) {
                continue;
            }

            info!("adding manager: {:?}", name);

            let creep_manager = new_role_manager(creep.clone());
            creep_managers.insert(name.to_string(), creep_manager);
        }
    });
}

pub fn setup() {
    create_managers()
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<HashMap<String,Box<dyn RoleManager>>> = RefCell::new(HashMap::new());
}
