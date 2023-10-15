use screeps::Part;

use crate::creep::role::Role;

#[derive(Debug)]
pub struct Recepie {

    pub parts: Box<[Part]>,
    pub role: Role

}
