use log::*;
use screeps::game;
use wasm_bindgen::prelude::*;

use crate::creep::CreepManager;
use crate::mem::creep::clean_creeps;
use crate::spawn::SpawnManager;
use crate::util::*;

mod creep;
mod logging;
mod mem;
mod room;
mod spawn;
mod util;

#[wasm_bindgen]
pub fn setup() {
    match || -> Result<()> {
        logging::setup_logging(logging::Trace);
        info!("setup");
        SpawnManager::setup()?;
        //CreepManager::setup()?;
        Ok(())
    }() {
        Ok(_) => (),
        Err(e) => warn!("{:?}", e),
    }
}

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    let rooms = game::rooms().to_rust_hash_map();
    debug!("{:?}", rooms);

    match || -> Result<()> {
        clean_creeps()?;
        SpawnManager::run_all()?;
        CreepManager::run_all()?;
        Ok(())
    }() {
        Ok(_) => (),
        Err(e) => warn!("{:?}", e),
    }
    info!("done: {}", game::cpu::get_used());
}
