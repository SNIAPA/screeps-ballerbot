use std::{cell::RefCell, collections::HashMap};

use log::info;
use screeps::game;

use crate::{creep::role::new_role_manager, util::ToRustHashMap};

use self::role::RoleManager;

pub mod role;

pub fn run_all() {
    create_managers();

    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creep_managers = creep_managers_refcell.borrow_mut();

        let mut creeps = game::creeps().trhm();

        //run existing creep managers
        creep_managers.iter_mut().for_each(|(name, creep_manager)| {
            let creep = creeps.get(name).unwrap();
            creep_manager.run(creep.clone()).unwrap();
            creeps.remove(name);
        });
    });
}

fn create_managers() {
    CREEP_MANAGERS.with(|creep_managers_refcell| {
        let mut creep_managers = creep_managers_refcell.borrow_mut();

        let creeps = game::creeps().trhm();

        //create managers for creeps that dont have one
        creeps.iter().for_each(|(name, creep)| {
            if creep_managers.contains_key(name) {
                return;
            }

            info!("adding manager: {:?}", name);

            let creep_manager = new_role_manager(creep.clone());
            creep_managers.insert(name.to_string(), creep_manager);
        });
    });
}

pub fn setup() {
    create_managers()
}

thread_local! {
  pub static CREEP_MANAGERS: RefCell<HashMap<String,Box<dyn RoleManager>>> = RefCell::new(HashMap::new());
}
