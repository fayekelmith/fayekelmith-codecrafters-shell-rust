#[allow(unused_imports)]
use std::io::{self, Write};

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

        //tokenize input
        let parts = execution::process_str(input);

        //parse input
        let command_info = execution::parse_str(parts);

        let commands = command_info.clone();

        let command = &command_info.clean_args[0];
        let args = &command_info.clean_args[1..];

        match command.as_str() {
                "exit" => {
                    break;
                }, 
                "echo" => {
                    let output = args.join(" ") + "\n";
                    execution::handler_output(output.into_bytes(), &mut command_info.stdout_redirect.clone(), true).unwrap();
                    execution::handler_output(vec![], &mut command_info.stderr_redirect.clone(), false).unwrap();
                }
                "pwd" => {
                    let path = std::env::current_dir().unwrap().to_string_lossy().into_owned();
                    let output = path + "\n";
                    execution::handler_output(output.into_bytes(), &mut command_info.stdout_redirect.clone(), true).unwrap();
                    execution::handler_output(vec![], &mut command_info.stderr_redirect.clone(), false).unwrap();
                },
                "type" => {
                    let target = args[0].as_str();
                    
                    if let Some(_) = commands::BuiltInCommands::from_str(target){
                        println!("{} is a shell builtin", target);
                    }
                    else if let (true, path) = execution::is_executable_cmd(target){
                        println!("{} is {}", target, path);
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
                if exists {
                    if let Err(err) = execution::execute_cmd(command, commands){
                        eprintln!("{}", err);
                    }
                } else {
                    println!("{}: command not found", command);
                }
                }
        }
    }

}
