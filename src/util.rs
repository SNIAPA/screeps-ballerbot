use std::{error::Error, fmt};


pub type Result<T> = std::result::Result<T,Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct MyError{
    pub message: &'static str
}

impl fmt::Display for MyError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl Error for MyError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Internal error"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
