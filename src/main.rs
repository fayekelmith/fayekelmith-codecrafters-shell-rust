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
                    }
                    else{
                        if execution::is_executable_cmd(remainder.trim()).0{
                            println!("{} is {}", remainder.trim(), execution::is_executable_cmd(remainder.trim()).1);
                        }else{
                            println!("{}: not found", remainder.trim());
                        }
                    }
                }
                "cd" => if let Err(e) = execution::change_directory(remainder.trim()) {
                    println!("cd: {}", e);
                },
                _ => {
                    if execution::is_executable_cmd(first_word).0{
                        let _ = execution::execute_cmd(first_word, remainder.split_whitespace().collect());
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
                "pwd" => {
                    let path = std::env::current_dir().unwrap();
                    println!("{}", path.display());
                }
                "cd" => if let Err(e) = execution::change_directory("") {
                    println!("cd: {}", e);
                },
                _ => {
                     println!("{}: command not found", input.trim());
                }
            }
        }

    }

}
