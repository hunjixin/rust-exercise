use std::process::{Command, Stdio};

fn main() {
    {
        let result = Command::new("ls")
        .arg("/").output().unwrap();
        println!("{:?}", result);
    }
    {
        let mut cmd  = Command::new("ls")
        .arg("/")
      //  .stdin(Stdio::piped())
      //  .stdout(Stdio::piped())
      .spawn().unwrap();
        let status = cmd.wait().unwrap();
        println!("{:?}", status);
    }
}
