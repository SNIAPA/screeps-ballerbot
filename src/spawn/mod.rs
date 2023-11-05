use std::{cell::RefCell, collections::HashMap};

use js_sys::JsString;
use log::debug;
use screeps::{game, look::ENERGY, HasPosition, Room, SpawnOptions, StructureSpawn, TextStyle};
use wasm_bindgen::JsStatic;

use crate::{
    creep::role::Role,
    mem::creep::{CreepMem, ParserMemeory},
    room::{RoomManager, ROOM_MANAGERS},
    util::{error::MyError, error::Result, ToRustHashMap},
};

use self::recepie::Recepie;

pub mod recepie;

#[derive(Debug, Clone)]
pub struct SpawnManager {
    pub name: String,
}

impl SpawnManager {
    pub fn run(&mut self, next_creep_to_spawn: Option<Recepie>) -> Result<()> {
        if let Some(recepie) = next_creep_to_spawn {
            self.spawn_creep(recepie).unwrap();
        }
        Ok(())
    }
    fn spawn(&self) -> StructureSpawn {
        game::spawns().get(self.name.clone()).unwrap()
    }
    fn spawn_creep(&self, recepie: Recepie) -> Result<()> {
        let spawn = self.spawn();

        let name = format!(
            "{}@{}#{}",
            recepie.role.as_string(),
            spawn.room().unwrap().name(),
            game::time()
        );

        let mem = CreepMem {
            room: spawn.room().unwrap().name(),
            role: recepie.role,
            role_mem: None,
            _move: None,
        };

        let options = SpawnOptions::new()
            .memory(js_sys::JSON::parse(&serde_json::to_string(&mem).unwrap()).unwrap());

        let options_test = SpawnOptions::new()
            .memory(js_sys::JSON::parse(&serde_json::to_string(&mem).unwrap()).unwrap())
            .dry_run(true);

        let test = self
            .spawn()
            .spawn_creep_with_options(&recepie.parts, &name, &options_test);

        let mut say_text = String::new();
        match test {
            Ok(_) => {
                spawn
                    .spawn_creep_with_options(&recepie.parts, &name, &options)
                    .unwrap();
                say_text = format!("🔄{}", recepie.role.as_string(),)
            }
            Err(e) => {
                match e {
                    screeps::ErrorCode::Busy => {
                        say_text = format!("🔄{}", spawn.spawning().unwrap().name())
                    }
                    screeps::ErrorCode::NotEnough => {
                        let energy = spawn
                            .store()
                            .get_used_capacity(Some(screeps::ResourceType::Energy));
                        say_text = format!(
                            "🔨{} {}% ",
                            recepie.role.as_string(),
                            energy * 100 / recepie.cost()
                        )
                    }
                    _ => (),
                };
            }
        };
        spawn.room().unwrap().visual().text(
            spawn.pos().x().u8() as f32,
            spawn.pos().y().u8() as f32 + 1.4,
            say_text,
            Some(TextStyle::default().color("#90B77D").stroke("2")),
        );
        Ok(())
    }
}
