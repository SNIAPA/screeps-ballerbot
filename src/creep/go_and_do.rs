use screeps::{Creep, SharedCreepProperties, HasPosition, Position};
use crate::util::Result;

pub trait GoAndDo {
    fn go_and_do(&self, f: fn(&Self) -> Option<Position>) -> Result<()>;
}
impl GoAndDo for Creep {
    fn go_and_do(&self, f: fn(&Self) -> Option<Position>) -> Result<()>{
        if let Some(pos) = f(self) {
            self.move_to(pos);
        }
        Ok(())
    }
}
