use log::*;
use wasm_bindgen::prelude::*;

use crate::spawn::run_spawns;

mod logging;
mod spawn;
mod util;
mod creep;
mod mem;

#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Info);
    info!("setup")
}

#[wasm_bindgen(js_name = loop)]
pub fn game_loop()  {
    match inner_loop() {
    Ok(_) => (),
    Err(e) => {
        error!("{}",e)
    },
}
}

pub fn inner_loop() -> crate::util::Result<()>  {
    //run_spawns()?;
    

    Ok(())
}

