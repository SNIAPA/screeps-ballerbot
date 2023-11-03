use std::collections::HashMap;

use log::debug;
use screeps::{
    find::{MY_STRUCTURES, SOURCES},
    HasPosition, Room,
};

use crate::creep::role::Role;

pub fn spawn_order(room: Room) -> Vec<Role> {
    room.find(SOURCES, None).iter().fold(vec![], |mut acc, x| {
        let spawn = x
            .room()
            .unwrap()
            .find(MY_STRUCTURES, None)
            .first()
            .unwrap()
            .clone();
        let dist = x.pos().get_range_to(spawn.pos());
        acc.push(Role::MINER);
        acc.push(Role::MINER);
        acc.push(Role::MINER);

        let miner_rate = 4;
        // no fatigue walk
        let carry_parts = 3;
        let required_haulers = ((dist * 2 * miner_rate) + 10) / (carry_parts * 50);
        debug!(
            "required_haulers {:?}",
            (required_haulers, dist, miner_rate, carry_parts)
        );

        acc
    })
}
