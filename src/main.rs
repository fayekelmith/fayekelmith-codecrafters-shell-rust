#[allow(unused_imports)]
use std::io::{self, Write};


pub mod commands;
pub mod execution;


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
                "type" => {
                    if let Some(_) = commands::BuiltInCommands::from_str(remainder.trim()){
                        println!("{} is a shell builtin", remainder.trim());
                    }else{
                        if execution::find_executable_files(remainder.trim()){
                            //works
                        }else{
                            println!("{}: not found", remainder.trim());
                        }
                    }
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
