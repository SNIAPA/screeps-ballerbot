use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{AddAssign, SubAssign},
    str::FromStr,
    sync::{Arc, Mutex},
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
    spawn_managers: Arc<Mutex<HashMap<String, SpawnManager>>>,
}

impl RoomManager {
    fn run(&mut self) {
        self.run_spawns();
    }
    pub fn run_all() {
        RoomManager::create_managers();
        ROOM_MANAGERS.with(|room_managers| {
            let mut room_managers = room_managers.borrow_mut();

            room_managers.iter_mut().for_each(|(_, room_manager)| {
                room_manager.run();
            });
        })
    }

    pub fn setup() {
        RoomManager::create_managers();
    }

    fn create_managers() {
        ROOM_MANAGERS.with(|room_managers| {
            let mut room_managers = room_managers.borrow_mut();
            let rooms = game::rooms().trhm();

            rooms.keys().for_each(|&name| {
                if room_managers.contains_key(&name) {
                    return;
                }
                let room_manager = RoomManager::new(name);
                room_managers.insert(name, room_manager);
            });
        })
    }
    pub fn creeps(&self) -> Vec<Creep> {
        game::creeps()
            .trhm()
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
        let spawn_managers = self.spawn_managers.clone();
        let mut spawn_managers = spawn_managers.lock().unwrap();
        for spawn_manager in spawn_managers.values_mut() {
            unwrap_or_print_error!(spawn_manager.run(self));
        }
    }
    pub fn get_next_creep_to_spawn(&self) -> Option<Recepie> {
        let mut created_roles = self.creeps().iter().fold(
            HashMap::from(
                Role::all()
                    .iter()
                    .map(|&x| (x, 0))
                    .collect::<HashMap<Role, u8>>(),
            ),
            |mut acc, creep| {
                let role = creep.get_parsed_memory().unwrap().role;
                let val = acc.get_mut(&role).unwrap();
                val.add_assign(1);
                acc
            },
        );

        let spawn_order = spawn_order(self);
        let mut order = spawn_order.iter().peekable();
        spawn_order
            .iter()
            .fold(None, |res, x| {
                if res.is_some() {
                    return res;
                }
                let curr = order.peek().unwrap();
                let count = created_roles.get_mut(curr).unwrap();

                if count > &mut 0 {
                    order.next();
                    count.sub_assign(1);
                    return res;
                }
                Some(x)
            })
            .map(|x| x.get_recepie())
    }
    fn new(name: RoomName) -> Self {
        let room_manager = RoomManager {
            name,
            spawn_managers: Arc::new(Mutex::new(HashMap::new())),
        };
        let spawn_managers = room_manager.spawn_managers.clone();
        let mut spawn_managers = spawn_managers.lock().unwrap();
        room_manager
            .room()
            .find(screeps::find::MY_SPAWNS, None)
            .iter()
            .for_each(|spawn| {
                let name = spawn.name().as_string().unwrap();
                spawn_managers.insert(name.clone(), SpawnManager { name });
            });

        room_manager
    }
    fn room(&self) -> Room {
        game::rooms().get(self.name).unwrap()
    }
    fn miner_per_source(&self) -> HashMap<ObjectId<Source>, u8> {
        let sources = self.room().find(find::SOURCES, None);
        game::creeps().trhm().values().fold(
            sources
                .iter()
                .map(|x| (x.id(), 0))
                .collect::<HashMap<_, u8>>(),
            |mut acc, creep| {
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
        )
    }
    pub fn assign_miner(&mut self) -> Result<Option<Source>> {
        let next_source = self
            .miner_per_source()
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
        return Ok(next_source);
    }
}

thread_local! {
  pub static ROOM_MANAGERS: RefCell<HashMap<RoomName,RoomManager>> = RefCell::new(HashMap::new());
}
