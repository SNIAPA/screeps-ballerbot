use screeps::Part;

use crate::creep::role::Role;

#[derive(Debug,Clone)]
pub struct Recepie {

    pub parts: Vec<Part>,
    pub role: Role

}
