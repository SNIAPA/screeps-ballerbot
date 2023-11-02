use screeps::Part;

use crate::creep::role::Role;

#[derive(Debug,Clone)]
pub struct Recepie {

    pub parts: Vec<Part>,
    pub role: Role

}

impl Recepie {
    pub fn cost(&self) -> u32 {
        self.parts.iter().fold(0, |acc, x| acc + x.cost())
        
    }
}
