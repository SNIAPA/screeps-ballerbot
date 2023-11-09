use std::collections::HashMap;

use js_sys::Math::ceil;
use log::debug;
use screeps::{
    find::{MY_STRUCTURES, SOURCES},
    HasPosition, Room,
};

use crate::creep::role::Role;

use super::RoomManager;

pub fn spawn_order(room_manager: &RoomManager) -> Vec<Role> {
    if room_manager.creeps().len() == 0 {
        return vec![Role::STARTER];
    }

    let order = room_manager
        .room()
        .find(SOURCES, None)
        .iter()
        .fold(vec![], |mut acc, source| {
            // resource regenerated every 300 ticks and one work part harvests 2 per tick
            let required_work_parts = (source.energy_capacity() / (300 * 2)) as f64;
            // our miners have 2 work parts
            let mut required_miners = ceil(required_work_parts / 2f64) as usize;

            let spawn = source
                .room()
                .unwrap()
                .find(MY_STRUCTURES, None)
                .first()
                .unwrap()
                .clone();
            let dist = source.pos().get_range_to(spawn.pos());

            let miner_rate = 4;
            // no fatigue walk
            let carry_parts = 3;
            let required_haulers = ((dist * 2 * miner_rate) + 10) / (carry_parts * 50);

            acc.push(Role::MINER);
            required_miners -= 1;
            for _ in 0..required_haulers {
                acc.push(Role::HAULER);
            }
            for _ in 0..required_miners {
                acc.push(Role::MINER);
            }

            acc
        });
    order
}
