#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut input : String = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();

    let _ = io::stdin().read_line(&mut input);

    print!("{}: Command not found", input);

    io::stdout().flush().unwrap();

}
