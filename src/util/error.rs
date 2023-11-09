use std::{error::Error, fmt};

use screeps::ErrorCode;


#[derive(Debug, Clone)]
pub struct MyError{
    pub message: String
}

impl fmt::Display for MyError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl From<ErrorCode> for MyError {
    fn from(code:ErrorCode) -> Self{
        MyError{message: format!("ErrorCode: {:?}", code) }
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

impl MyError {
    pub fn new(message: &str) -> MyError {
        MyError { message: message.to_string() }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[macro_export]
macro_rules! unwrap_or_print_error {
    ($resutl:expr) => {
        match $resutl {
            Err(e) => error!("{}",e),
            _ => ()
        };
        
    };
}


