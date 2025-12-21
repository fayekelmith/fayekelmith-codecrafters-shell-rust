#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;


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
                    }
                    else{
                        if execution::is_executable_cmd(remainder.trim()).0{
                            println!("{} is {}", remainder.trim(), execution::is_executable_cmd(remainder.trim()).1);
                        }else{
                            println!("{}: not found", remainder.trim());
                        }
                    }
                }
                _ => {
                    if execution::is_executable_cmd(first_word).0{
                        let output = Command::new(first_word)
                            .args(remainder.split_whitespace())
                            .output()
                            .expect("Failed to execute command");
                        io::stdout().write_all(&output.stdout).unwrap();
                    }else{
                        println!("{}: command not found", input.trim());
                    }
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
