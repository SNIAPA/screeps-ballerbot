use js_sys::JsString;
use log::*;
use screeps::{game, console};
use wasm_bindgen::prelude::*;

use crate::{
    creep::CreepManager, manager::Manager, mem::creep::clean_creeps, room::RoomManager,
    spawn::SpawnManager,
};

mod creep;
mod logging;
mod manager;
mod mem;
mod room;
mod spawn;
mod util;

#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Trace);
    info!("setup");
    //SpawnManager::setup().unwrap();
    CreepManager::setup().unwrap();
    RoomManager::setup().unwrap();
}

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    clean_creeps().unwrap();
    //SpawnManager::run_all().unwrap();
    CreepManager::run_all().unwrap();
    RoomManager::run_all().unwrap();
    let heap_stats = game::cpu::get_heap_statistics();
    web_sys::console::log_1(&JsString::from(format!(
        "<font size=\"+3\">{} cpu:{}% mem:{}%</font>",
        game::time(),
        (game::cpu::get_used()/game::cpu::tick_limit() * 100.0).round(),
        (heap_stats.used_heap_size()/heap_stats.heap_size_limit() * 100) ,
    )));
}
