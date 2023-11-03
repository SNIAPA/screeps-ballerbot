use std::collections::HashMap;

use log::debug;
use screeps::{
    find::{MY_STRUCTURES, SOURCES},
    HasPosition, Room,
};

use crate::creep::role::Role;

pub fn spawn_order(room: Room) -> Vec<Role> {
    let mut order = room.find(SOURCES, None).iter().fold(vec![], |mut acc, x| {
        let spawn = x
            .room()
            .unwrap()
            .find(MY_STRUCTURES, None)
            .first()
            .unwrap()
            .clone();
        let dist = x.pos().get_range_to(spawn.pos());

        let miner_rate = 4;
        // no fatigue walk
        let carry_parts = 3;
        let required_haulers = ((dist * 2 * miner_rate) + 10) / (carry_parts * 50);

        acc.push(Role::MINER);
        for _ in 0 .. required_haulers {

        acc.push(Role::HAULER);

        }
        acc.push(Role::MINER);
        acc.push(Role::MINER);


        acc
    });
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order.push(Role::UPGRADER);
    order
}
