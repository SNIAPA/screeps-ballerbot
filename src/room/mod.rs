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
use screeps::{
    find, game, Creep, HasTypedId, ObjectId, Room, RoomName, RoomObjectProperties, Source,
};
use web_sys::console::debug;

use self::spawn_order::spawn_order;

mod spawn_order;

#[derive(Debug)]
pub struct RoomManager {
    name: RoomName,
    spawn_managers: HashMap<String, SpawnManager>,
    miner_per_source: HashMap<ObjectId<Source>, u8>,
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
                vec![(Role::MINER, 0), (Role::HAULER, 0), (Role::UPGRADER, 0)]
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
                    return res;
                }

                if count > &mut 0 {
                    count.sub_assign(1);
                    order.next();
                    return res;
                }
                Some(x)
            })
            .map(|x| x.get_recepie())
    }
    fn new(name: RoomName) -> Self {
        let mut room_manager = RoomManager {
            name,
            spawn_managers: HashMap::new(),
            miner_per_source: HashMap::new(),
        };
        let sources = room_manager.room().find(find::SOURCES, None);
        room_manager.miner_per_source = sources.iter().map(|x| (x.id(), 0u8)).collect();
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
    pub fn assign_miner(&mut self) -> Result<Option<Source>> {
        let source = self
            .miner_per_source
            .iter()
            .fold(None, |mut res, (source, &miners)| {
                if miners < 3 {
                    if let Some((_, res_miners)) = res {
                        if miners > res_miners {
                            return res;
                        }
                    }
                    res = Some((source, miners))
                }
                res
            })
            .map(|(id, _)| game::get_object_by_id_typed::<Source>(id).unwrap());
        if let Some(source) = source.clone() {
            let count = self.miner_per_source.get_mut(&source.id()).unwrap();
            count.add_assign(1);
        }
        return Ok(source)
    }
}

thread_local! {
  pub static ROOM_MANAGERS: RefCell<HashMap<RoomName,RoomManager>> = RefCell::new(HashMap::new());
}
