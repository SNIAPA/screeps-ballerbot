use std::collections::HashMap;

use screeps::{Room, find::SOURCES};

use crate::creep::role::Role;

pub fn spawn_order(room: Room) -> Vec<Role> {

    room.find(SOURCES, None).iter().fold(vec![], |mut acc, x|{
        acc.push(Role::MINER);
        acc.push(Role::HAULER);
        acc.push(Role::MINER);
        acc.push(Role::MINER);
        acc

    })
}
