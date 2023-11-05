use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

use crate::{
    creep::role::{self, Role},
    mem::creep::{get_mem, ParserMemeory},
    spawn::{recepie::Recepie, SpawnManager},
    unwrap_or_print_error,
    util::{error::Result, ToRustHashMap},
};

use log::{debug, error};
use screeps::{
    find, game, Creep, HasTypedId, ObjectId, Room, RoomName, RoomObjectProperties, Source,
};

use self::spawn_order::spawn_order;

mod spawn_order;

#[derive(Debug)]
pub struct RoomManager {
    name: RoomName,
    spawn_managers: HashMap<String, SpawnManager>,
}

pub fn run_all() {
    create_managers();
    ROOM_MANAGERS.with(|room_managers| {
        let mut room_managers = room_managers.borrow_mut();

        room_managers.iter_mut().for_each(|(_, room_manager)| {
            room_manager.run();
        });
    })
}

pub fn setup() {
    create_managers();
}

fn create_managers() {
    ROOM_MANAGERS.with(|room_managers| {
        let mut room_managers = room_managers.borrow_mut();
        let rooms = game::rooms().to_rust_hash_map();

        rooms.keys().for_each(|&name| {
            if room_managers.contains_key(&name) {
                return;
            }
            let room_manager = RoomManager::new(name);
            room_managers.insert(name, room_manager);
        });
    })
}

impl RoomManager {
    fn run(&mut self) {
        self.run_spawns();
    }
}

impl RoomManager {
    pub fn creeps(&self) -> Vec<Creep> {
        game::creeps()
            .to_rust_hash_map()
            .values()
            .cloned()
            .filter_map(|creep| {
                if creep.get_parsed_memory().unwrap().room == self.name {
                    return Some(creep);
                }
                None
            })
            .collect::<Vec<_>>()
    }
    fn run_spawns(&mut self) {
        for spawn_manager in self.spawn_managers.values() {
            unwrap_or_print_error!(spawn_manager.run(self.get_next_creep_to_spawn()));
        };
    }
    pub fn get_next_creep_to_spawn(&self) -> Option<Recepie> {
        let mut created_roles = self.creeps().iter().fold(
            HashMap::from(
                Role::all()
                    .iter()
                    .map(|&x| (x, 0u8))
                    .collect::<Vec<_>>()
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

        let spawn_order = spawn_order(self);
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
                    order.next();
                    count.sub_assign(1u8);
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
    pub fn assign_miner(&mut self) -> Result<Option<Source>> {
        let sources = self.room().find(find::SOURCES, None);
        let miner_per_source = game::creeps().to_rust_hash_map().iter().fold(
            sources
                .iter()
                .map(|x| (x.id(), 0u8))
                .collect::<HashMap<ObjectId<Source>, u8>>(),
            |mut acc, (_, creep)| {
                let mem = creep.get_parsed_memory().unwrap();
                if mem.room != self.name || mem.role != Role::MINER {
                    return acc;
                }
                if let Some(source_id) = mem.role_mem {
                    let source_id = ObjectId::<Source>::from_str(&source_id).unwrap();
                    let miner_count = acc.get_mut(&source_id).unwrap();
                    miner_count.add_assign(1)
                }
                acc
            },
        );
        let source = miner_per_source
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
        return Ok(source);
    }
}

thread_local! {
  pub static ROOM_MANAGERS: RefCell<HashMap<RoomName,RoomManager>> = RefCell::new(HashMap::new());
}
