use log::*;
use screeps::game;
use wasm_bindgen::prelude::*;

use crate::creep::CreepManager;
use crate::spawn::SpawnManager;
use crate::util::*;

mod creep;
mod logging;
mod mem;
mod spawn;
mod util;

#[wasm_bindgen]
pub fn setup() {
    match || -> Result<()> {
        logging::setup_logging(logging::Trace);
        info!("setup");
        SpawnManager::setup()?;
        CreepManager::setup()?;
        Ok(())
    }() {
        Ok(_) => (),
        Err(e) => warn!("{:?}", e),
    }
}

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    match || -> Result<()> {
        SpawnManager::run_all()?;
        CreepManager::run_all()?;
        Ok(())
    }() {
        Ok(_) => (),
        Err(e) => warn!("{:?}", e),
    }
    info!("done: {}", game::cpu::get_used());
}
