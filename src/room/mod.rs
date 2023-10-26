use std::{cell::RefCell, collections::HashMap};

use crate::{
    creep::role::Role,
    manager::Manager,
    mem::creep::{get_mem, GetParsedCreepMemory},
    spawn::{recepie::Recepie, SpawnManager},
    util::{Result, ToRustHashMap},
};

use log::debug;
use screeps::{game, Creep, Room, RoomName};

#[derive(Debug)]
pub struct RoomManager {
    name: RoomName,
    spawn_managers: HashMap<String, SpawnManager>,
    pub required_creeps: HashMap<Role, u8>,
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
        self.run_spawns()?;
        Ok(())
    }
}

impl RoomManager {
    pub fn creeps(&self) -> Vec<Creep> {
        let mem = get_mem();
        match mem.creeps {
            Some(creeps) => creeps
                .iter()
                .filter_map(|(name, creep)| {
                    if creep.as_ref().unwrap().room == self.name {
                        return game::creeps().get(name.clone());
                    };
                    None
                })
                .collect(),
            None => vec![],
        }
    }
    fn run_spawns(&mut self) -> Result<()> {
        for (_, spawn_manager) in self.spawn_managers.clone().iter_mut() {
            spawn_manager.run(self)?;
        }

        Ok(())
    }
    pub fn get_next_creep_to_spawn(&self) -> Option<Recepie> {
        let created_roles = self.creeps().iter().fold(
            HashMap::from(
                vec![(Role::MINER, 0), (Role::HAULER, 0)]
                    .iter()
                    .copied()
                    .collect::<HashMap<Role, u8>>(),
            ),
            |mut acc, creep| {
                let role = creep.get_parsed_memory().unwrap().role;
                acc.insert(role, acc[&role] + 1);
                acc
            },
        );

        Vec::from_iter(self.required_creeps.iter())
            .iter()
            .fold(None, |acc, (role, required)| {
                let count = created_roles.get(role).unwrap();
                if let Some((_, x)) = acc {
                    if x == 0 {
                        return acc;
                    }
                }
                if count < *required {
                    return Some((role, *count));
                }
                None
            })
            .map(|x| x.0.get_recepie())
    }
    fn new(name: RoomName) -> Self {
        let mut room_manager = RoomManager {
            name,
            spawn_managers: HashMap::new(),
            required_creeps: vec![(Role::MINER, 3), (Role::HAULER, 2)]
                .iter()
                .copied()
                .collect(),
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
