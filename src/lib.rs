use log::*;
use wasm_bindgen::prelude::*;

use crate::spawn::SpawnManager;
use crate::creep::CreepManager;
use crate::util::*;

mod creep;
mod logging;
mod mem;
mod spawn;
mod util;

#[wasm_bindgen]
pub fn setup() {
    match || -> Result<()> {
        logging::setup_logging(logging::Info);
        SpawnManager::setup()?;
        CreepManager::setup()?;
        Ok(())
    }() {
        Ok(_) => (),
        Err(e) => error!("{}", e),
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
        Err(e) => error!("{}", e),
    }
}

