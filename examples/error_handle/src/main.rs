use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use std::{fmt, thread};

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

fn main() {
    std::panic::set_hook(Box::new(|_| {
        println!("panic thread{:?}", thread::current().id());
    }));

    let closure = || {
        println!("parent thread {:?}", thread::current().id());
        std::thread::spawn(|| {
            println!("new thread {:?}", thread::current().id());
            do_something().unwrap()
        });
    };

    if let Err(e) = std::panic::catch_unwind(closure) {
        println!("{:?}", e);
    }

    sleep(Duration::from_secs(5))
}
