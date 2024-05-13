use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom Error: {}", self.message)
    }
}

fn do_something() -> Result<(), Box<dyn Error>> {
    // 在这里可能发生错误
    Err(Box::new(CustomError {
        message: String::from("Something went wrong."),
    }))
}

fn main(){
    let closure = ||{
        do_something().unwrap()
    };

    if let Err(e) = std::panic::catch_unwind(closure){
        println!("{:?}", e);
    }

    if let Err(e) = do_something() {
        println!("{e}");
    }
}