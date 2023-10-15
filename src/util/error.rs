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
