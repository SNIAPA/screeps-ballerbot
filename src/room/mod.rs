use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

use crate::{
    creep::role::Role,
    manager::Manager,
    mem::creep::{get_mem, ParserMemeory},
    spawn::{recepie::Recepie, SpawnManager},
    util::{Result, ToRustHashMap},
};

use log::debug;
use screeps::{game, Creep, Room, RoomName, RoomObjectProperties};

use self::spawn_order::spawn_order;

mod spawn_order;

#[derive(Debug)]
pub struct RoomManager {
    name: RoomName,
    spawn_managers: HashMap<String, SpawnManager>,
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
        mem.creeps
            .unwrap_or(HashMap::new())
            .iter()
            .filter_map(|(name, creep)| {
                if creep.as_ref().unwrap().room == self.name {
                    return game::creeps().get(name.clone());
                };
                None
            })
            .collect()
    }
    fn run_spawns(&mut self) -> Result<()> {
        for (_, spawn_manager) in self.spawn_managers.clone().iter_mut() {
            spawn_manager.run(self)?;
        }

        Ok(())
    }
    pub fn get_next_creep_to_spawn(&self) -> Option<Recepie> {
        let mut created_roles = self.creeps().iter().fold(
            HashMap::from(
                vec![(Role::MINER, 0), (Role::HAULER, 0),(Role::UPGRADER, 0)]
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

        let spawn_order = spawn_order(self.room());
        let mut order = spawn_order.iter().peekable();
        spawn_order
            .iter()
            .fold(None, |res, x| {
                let curr = order.peek().unwrap();
                let count = created_roles.get_mut(curr).unwrap();

                if res.is_some() {
                    return res
                }

                if count > &mut 0 {
                    count.sub_assign(1);
                    order.next();
                    return res
                }
                Some(x)

            })
            .map(|x| x.get_recepie())
    }
    fn new(name: RoomName) -> Self {
        let mut room_manager = RoomManager {
            name,
            spawn_managers: HashMap::new(),
            //TODO: this should be dynamic based on the rcl and if we are getting attacked, and stuff
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
