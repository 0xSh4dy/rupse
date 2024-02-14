use std::error::Error;

pub fn throw_custom_error(error_message:String)->Box<dyn Error>{
    return Box::new(std::io::Error::new(std::io::ErrorKind::Other,error_message));
}