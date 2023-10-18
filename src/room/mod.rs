use std::{cell::RefCell, collections::HashMap};

use crate::{
    creep::role::Role,
    manager::Manager,
    spawn::SpawnManager,
    util::{Result, ToRustHashMap},
};
use log::debug;
use screeps::{game, Room, RoomName, StructureSpawn};

#[derive(Debug)]
pub struct RoomManager {
    name: RoomName,
    spawn_managers: HashMap<String, SpawnManager>,
    required_creeps: HashMap<Role, u8>,
    //TODO: this will need to be cleaned on creep death
    //maybe creep.deat() function
    creeps: Vec<String>,
}

impl Manager for RoomManager {
    fn run_all() -> Result<()> {
        ROOM_MANAGERS.with(|room_managers| {
            let mut room_managers = room_managers.borrow_mut();
            room_managers.iter_mut().for_each(|(_, room_manager)| {
                room_manager.run().unwrap();
            });
            Ok(())
        })
    }
    fn setup() -> Result<()> {
        ROOM_MANAGERS.with(|room_managers| {
            let mut room_managers = room_managers.borrow_mut();
            let rooms = game::rooms().to_rust_hash_map();
            rooms.keys().for_each(|&name| {
                let room_manager = RoomManager::new(name);
                room_managers.insert(name, room_manager);
            });
            Ok(())
        })
    }

    fn run(&mut self) -> Result<()> {
        debug!("{:#?}", self);
        self.run_spawns()?;
        Ok(())
    }
}

impl RoomManager {
    fn run_spawns(&mut self) -> Result<()> {
        for (_, spawn_manager) in self.spawn_managers.iter_mut() {
            //TODO: match this?
            spawn_manager.run()?;
        }

        Ok(())
    }
    fn new(name: RoomName) -> Self {
        let mut room_manager = RoomManager {
            name,
            spawn_managers: HashMap::new(),
            required_creeps: vec![(Role::MINER, 2),(Role::HAULER, 2)].iter().copied().collect(),
        };
        room_manager.spawn_managers = room_manager
            .room()
            .find(screeps::find::MY_SPAWNS, None)
            .iter()
            .map(|spawn| {
                let name = spawn.name().as_string().unwrap();
                return (name.clone(), SpawnManager { name });
            })
            .collect();

        room_manager
    }
    fn room(&self) -> Room {
        game::rooms().get(self.name).unwrap()
    }
}

thread_local! {
  pub static ROOM_MANAGERS: RefCell<HashMap<RoomName,RoomManager>> = RefCell::new(HashMap::new());
}
