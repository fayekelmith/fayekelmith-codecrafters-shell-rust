#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        
        let mut input: String = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        if let Some((first_word, remainder)) = input.trim().split_once(char::is_whitespace){
            match first_word {
                "exit" => {
                    break;
                }
                "echo" => {
                    println!("{}", remainder);
                }
                _ => {
                     println!("{}: command not found", input.trim());
                }
            }
        }else{
            match input.trim() {
                "exit" => {
                    break;
                }
                _ => {
                     println!("{}: command not found", input.trim());
                }
            }
        }

    }

}
