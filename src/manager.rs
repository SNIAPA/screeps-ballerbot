use crate::util::Result;

pub trait Manager {
    fn setup() -> Result<()>;
    fn run_all() -> Result<()>;
    fn run(&mut self) -> Result<()>;
}
