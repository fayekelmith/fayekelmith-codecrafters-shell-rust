#[allow(unused_imports)]
use std::io::{self, Write};

use crate::execution::process_echo_str;

pub mod commands;
pub mod execution;


fn main() {
    loop {
        
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() { continue; }


        let parts = execution::process_echo_str(input);
        let command = &parts[0];
        let args = &parts[1..];

        match command {
                "exit" => {
                    break;
                }
                "echo" => {
                    println!("{}", args.join(" "));
                },
                "pwd" => {
                    let path = std::env::current_dir().unwrap();
                    println!("{}", path.display());
                },
                "type" => {
                    let target = args[0].as_str();
                    
                    if let Some(_) = commands::BuiltInCommands::from_str(target){
                        println!("{} is a shell builtin", target);
                    }
                    else if let (true, path) = execution::is_executable_cmd(target){
                        println!("{} is {}", target, path.display());
                    }
                    else{
                        println!("{}: not found", target);
                    }
                }
                "cd" => {
                    let path = args.get(0).map_or("", |s| s.as_str());
                    if let Err(e) = execution::change_directory(path){ 
                        println!("cd: {}", e);
                    }
                },
                _ => {
                    let (exists, _) = execution::is_executable_cmd(command);
                    if exists{
                        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                        let _ = execution::execute_cmd(command, args_refs);
                    }else{
                        println!("{}: command not found", input.trim());
                    }
                }
        }

    }

}
